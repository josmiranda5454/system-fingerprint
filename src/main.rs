use partition_identity::{PartitionID, PartitionSource};
use self::PartitionSource::*;
use sysinfo::{System, SystemExt, DiskExt};
use sha2::{Sha256, Digest};


fn main() {

    let syst = System::new();

    let disk = syst.get_disks();
    let partition = &disk[0].get_name().to_str().unwrap();
    let path = format!("/dev/{}", partition); 

    let disk_uuid = PartitionID::get_source(UUID, &path).map(|id| id.id).unwrap();
    println!("UUID:{}", disk_uuid);

    let mut hasher = Sha256::new();
    hasher.input(disk_uuid.as_bytes());
    let result = format!("{:X}",hasher.result());
    println!("Fingerprint: {}", result);
}
