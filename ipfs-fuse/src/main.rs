extern crate fuser;

use std::env;
use fuser::Filesystem;

struct IpfsFilesystem;

impl Filesystem for IpfsFilesystem {
}

fn main() {
  let args: Vec<String> = env::args().collect();
  // dbg!(args);
  match args.len() {
    // one argument passed
    2 => {
      let mountpoint = &args[1];
      fuser::mount2(IpfsFilesystem, &mountpoint, &[]);
    }
    _ => {
      println!("Usage: {} <mountpoint>", args[0]);
    }
  }
}
