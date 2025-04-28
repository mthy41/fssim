use crate::system::{fs::FileSystem, VirtualDisk};

pub fn run(vd: &mut VirtualDisk, args: Vec<String>) -> Result<(), String>{
    let mut ptn_arg: Option<u8> = None;
    let mut fs_arg: Option<String> = None;
    let mut mp_arg: Option<String> = None;

    for i in (1..args.len()).step_by(2) {
        match args[i].as_str() {
            "-f" => {
                if let Some(fs) = args.get(i+1){
                    fs_arg = Some(fs.to_string());
                } else {return Err("mkfs: no filesystems argument was given.".to_string());}
            },
            "-p" => {
                if let Some(part) = args.get(i+1){
                    if part.parse::<u8>().is_err(){return Err("mkfs: invalid partition number.".to_string());}
                    for p in vd.partitions.clone(){
                        if p.number == part.parse().unwrap(){
                            ptn_arg = Some(p.number);
                        }
                    }
                } else {return Err("mkfs: no partition number was given.".to_string());}
            },
            "-m" => {
                if let Some(mp) = args.get(i+1){
                    mp_arg = Some(mp.to_owned());
                } else {return Err("mkfs: no mountpoint was given.".to_string());}
            },
            "-l" => {
                println!("
                ext4
                fat32
                btrfs
                swap
                ");
                return Ok(());
            }
            _    => {print_help(); return Ok(()); },
        };
    }

    if ptn_arg.is_none(){
        return Err("mkfs: partition number argument missing.".to_string());
    }
    if fs_arg.is_none(){
        return Err("mkfs: filesystem argument missing.".to_string());
    }
    if mp_arg.is_none(){
        return Err("mkfs: mount point argument missing".to_string());
    }

    let fs_b = match fs_arg.unwrap().as_str() {
        "ext4"  => FileSystem::Ext4(mp_arg.unwrap()),
        "fat32" => FileSystem::Fat32(mp_arg.unwrap()),
        "btrfs" => FileSystem::Btrfs(mp_arg.unwrap()),
        "swap"  => FileSystem::Swap(mp_arg.unwrap()),
        _       => {return Err("mkfs: unknown filesystem.".to_string());}
    };

    for i in 0..vd.partitions.len(){
        if vd.partitions[i].number == ptn_arg.unwrap(){
            vd.partitions[i].file_system = Some(fs_b);
            break;
        }
    }

    Ok(())
}

fn print_help(){
    println!("
        -f<arg>    filesystem
        -p<arg>    partition number
        -m<arg>    mount point
        -l         list available filesystems
        -h         prints this menu
        ")
}
