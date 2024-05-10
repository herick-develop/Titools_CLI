use clap::{App, Arg};
use dotenv::dotenv;
use std::env;

mod functions {
    pub mod execute_reg;
    pub mod msi_install;
}

mod modules {
    pub mod anydesk {
        pub mod anydesk_id;
        pub mod anydesk_install;
        pub mod anydesk_password;
    }
    pub mod seven_zip {
        pub mod seven_zip_install;
    }
    pub mod firefox {
        pub mod firefox_install;
    }
    pub mod printer {
        pub mod printer_spool_restart;
        pub mod printer_share;
    }
    pub mod titools {
        pub mod titools;
    }
    pub mod tech_cli {
        pub mod tech_cli_update;
    }
}

use modules::anydesk::{
    anydesk_id::anydesk_id,
    anydesk_install::anydesk_install,
    anydesk_password::anydesk_password
};
use modules::seven_zip::{
    seven_zip_install::seven_zip_install
};
use modules::firefox::{
    firefox_install::firefox_install
};
use modules::printer::{
    printer_spool_restart::printer_spool_restart,
    printer_share::printer_share
};
use modules::titools::{
    titools::titools
};
use modules::tech_cli::{
    tech_cli_update::tech_cli_update
};

fn main() {

    dotenv().ok();

    let dir_titools = match env::var("dir_titools") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Failed to read dir_titools environment variable: {}", e);
            return;
        }
    };

    let matches = App::new("titools_cli")
        .version("1.1")
        .author("Herick Lucas")
        .about("tools command")
        .arg(
            Arg::with_name("restart_spool")
                .long("rimp")
                .help("Restart the Printer Spool")
        )
        .arg(
            Arg::with_name("printer_share")
                .long("shimp")
                .help("Share the printer")
        )
        .arg(
            Arg::with_name("anydesk_id")
                .long("idany")
                .help("AnyDesk ID")
        )
        .arg(
            Arg::with_name("anydesk_install")
                .long("iany")
                .help("AnyDesk Install")
        )
        .arg(
            Arg::with_name("7z_install")
                .long("i7z")
                .help("7z Install")
        )
        .arg(
            Arg::with_name("firefox_install")
                .long("iff")
                .help("Firefox Install")
        )
        .arg(
            Arg::with_name("anydesk_set_password")
                .long("pwany")
                .help("Anydesk Password")
        )
        .arg(
            Arg::with_name("ti_tools")
                .long("titools")
                .help("TITools")
        )
        .arg(
            Arg::with_name("tech_cli_update")
                .long("utech")
                .help("Update Tech CLI")
        )
        .get_matches();

    if matches.is_present("restart_spool") {
        printer_spool_restart().unwrap_or_else(|e| eprintln!("Failed to restart printer spooler: {}", e));
    }

    if matches.is_present("printer_share") {
        printer_share(&dir_titools).unwrap_or_else(|e| eprintln!("Failed to share printer: {}", e));
        printer_spool_restart().unwrap_or_else(|e| eprintln!("Failed to restart printer spooler: {}", e));
        println!("If not solve restart this computer");
    }

    if matches.is_present("anydesk_id") {
        anydesk_id().unwrap_or_else(|e| eprintln!("Failed to get AnyDesk ID: {}", e));
    }

    if matches.is_present("anydesk_install") {
        anydesk_install(&dir_titools).unwrap_or_else(|e| eprintln!("Failed to install AnyDesk: {}", e));
        anydesk_id().unwrap_or_else(|e| eprintln!("Failed to get AnyDesk ID: {}", e));
        anydesk_password(&dir_titools).unwrap_or_else(|e| eprintln!("Failed to install AnyDesk: {}", e));
    }

    if matches.is_present("anydesk_set_password") {
        anydesk_password(&dir_titools).unwrap_or_else(|e| eprintln!("Failed to set AnyDesk password: {}", e));
    }

    if matches.is_present("7z_install") {
        seven_zip_install(&dir_titools).unwrap_or_else(|e| eprintln!("Failed to install 7z: {}", e));
    }

    if matches.is_present("firefox_install") {
        firefox_install(&dir_titools).unwrap_or_else(|e| eprintln!("Failed to install 7z: {}", e));
    }

    if matches.is_present("ti_tools") {
        titools(&dir_titools).unwrap_or_else(|e| eprintln!("Failed to Open TITools: {}", e));
    }
    if matches.is_present("tech_cli_update") {
        // install().unwrap_or_else(|e| eprintln!("Failed to Open TITools: {}", e));
        tech_cli_update(&dir_titools).unwrap_or_else(|e| eprintln!("Failed to Open TITools: {}", e));
    }
}