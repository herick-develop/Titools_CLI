use std::io;
use runas::Command as RunasCommand;

pub fn install_local(dir_titools: &str) -> io::Result<()> {

    let output = RunasCommand::new("cmd")
    .args(&["/C", &format!("{}\\Modules\\Ferramentas\\tech_copy.bat",dir_titools), "/qb"])
    .status();

    match output {
        Ok(status) => {
            if status.success() {
                println!("Installed with sucess");
            } else {
                println!("Error while install Tech CLI: {}", status.code().unwrap_or(-1));
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("failed to execute command: {}", e);
            Err(e.into())
        }
    }

}