use std::process::Command;
use std::env;

fn main() {
    let tools = vec![
        ("Git", "git --version"),
        ("Rust", "rustc --version"),
        ("Cargo", "cargo --version"),
    ];

    for (tool_name, command) in tools {
        match check_tool_version(command) {
            Ok(version) => {
                println!("{} is installed: {}", tool_name, version);
            }
            Err(_) => {
                println!("{} is not installed or not found.", tool_name);
            }
        }
    }
}

fn check_tool_version(command: &str) -> Result<String, String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout);
                Ok(version.trim().to_string())
            } else {
                Err("Failed to retrieve version".to_string())
            }
        }
        Err(_) => Err("Failed to execute command".to_string()),
    }
}

