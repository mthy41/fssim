mod shell;
pub mod utils;
pub mod system;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    use system::VirtualDisk;

    let mut vd = VirtualDisk::new();

    shell::exec_shell(&mut vd)?;
    Ok(())
}

