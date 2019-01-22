use partition_identity::{PartitionID, PartitionSource};
use self::PartitionSource::*;
use sysinfo::{System, SystemExt, DiskExt};
use sha2::{Sha256, Digest};
use uname::uname;
use std::process::Command;

fn main() {
    // Assuming it is a Linux-based system 
    let disk = System::new();
    let disk = disk.get_disks();
    let path = format!("/dev/{}", &disk[0].get_name().to_str().unwrap()); 
    // Let's get the disk's UUID based on the path given if found
    let disk_uuid = PartitionID::get_source(UUID, &path)
                                .map(|id| id.id).unwrap();

    // Let's calculate the first fingerprint
    let mut hasher = Sha256::new();
    hasher.input(disk_uuid.as_bytes());
    let mut result = format!("{:X}", hasher.result());
    println!("First Fingerprint (Disk's UUID): {}", result);

    // Use crate to get uname's OS version
    let info = uname().unwrap();

    // Let's concatenate uname with disk's UUID
    let mut hasher = Sha256::new();
    let second_hash = format!("{}-{}", disk_uuid, info.version);
    hasher.input(second_hash.as_bytes());
    result = format!("{:X}",hasher.result());
    println!("Second fingerprint (Disk's UUID with OS version info) {}", result);
    
    // Let's use Hardware Lister to get info about hardware paths, etc.
    let output = Command::new("lshw").arg("-short").output().expect("lshw installed?");
    let output_str = String::from_utf8(output.stdout).unwrap();

    // Let's concatenate lshw, disk's UUID, and OS version
    let mut hasher = Sha256::new();
    let third_hash = format!("{}-{}-{}", disk_uuid, info.version, output_str);
    hasher.input(third_hash.as_bytes());
    result = format!("{:X}",hasher.result());
    println!("Third fingerprint (Disk's UUID with some uname info and lshw) {}", result);

    // Let's get output of cmdline
    let output = Command::new("/bin/cat").arg("/proc/cmdline").output().expect("Didn't get info");
    let output_str = String::from_utf8(output.stdout).unwrap();
    
    // Let's concatenate the disk's UUID and cmdline
    let mut hasher = Sha256::new();
    let fourth_hash = format!("{}-{}", disk_uuid, output_str);
    hasher.input(fourth_hash.as_bytes());
    result = format!("{:X}",hasher.result());
    println!("Fourth fingerprint (Disk's UUID with /proc/cmdline) {}", result);
}
