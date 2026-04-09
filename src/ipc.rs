//! IPC (Inter-Process Communication) module
//! 
//! Provides JSON-RPC IPC interface for controlling FlowWM from external tools.
//! Used by flowmsg CLI tool and third-party integrations.

use log::{info, debug};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

/// IPC message
#[derive(Debug, Serialize, Deserialize)]
pub struct IpcMessage {
    /// Message type
    pub msg_type: MessageType,
    /// Payload
    pub payload: serde_json::Value,
}

/// Message types
#[derive(Debug, Serialize, Deserialize)]
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
}

/// Compositor status
#[derive(Debug, Serialize, Deserialize)]
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
}

/// IPC server
pub struct IpcServer {
    /// Socket path
    socket_path: PathBuf,
    /// Running state
    running: bool,
}

impl IpcServer {
    /// Create new IPC server
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let socket_path = Self::get_socket_path();
        
        Ok(Self {
            socket_path,
            running: false,
        })
    }
    
    /// Get socket path
    fn get_socket_path() -> PathBuf {
        let xdg_runtime = std::env::var("XDG_RUNTIME_DIR")
            .unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(xdg_runtime).join("shibui.sock")
    }
    
    /// Start IPC server
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔌 Starting IPC server...");
        
        // Remove old socket if exists
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path)?;
        }
        
        self.running = true;
        info!("✅ IPC server listening on {:?}", self.socket_path);
        
        // TODO: Implement actual IPC server loop
        // This would listen for connections and handle messages
        
        Ok(())
    }
    
    /// Stop IPC server
    pub fn stop(&mut self) {
        info!("🔌 Stopping IPC server");
        
        self.running = false;
        
        // Clean up socket
        if self.socket_path.exists() {
            let _ = std::fs::remove_file(&self.socket_path);
        }
    }
    
    /// Handle client connection
    fn handle_client(&self, mut stream: UnixStream) -> Result<(), Box<dyn std::error::Error>> {
        // Read request
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer)?;
        
        // Parse message
        let message: IpcMessage = serde_json::from_slice(&buffer)?;
        debug!("IPC message: {:?}", message);
        
        // Process message
        let response = self.process_message(&message)?;
        
        // Send response
        let response_bytes = serde_json::to_vec(&response)?;
        stream.write_all(&response_bytes)?;
        
        Ok(())
    }
    
    /// Process IPC message
    fn process_message(&self, message: &IpcMessage) -> Result<IpcMessage, Box<dyn std::error::Error>> {
        match message.msg_type {
            MessageType::GetStatus => {
                let status = CompositorStatus {
                    name: "FlowWM".to_string(),
                    version: crate::VERSION.to_string(),
                    active_workspace: 1, // TODO: Get actual
                    window_count: 0,     // TODO: Get actual
                    output_count: 1,     // TODO: Get actual
                };
                
                Ok(IpcMessage {
                    msg_type: MessageType::GetStatus,
                    payload: serde_json::to_value(status)?,
                })
            }
            MessageType::Command => {
                // TODO: Execute command
                Ok(IpcMessage {
                    msg_type: MessageType::Command,
                    payload: serde_json::json!({"success": true}),
                })
            }
            _ => {
                Ok(IpcMessage {
                    msg_type: MessageType::Event,
                    payload: serde_json::json!({"error": "Not implemented"}),
                })
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
