pub enum ByteScale {
    GiB(i64),
    MiB(i64),
    KiB(i64),
    Byte(i64)
}

pub fn format_gib(bytes: i64) -> String {
    let gib = bytes/(1024 * 1024 * 1024);
    let mut gib_str = gib.to_string();
    gib_str.push_str("GiB");
    gib_str
}

pub fn parse_mem_scale(s: &str) -> Result<ByteScale, String> {
    let s = s.trim().to_lowercase();
    let split_indx = s.find(|c: char| c.is_ascii_alphabetic())
        .unwrap_or(s.len());
    let (digit, scale) = s.split_at(split_indx);
    let digit = digit.parse();
    if digit.is_err() {
        return Err("ERROR: Invalid memory format".to_string());
    }
    let digit: i64 = digit.unwrap();

    match scale {
        "gib" => Ok(ByteScale::GiB(digit * 1024 * 1024 * 1024)),
        "mib" => Ok(ByteScale::MiB(digit * 1024 * 1024)),
        "kib" => Ok(ByteScale::KiB(digit * 1024)),
        ""    => Ok(ByteScale::Byte(digit)),
        _     => Err("ERROR: Invalid memory format".to_string())
    }
}

pub fn parse_gib_to_bytes(gib: &str) -> Result<i64, String> {
    todo!()
}
