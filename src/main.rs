use partition_identity::{PartitionID, PartitionSource};
use self::PartitionSource::*;
use sysinfo::{System, SystemExt, DiskExt};
use sha2::{Sha256, Digest};
use uname::uname;
use std::process::Command;

fn main() {
    // Assuming it is a Linux-based system 
    // Let't try to get the disk info
    let syst = System::new();
    let disk = syst.get_disks();
    let partition = &disk[0].get_name().to_str().unwrap();
    let path = format!("/dev/{}", partition); 

    let disk_uuid = PartitionID::get_source(UUID, &path).map(|id| id.id).unwrap();

    let mut hasher = Sha256::new();
    hasher.input(disk_uuid.as_bytes());
    let mut result = format!("{:X}", hasher.result());
    println!("First Fingerprint (Disk's UUID): {}", result);

    let info = uname().unwrap();

    let mut hasher = Sha256::new();
    let second_hash = format!("{}-{}", disk_uuid, info.version);
    hasher.input(second_hash.as_bytes());
    result = format!("{:X}",hasher.result());
    println!("Second fingerprint (Disk's UUID with some uname info) {}", result);
    
    let output = Command::new("lshw").arg("-short").output().expect("lshw installed");

    let mut hasher = Sha256::new();
    let third_hash = format!("{}-{}-{:?}", disk_uuid, info.version, output);
    hasher.input(third_hash.as_bytes());
    result = format!("{:X}",hasher.result());
    println!("Third fingerprint (Disk's UUID with some uname info and lshw) {}", result);
    
}
