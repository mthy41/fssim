pub mod lsblk;
pub mod gdisk;

pub enum Command {
    Lsblk,
}

pub fn filter(com: &str) -> Result<Command, String>{
    match com {
        "lsblk" => Ok(Command::Lsblk),
        _ => Err(String::from("invalid command."))
    }
}
