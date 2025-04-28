// the value is measured in bytes and it's defaulted to 2GiB.
pub const MAX_STORAGE_CAPACITY: i64 = 128 * 1024 * 1024 * 1024;

pub mod fs {
    #[derive(Clone)]
    pub enum FileSystem{
        Ext4(String),
        Fat32(String),
        Btrfs(String),
        Swap(String)
    }

    impl FileSystem {
        pub fn get_mount_point(&self) -> String {
            match self { 
                Self::Ext4(mp)
                | Self::Fat32(mp)
                | Self::Swap(mp)
                | Self::Btrfs(mp) => mp.to_owned(),
            }
        }
    }

    #[derive(Clone)]
    pub enum PartitionKind {
        LinuxFS,
        UEFIBoot,
        LinuxSwap,
        Home,
    }

    impl PartitionKind {
        pub fn get_partition_kind_name(&self) -> String {
            match self {
                Self::LinuxFS => "linux fs".to_string(),
                Self::UEFIBoot => "boot".to_string(),
                Self::Home => "home".to_string(),
                Self::LinuxSwap => "swap".to_string()
            }
        }
    }

    #[derive(Clone)]
    pub struct Partition {
        pub number: u8,
        pub kind: PartitionKind,
        pub file_system: Option<FileSystem>,
        pub section_size: i64
    }
}

pub use fs::{Partition, PartitionKind};

#[derive(Clone)]
pub struct VirtualDisk{
    pub name: String,
    pub capacity: i64,
    pub used: i64,
    pub partitions: Vec<Partition>
}

#[allow(clippy::new_without_default)]
impl VirtualDisk {
    pub fn new() -> Self{
        Self{
            name: "dev/sda".to_string(),
            capacity: MAX_STORAGE_CAPACITY,
            used: 0_i64,
            partitions: Vec::new()
        }
    }
}
