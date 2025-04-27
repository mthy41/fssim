// the value is measured in bytes and it's defaulted to 2GiB.
pub const MAX_STORAGE_CAPACITY: i64 = 2 * 1024 * 1024 * 1024;

pub struct VirtualDisk{
    pub name: String,
    pub capacity: i64,
    pub used: i64,
}

#[allow(clippy::new_without_default)]
impl VirtualDisk {
    pub fn new() -> Self{
        Self{
            name: "dev/sda1".to_string(),
            capacity: MAX_STORAGE_CAPACITY,
            used: 0_i64,
        }
    }
}
