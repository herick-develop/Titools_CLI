use runas::Command as RunasCommand;

pub fn titools(dir_titools: &str) -> Result<(), Box<dyn std::error::Error>> {

    let service = RunasCommand::new("mshta")
        .args(&[&format!("{}\\TiTools.hta", dir_titools)])
        .status();

    match service {
        Ok(status) => {
            if status.success() {
                println!("TITools openned sucess");
            } else {
                println!("TITools open with error code: {}", status.code().unwrap_or(-1));
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("failed to execute command: {}", e);
            Err(e.into())
        }
    }
}
