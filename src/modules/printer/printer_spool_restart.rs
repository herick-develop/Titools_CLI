use runas::Command as RunasCommand;

pub fn printer_spool_restart() -> Result<(), Box<dyn std::error::Error>> {
    let stop_command = RunasCommand::new("net")
        .args(&["stop", "spooler"])
        .status()?;
    if stop_command.success() {} else {
        eprintln!("Failed to stop printer spooler");
    }

    let start_command = RunasCommand::new("net")
        .args(&["start", "spooler"])
        .status()?;
    if start_command.success() {} else {
        eprintln!("Failed to start printer spooler");
    }
    println!("Printer spooler restarted successfully");

    Ok(())
}