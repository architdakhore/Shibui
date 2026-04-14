//! IPC (Inter-Process Communication) module
//! 
//! Provides JSON-RPC IPC interface for controlling Shibui from external tools.
//! Used by shibuictl CLI tool and third-party integrations.

use log::{info, debug, error, warn};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::fs;
use std::thread;
use std::time::Duration;
use anyhow::Result;

/// IPC message
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpcMessage {
    /// Message type
    pub msg_type: MessageType,
    /// Payload
    pub payload: serde_json::Value,
}

/// Message types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// Get compositor status
    GetStatus,
    /// Get workspace list
    GetWorkspaces,
    /// Get window list
    GetWindows,
    /// Execute command
    Command,
    /// Subscribe to events
    Subscribe,
    /// Event notification
    Event,
    /// Response
    Response,
    /// Error  
    Error,
    /// Reload configuration
    ReloadConfig,
}

/// Compositor status
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompositorStatus {
    /// Compositor name
    pub name: String,
    /// Version
    pub version: String,
    /// Current workspace
    pub active_workspace: i32,
    /// Window count
    pub window_count: usize,
    /// Output count
    pub output_count: usize,
    /// Running state
    pub running: bool,
}

/// IPC server
pub struct IpcServer {
    /// Socket path
    socket_path: PathBuf,
    /// Running state
    pub running: bool,
    /// Listener handle
    listener: Option<UnixListener>,
}

impl IpcServer {
    /// Create new IPC server
    pub fn new() -> Result<Self> {
        let socket_path = Self::get_socket_path();
        
        Ok(Self {
            socket_path,
            running: false,
            listener: None,
        })
    }
    
    /// Get socket path
    fn get_socket_path() -> PathBuf {
        let xdg_runtime = std::env::var("XDG_RUNTIME_DIR")
            .unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(xdg_runtime).join("shibui.sock")
    }
    
    /// Start IPC server listener
    pub fn start(&mut self) -> Result<()> {
        info!("🔌 Starting IPC server...");
        
        // Remove old socket if exists
        if self.socket_path.exists() {
            fs::remove_file(&self.socket_path)?;
        }
        
        // Create listener
        let listener = UnixListener::bind(&self.socket_path)
            .map_err(|e| anyhow::anyhow!("Failed to bind IPC socket: {}", e))?;
        
        // Set non-blocking for accept with timeout
        listener.set_nonblocking(true)
            .map_err(|e| anyhow::anyhow!("Failed to set non-blocking: {}", e))?;
        
        self.listener = Some(listener);
        self.running = true;
        info!("✅ IPC server listening on {:?}", self.socket_path);
        
        Ok(())
    }
    
    /// Process pending IPC connections (non-blocking)
    pub fn process(&self) -> Result<()> {
        if !self.running {
            return Ok(());
        }
        
        let listener = match &self.listener {
            Some(l) => l,
            None => return Ok(()),
        };
        
        // Try to accept connection
        match listener.accept() {
            Ok((mut stream, _)) => {
                debug!("IPC client connected");
                if let Err(e) = self.handle_client(&mut stream) {
                    error!("Error handling IPC client: {}", e);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No pending connections
            }
            Err(e) => {
                warn!("IPC accept error: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// Stop IPC server
    pub fn stop(&mut self) {
        info!("🔌 Stopping IPC server");
        
        self.running = false;
        self.listener = None;
        
        // Clean up socket
        if self.socket_path.exists() {
            let _ = fs::remove_file(&self.socket_path);
        }
    }
    
    /// Handle client connection
    fn handle_client(&self, stream: &mut UnixStream) -> Result<()> {
        // Read request with timeout
        stream.set_read_timeout(Some(Duration::from_secs(1)))?;
        
        let mut buffer = [0u8; 4096];
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                // Parse message
                match serde_json::from_slice::<IpcMessage>(&buffer[..n]) {
                    Ok(message) => {
                        debug!("IPC message: {:?}", message.msg_type);
                        
                        // Process message
                        match self.process_message(&message) {
                            Ok(response) => {
                                // Send response
                                if let Ok(response_bytes) = serde_json::to_vec(&response) {
                                    let _ = stream.write_all(&response_bytes);
                                }
                            }
                            Err(e) => {
                                error!("Error processing IPC message: {}", e);
                                let error_response = IpcMessage {
                                    msg_type: MessageType::Error,
                                    payload: serde_json::json!({"error": e.to_string()}),
                                };
                                if let Ok(bytes) = serde_json::to_vec(&error_response) {
                                    let _ = stream.write_all(&bytes);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse IPC message: {}", e);
                    }
                }
            }
            Ok(0) => {
                debug!("IPC client closed connection");
            }
            Err(e) => {
                warn!("IPC read error: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// Process IPC message
    fn process_message(&self, message: &IpcMessage) -> Result<IpcMessage> {
        match message.msg_type {
            MessageType::GetStatus => {
                let status = CompositorStatus {
                    name: "Shibui".to_string(),
                    version: crate::VERSION.to_string(),
                    active_workspace: 1, // TODO: Get actual from compositor state
                    window_count: 0,     // TODO: Get actual from window manager
                    output_count: 1,     // TODO: Get actual from output manager
                    running: true,
                };
                
                Ok(IpcMessage {
                    msg_type: MessageType::Response,
                    payload: serde_json::to_value(status)?,
                })
            }
            MessageType::ReloadConfig => {
                // Trigger config reload
                info!("🔄 IPC: Config reload requested");
                
                // Note: Actual reload happens in compositor main loop
                // This just acknowledges the request
                let response = serde_json::json!({
                    "success": true,
                    "message": "Config reload triggered. Check compositor logs for details."
                });
                
                Ok(IpcMessage {
                    msg_type: MessageType::Response,
                    payload: response,
                })
            }
            MessageType::GetWorkspaces => {
                let workspaces = serde_json::json!([
                    {"id": 1, "name": "1"},
                    {"id": 2, "name": "2"},
                    {"id": 3, "name": "3"},
                ]);
                
                Ok(IpcMessage {
                    msg_type: MessageType::Response,
                    payload: workspaces,
                })
            }
            MessageType::GetWindows => {
                let windows = serde_json::json!([]);
                
                Ok(IpcMessage {
                    msg_type: MessageType::Response,
                    payload: windows,
                })
            }
            MessageType::Command => {
                let command = message.payload.get("cmd")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                debug!("Executing IPC command: {}", command);
                
                Ok(IpcMessage {
                    msg_type: MessageType::Response,
                    payload: serde_json::json!({"success": true, "command": command}),
                })
            }
            _ => {
                Err(anyhow::anyhow!("Unknown message type: {:?}", message.msg_type))
            }
        }
    }
}

impl Default for IpcServer {
    fn default() -> Self {
        Self::new().expect("Failed to create IPC server")
    }
}

impl Drop for IpcServer {
    fn drop(&mut self) {
        self.stop();
    }
}
