# MissEdwardsKernel

tuff kernel loading =========100%

tuff kernel loaded

lauching miss edwards kernel
# USING UEFI BY THE WAY

# also you can just get versions from releases if u want


# step 1

to compile

have rust and cargo installed with like rustup

then run `rustup target add x86_64-unknown-uefi`

then clone with git and when you in master directory run `cargo build --release --target x86_64-unknown-uefi`

now our .efi is at go to `target/x86_64-unknown-uefi/release/miss-edwards-kernel.efi`

now, either run in vm or be tuff and use real usb

# USB STEPS (optional)

insert usb make gpt partition table, have it formatted to FAT32 and then copy it to usb with under USB/EFI/BOOT/BOOTX64.EFI

then boom u got the official miss edwards kernel

v0.0.0

