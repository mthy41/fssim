pub fn convert_gib(bytes: i64) -> String {
    let gib = bytes/(1024 * 1024 * 1024);
    let mut gib_str = gib.to_string();
    gib_str.push_str("GiB");
    gib_str
}
