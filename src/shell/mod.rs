use std::io::{stdin, Write};
use commands::Command;
use super::system::VirtualDisk;

mod commands;

pub fn exec_shell(vd: &mut VirtualDisk) -> Result<(), Box<dyn std::error::Error>>{
    let pwd = String::from("root:# ");
    let mut buffer = String::new();

    loop {
        user_input(&mut buffer, &pwd)?;
        if buffer.eq("exit") { break; }
        if buffer.is_empty() { continue; }

        let filtered = commands::filter(&buffer);
        if filtered.is_err(){
            let e = filtered.err().unwrap();
            println!("ERROR: {}", &e);
            continue;
        }

        match filtered.ok().unwrap() {
            Command::Lsblk => {
                let r = commands::lsblk::run(vd); 
                if r.is_err(){ println!("lsblk: something went wrong.") }
            },
            Command::Gdisk(args) => {
                if let Err(e) = commands::gdisk::run(vd, args){
                    println!("{e}");
                }
            },
            Command::Mfks(args) => {
                if let Err(e) = commands::mkfs::run(vd, args){
                    println!("{e}");
                }
            },
            Command::Ls() => {    commands::ls::run();}
        }
    }

    Ok(())
}

pub fn user_input(buffer: &mut String, pwd: &String) 
    -> Result<(), std::io::Error>
{
    let si = stdin();
    print!("{}", pwd);
    std::io::stdout().flush()?;
    buffer.clear();
    si.read_line(buffer)?;
    *buffer = buffer.trim().to_string();
    Ok(())
}
