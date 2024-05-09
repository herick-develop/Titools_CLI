use std::io;
use runas::Command as RunasCommand;

pub fn install_local(dir_titools: &str) -> io::Result<()> {

  let source_path = format!("{}\\Apps\\bin\\ti.exe", dir_titools);
  let destination_path = "C:\\ti.exe";

  // Combine copy operation and error handling          echo todos | Xcopy %1\Apps\MS1_NFS C:\MS1_NFS  /E /H /C /I
  let output = RunasCommand::new("cmd")
  .args(&["/C", "echo todos | xcopy \"\\\\10.11.50.50\\driveti\\driveti\\Sistemas_Umuprev\\Titools\\Apps\\bin\\ti.exe\" \"C:\\ti.exe\" /E /H /C /I & pause",])
        .status();
    
    match output {
        Ok(status) => {
            if status.success() {
                println!("Password Configured with sucess");
            } else {
                println!("Error while configure password: {}", status.code().unwrap_or(-1));
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("failed to execute command: {}", e);
            Err(e.into())
        }
    }

}