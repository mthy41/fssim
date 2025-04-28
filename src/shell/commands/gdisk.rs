use std::num;

use crate::{shell::user_input, system::{fs::FileSystem, Partition, PartitionKind, VirtualDisk}, utils::parse_mem_scale};

pub fn run(vd: &mut VirtualDisk, args: Vec<String>) -> Result<(), String> {

    //check if is there args
    if args.get(1).is_none(){ 
        return Err("gdisk: no device name was given.".to_string());
    }

    //check if device exists
    if !(vd.name.eq(args.get(1).unwrap())){
        let err = format!("gdisk: device {} not found.", args.get(1).unwrap());
        return Err(err);
    }
    
    let mut input: String = String::new();
    let pwd = "Command (? to help) ".to_string();

    let mut action_counter = 0_i32;
    let mut vd_buffer = vd.clone();

    loop {
        _ = user_input(&mut input, &pwd);
        match input.as_str() {
            "?" => { println!("
            l    list partitions all partition types
            p    print partition table
            o    create new empty GUID table (GPT)
            n    add new partition
            d    delete partition
            w    write table on the disk and exit
            ?    print this menu
            e    exit
                "); },
            "e" => { 
                if action_counter > 0 {
                    println!("do you want to leave without saving? {} changes", action_counter);
                    let mut confirmation = String::new();
                    _ = user_input(&mut confirmation, &"Y or N: ".to_string());
                    match confirmation.as_str() { 
                        "y" | "Y" => {break;},
                        _         => {continue;}
                    };
                } else { break; }
            },
            "p" => list_partitions(&vd_buffer.partitions),
            "o" => {
                create_empty_gpt(&mut vd_buffer);
                action_counter += 1;
            },
            "n" => add_new_partition(&mut vd_buffer, &mut action_counter),
            "w" => {
                if action_counter > 0 {
                    println!("do you really want to save changes in disk? {} changes", action_counter);
                    let mut confirmation = String::new();
                    _ = user_input(&mut confirmation, &"Y or N: ".to_string());
                    match confirmation.as_str() { 
                        "y" | "Y" => { write_changes(&mut vd_buffer, vd); break;},
                        _         => {continue;}
                    };
                } else { break; }
            },
            "d" => {
                if delete_partition(&mut vd_buffer).is_ok(){
                    action_counter += 1;
                }
            },
            "l" => {
                println!("
            [8300] -> linux filesystem
            [8302] -> Home
            [8200] -> linux swap
            [ef00] -> UEFI boot
                ")
            }
            
            _ => {println!("Unknown command.");}
        };
    }
    
    println!("exiting gdisk");
    Ok(())
}

fn add_new_partition(vd_buffer: &mut VirtualDisk, counter: &mut i32){
    let vd_buffer_available = vd_buffer.capacity - vd_buffer.used;

    let mut pt_number_b = String::new();
    _ = user_input(&mut pt_number_b, &"partition number: ".to_string());
    if pt_number_b.parse::<u8>().is_err(){
        println!("invalid partition number.");
        return;
    }
    let pt_number_b: u8 = pt_number_b.parse::<u8>().unwrap();
    for vd_pt_buffer in vd_buffer.partitions.clone(){
        if pt_number_b == vd_pt_buffer.number{
            println!("partition number already in use.");
            return;
        }
    }
    
    let mut pt_kind_b = String::new(); 
    _ = user_input(&mut pt_kind_b, &"Hex code (Enter = 8300): ".to_string());
    let pt_kind_b: PartitionKind = match pt_kind_b.as_str() {
        ""     => PartitionKind::LinuxFS,
        "8300" => PartitionKind::LinuxFS,
        "8302" => PartitionKind::Home,
        "8200" => PartitionKind::LinuxSwap,
        "ef00" => PartitionKind::UEFIBoot,
        _      => {
            println!("invalid partition hex code.");
            return;
        }
    };

    let mut pt_size_b = String::new();
    _ = user_input(&mut pt_size_b, &"section size: ".to_string());

    let pt_size_b = parse_mem_scale(&pt_size_b);
    if let Err(e) = pt_size_b{
        println!("{e}");
        return;
    }
    let pt_size_b: i64 = pt_size_b.unwrap().get_bytes();
    if vd_buffer_available < pt_size_b{
        println!("there is not enough memory for that.");
        return;
    }

    let partition_b = Partition {
        number: pt_number_b,
        kind: pt_kind_b,
        file_system: None,
        section_size: pt_size_b
    };

    vd_buffer.partitions.push(partition_b);
    vd_buffer.used += pt_size_b;
    *counter += 1;
    println!("partition created on {}{}", vd_buffer.name, pt_number_b);
}

pub fn list_partitions(part_table: &[Partition]){
    if part_table.is_empty(){
        println!("The partition table is empty.");
        return;
    }
    
    for p in part_table{
        let mut mount_point = "".to_string();
        if p.file_system.clone().is_some(){
            mount_point = p.file_system.clone().unwrap().get_mount_point()
        }
        let mut file_system = "".to_string();
        if let Some(fs) = p.file_system.clone(){
            match fs {
                FileSystem::Ext4(_) => {file_system = "ext4".to_string();},
                FileSystem::Fat32(_) => {file_system = "fat32".to_string();},
                FileSystem::Btrfs(_) => {file_system = "btrfs".to_string();},
                FileSystem::Swap(_) => {file_system = "swap".to_string();},
            }
        }
        println!("[dev/sda{}]\n   size: {}\n   kind: {}\n   mount point: {}\n   file system: {}",  
            p.number, 
            p.section_size, 
            p.kind.get_partition_kind_name(), 
            mount_point,
            file_system
        );
    }
}

fn create_empty_gpt(vd_buffer: &mut VirtualDisk){
    vd_buffer.partitions = Vec::new();
}

fn write_changes(vd_buffer: &mut VirtualDisk, vd: &mut VirtualDisk){
    vd.used = vd_buffer.used;
    vd.partitions = vd_buffer.partitions.clone();
}

fn delete_partition(vd_buffer: &mut VirtualDisk) -> Result<(), ()>{
    let mut pt_number_b = String::new();
    _ = user_input(&mut pt_number_b, &"partition number: ".to_string());
    if pt_number_b.parse::<u8>().is_err(){
        println!("invalid partition number.");
        return Err(());
    }
    let pt_number_b = pt_number_b.parse::<u8>().unwrap();
    for p in vd_buffer.partitions.clone(){
        if pt_number_b == p.number{
            vd_buffer.partitions.retain(|n| n.number != pt_number_b);
            vd_buffer.used -= p.section_size;
            println!("deleting partition {}{}", vd_buffer.name, pt_number_b);
            return Ok(());
        }
    }
    println!("no partition found.");
    Err(())
}   

