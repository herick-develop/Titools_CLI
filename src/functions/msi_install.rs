use runas::Command as RunasCommand;

//use runas
pub fn msi_install(msi_path: &str) -> Result<(), Box<dyn std::error::Error>> {

    let output = RunasCommand::new("msiexec")
        .args(&["/i", msi_path, "/qb"])
        .status();
    
    match output {
        Ok(status) => {
            if status.success() {
                println!("MSI install with sucess");
            } else {
                println!("Error while install msi: {}", status.code().unwrap_or(-1));
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("failed to execute command: {}", e);
            Err(e.into())
        }
    }
}
