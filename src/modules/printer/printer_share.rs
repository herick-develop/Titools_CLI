use std::io;
use runas::Command as RunasCommand;

use super::printer_spool_restart::printer_spool_restart;

pub fn printer_share(dir_titools: &str) -> Result<(), Box<dyn std::error::Error>> {

    let service_regedit = RunasCommand::new("regedit")
        .arg("/S")
        .arg(format!("{}\\Dependencies\\registros\\habilitaImpressora.reg", dir_titools))
        .status()
        .expect("Falha ao executar regedit.exe");

    if service_regedit.success() {
        println!("Regedit changed successfully");
    } else {
        eprintln!("Failed to change Regedit");
    }
    printer_spool_restart().unwrap_or_else(|e| eprintln!("Failed to restart printer spooler: {}", e));

    let features = vec![
        "Printing-Foundation-Features",
        "Printing-Foundation-InternetPrinting-Client",
        "Printing-Foundation-LPDPrintService",
        "Printing-Foundation-LPRPortMonitor",
    ];

    for feature in features {
        if let Err(err) = enable_feature(feature) {
            eprintln!("Failed to enable feature {}: {}", feature, err);
        }
    }

    Ok(())
}

fn enable_feature(feature_name: &str) -> Result<(), io::Error> {
    let result = RunasCommand::new("DISM")
        .arg("/Online")
        .arg("/Enable-Feature")
        .arg(format!("/FeatureName:{}", feature_name))
        .arg("/All")
        .arg("/Quiet")
        .arg("/NoRestart")
        .status()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to execute DISM command: {}", e)))
        .and(Ok(()));

    match result {
        Ok(_) => {
            println!("Printer Services enabled");
            Ok(())
        }
        Err(e) => {
            println!("Error while enabling the printer services: {}", e);
            Err(e)
        }
    }
}