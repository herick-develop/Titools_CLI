use std::fs;

use crate::functions::msi_install::msi_install;

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

    if let Err(err) = msi_install(&format!("{}\\Apps\\AnyDesk.msi", dir_titools)) {
        println!("Error installing AnyDesk: {}", err);
        return Err(err.into());
    }

    Ok(())
}
