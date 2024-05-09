use std::io;
use runas::Command as RunasCommand;

use super::printer_spool_restart::printer_spool_restart;
use crate::functions::execute_reg::execute_reg;

pub fn printer_share(dir_titools: &str) -> Result<(), Box<dyn std::error::Error>> {

    execute_reg(&format!("{}\\Dependencies\\registros\\habilitaImpressora.reg",dir_titools));

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
            println!("Printer Feature Enabled: {feature_name}");
            Ok(())
        }
        Err(e) => {
            println!("Error while enabling the printer services: {}", e);
            Err(e)
        }
    }
}