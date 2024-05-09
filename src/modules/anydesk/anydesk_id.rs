use std::fs;
use std::process::{Command, Output};

pub fn anydesk_id() -> Result<(), std::io::Error> {
    let path_exe: &str = "C:\\Program Files (x86)\\AnyDesk\\AnyDesk.exe";
    let path_msi: &str = "C:\\Program Files (x86)\\AnyDeskMSI\\AnyDeskMSI.exe";
    let path_current: &str;

    if fs::metadata(path_msi).is_ok() {
        path_current = path_msi;
    } else if fs::metadata(path_exe).is_ok() {
        path_current = path_exe;
    } else {
        println!("Anydesk Path not Found");
        return Ok(());
    }

    let output: Result<Output, std::io::Error> = Command::new(path_current)
        .args(&["--get-id"])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("AnyDesk ID: {}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("Command failed with error: {}", stderr);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }

    Ok(())
}
