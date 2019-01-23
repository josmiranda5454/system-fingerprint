# system-fingerprint
an application which is capable of uniquely fingerprinting your system. The fingerprint should be different even on two similar systems.  Feel free to explore several approaches.

## Building and Running
1. ```cd <project_source>```
2. ```cargo build```
3. ```sudo src/target/debug/system-fingerprint```

##### References
1. [partition-identity crate](https://crates.io/crates/partition-identity)
2. [sysinfo](https://crates.io/crates/sysinfo)
3. [sha2](https://crates.io/crates/sha2)
4. [uname](https://crates.io/crates/uname)
5. [Rust book](https://doc.rust-lang.org/book/)
