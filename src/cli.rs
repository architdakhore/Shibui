//! ShibUI CLI Tool
//! 
//! Command-line interface for controlling ShibUI via IPC.
//! Similar to hyprctl for Hyprland.

use std::env;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "status" => cmd_status(),
        "workspaces" => cmd_workspaces(),
        "windows" => cmd_windows(),
        "monitors" => cmd_monitors(),
        "activewindow" => cmd_activewindow(),
        "reload" => cmd_reload(),
        "kill" => cmd_kill(),
        "dispatch" => cmd_dispatch(&args[2..]),
        "help" | "--help" | "-h" => print_help(),
        "version" | "--version" | "-V" => print_version(),
        _ => {
            eprintln!("Unknown command: {}", command);
            print_help();
        }
    }
}

fn get_socket_path() -> PathBuf {
    let xdg_runtime = env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(xdg_runtime).join("flowwm.sock")
}

fn send_command(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let socket_path = get_socket_path();
    
    let mut stream = UnixStream::connect(&socket_path)?;
    stream.write_all(command.as_bytes())?;
    
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    
    Ok(response)
}

fn cmd_status() {
    match send_command("status") {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Failed to get status: {}", e),
    }
}

fn cmd_workspaces() {
    match send_command("workspaces") {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Failed to get workspaces: {}", e),
    }
}

fn cmd_windows() {
    match send_command("windows") {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Failed to get windows: {}", e),
    }
}

fn cmd_monitors() {
    match send_command("monitors") {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Failed to get monitors: {}", e),
    }
}

fn cmd_activewindow() {
    match send_command("activewindow") {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Failed to get active window: {}", e),
    }
}

fn cmd_reload() {
    match send_command("reload") {
        Ok(_) => println!("✅ Configuration reloaded"),
        Err(e) => eprintln!("Failed to reload: {}", e),
    }
}

fn cmd_kill() {
    match send_command("killactive") {
        Ok(_) => println!("✅ Window killed"),
        Err(e) => eprintln!("Failed to kill window: {}", e),
    }
}

fn cmd_dispatch(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: flowmsg dispatch <command>");
        return;
    }
    
    let command = args.join(" ");
    match send_command(&format!("dispatch {}", command)) {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Failed to dispatch: {}", e),
    }
}

fn print_help() {
    println!(r#"
� ShibUI CLI Tool v{}

USAGE:
    shibuictl <COMMAND> [ARGS]

COMMANDS:
    status          Get compositor status
    workspaces      List workspaces
    windows         List windows
    monitors        List monitors
    activewindow    Get active window
    reload          Reload configuration
    kill            Kill active window
    dispatch        Execute command
    help            Show this help
    version         Show version

EXAMPLES:
    shibuictl status
    shibuictl workspaces
    shibuictl dispatch exec alacritty
    shibuictl dispatch workspace 2
    shibuictl dispatch movetoworkspace 3

For more information, see the documentation at:
https://github.com/yourusername/shibui
"#, crate::VERSION);
}

fn print_version() {
    println!("FlowWM {}", crate::VERSION);
}

mod VERSION {
    pub const VERSION: &str = "0.1.0";
}
