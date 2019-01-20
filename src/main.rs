extern crate partition_identity;
extern crate sysinfo;
use partition_identity::{PartitionID, PartitionSource};
use self::PartitionSource::*;
use sysinfo::{System, SystemExt, DiskExt};

fn main() {

    let syst = System::new();

    let disk = syst.get_disks();
    let partition = &disk[0].get_name().to_str().unwrap();
    let path = format!("/dev/{}", partition); 

    let disk_uuid = PartitionID::get_source(UUID, &path).map(|id| id.id).unwrap();
    println!("UUID: {}", disk_uuid);

}
