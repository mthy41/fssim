use std::error::Error;
use crate::system::VirtualDisk;
use crate::utils::format_gib;

pub fn run(vd: &VirtualDisk) -> Result<(), Box<dyn Error>>{
    let (cap, used) = (
        vd.capacity.to_owned(),
        vd.used.to_owned()
    );
    let (cap_str, used_str) = (
        format_gib(cap), 
        format_gib(used)
    );

    let available: i64 = cap - used;
    println!("Disk name:      {}", vd.name.to_owned());
    println!("Total storage:  {} ({})", cap, cap_str);
    println!("Used:           {} ({})", used, used_str);
    println!("Available:      {} ({})", available, format_gib(available));
    Ok(())
}
