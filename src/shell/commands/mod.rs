pub mod lsblk;
pub mod gdisk;
pub mod mkfs;
pub mod ls;

pub enum Command {
    Lsblk,
    Gdisk(Vec<String>),
    Mfks(Vec<String>),
    Ls(),
}

pub fn filter(com: &str) -> Result<Command, String>{
    let com = com.to_string();
    let com_list: Vec<&str> = com.split_whitespace().collect();

    //i don't care about boundary check hahahahahahahahahaha ! !
    match com_list[0] {
        "lsblk" => Ok(Command::Lsblk),
        "gdisk" => Ok(Command::Gdisk(convert_args(com_list))),
        "mkfs"  => Ok(Command::Mfks(convert_args(com_list))),
        "ls"    => Ok(Command::Ls()),
        _ => Err(String::from("invalid command."))
    }
}

fn convert_args(vec_str: Vec<&str>) -> Vec<String> {
    vec_str.iter().map(|s| s.to_string()).collect()
}
