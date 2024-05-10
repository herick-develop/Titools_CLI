use std::fs;

use crate::functions::msi_install::msi_install;

pub fn firefox_install(dir_titools: &str) -> Result<(), Box<dyn std::error::Error>> {

    let path_exe: &str = "C:\\Program Files\\Mozilla Firefox\\firefox.exe";

    if fs::metadata(path_exe).is_ok() {
        println!("Firefox has installed!");
        return Ok(());
    } else {
        println!("Firefox Path not Found :: Installing...");
    }

    if let Err(err) = msi_install(&format!("{}\\Apps\\Firefox_Setup.msi", dir_titools)) {
        println!("Error installing Firefox: {}", err);
        return Err(err.into());
    }

    Ok(())
}
