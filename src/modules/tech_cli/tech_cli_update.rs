use std::io;
use runas::Command as RunasCommand;

pub fn tech_cli_update(dir_titools: &str) -> io::Result<()> {

    let output = RunasCommand::new("cmd")
    .args(&["/C", &format!("{}\\Modules\\Ferramentas\\tech_update.bat",dir_titools), "/qb"])
    .status();

    match output {
        Ok(status) => {
            if status.success() {
                println!("Tech CLI Updated with sucess");
            } else {
                println!("Error while Update Tech CLI: {}", status.code().unwrap_or(-1));
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("failed to execute command: {}", e);
            Err(e.into())
        }
    }

}