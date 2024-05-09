use runas::Command as RunasCommand;
use std::fs;

//use runas
pub fn anydesk_install(dir_titools: &str) -> Result<(), Box<dyn std::error::Error>> {

    let path_exe: &str = "C:\\Program Files (x86)\\AnyDesk\\AnyDesk.exe";
    let path_msi: &str = "C:\\Program Files (x86)\\AnyDeskMSI\\AnyDeskMSI.exe";

    if fs::metadata(path_msi).is_ok() {
        println!("AnyDeskMSI has installed!");
        return Ok(());
    } else if fs::metadata(path_exe).is_ok() {
        println!("AnyDeskEXE has installed!");
        return Ok(());
    } else {
        println!("Anydesk Path not Found :: Installing...");
    }

    let output = RunasCommand::new("msiexec")
        .args(&["/i", &format!("{}\\Apps\\AnyDesk.msi",dir_titools), "/qb"])
        .status();
    
    match output {
        Ok(status) => {
            if status.success() {
                println!("Password Configured with sucess");
            } else {
                println!("Error while configure password: {}", status.code().unwrap_or(-1));
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("failed to execute command: {}", e);
            Err(e.into())
        }
    }
}
