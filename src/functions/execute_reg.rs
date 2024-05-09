use runas::Command as RunasCommand;

pub fn execute_reg(reg_dir: &str) {
    let service_regedit = RunasCommand::new("regedit")
        .arg("/S")
        .arg(reg_dir)
        .status()
        .expect("Falha ao executar regedit.exe");

    if service_regedit.success() {
        println!("Regedit changed successfully");
    } else {
        eprintln!("Failed to change Regedit");
    }
}