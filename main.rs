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

# Add your files
git add .

# Create your first commit
git commit -m "upload files"

# Create a repository on GitHub/GitLab/etc., then add it as remote
git remote add origin https://github.com/cramangako/miss-edwards-kernel.git

# Push to the remote
git push -u origin main
