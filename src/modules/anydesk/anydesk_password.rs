use runas::Command as RunasCommand;

pub fn anydesk_password(dir_titools: &str) -> Result<(), Box<dyn std::error::Error>> {

    let service = RunasCommand::new("cmd")
        .args(&["/C", &format!("{}\\Modules\\anydesk_set_password.bat", dir_titools)])
        .status();

    match service {
        Ok(status) => {
            if status.success() {
                println!("Password Configured with sucess");
            } else {
                println!("Error while configure password: {} (without license)", status.code().unwrap_or(-1));
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("failed to execute command: {}", e);
            Err(e.into())
        }
    }
}