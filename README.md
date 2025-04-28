# fssim
Requirements to compile:
- `rustc`
- `cargo` 
In the cloned repo, run `cargo run`
## summary
The default virtual disk drive is `dev/sda`.
- `lsblk` list all partitions and current disk info.
- `gdisk <device>` enter gdisk with specified disk.
- `mkfs <args>` format a partition. Run `mkfs -h` for more details.
- `ls` list all available commands.
- `exit` exit the program.
