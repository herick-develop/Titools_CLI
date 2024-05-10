use std::fs;

use crate::functions::msi_install::msi_install;

pub fn seven_zip_install(dir_titools: &str) -> Result<(), Box<dyn std::error::Error>> {

    let path_exe: &str = "C:\\Program Files\\7-Zip\\7z.exe";

    if fs::metadata(path_exe).is_ok() {
        println!("7z has installed!");
        return Ok(());
    } else {
        println!("7z Path not Found :: Installing...");
    }

    if let Err(err) = msi_install(&format!("{}\\Apps\\7z_x64.msi", dir_titools)) {
        println!("Error installing AnyDesk: {}", err);
        return Err(err.into());
    }

    Ok(())
}
