//0.0.0
  
#![no_std]
#![no_main]

use core::time::Duration;
use uefi::prelude::*;
use uefi::boot::stall;

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init().unwrap();

    log::info!("MISS EDWARDS KERNEL v0.0.0");
    log::info!("welcome my son");

    loop {
        stall(Duration::from_secs(1));
    }
}
