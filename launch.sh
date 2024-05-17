#!/bin/bash

if [[ $1 == "clean" ]]; then
    cargo clean
    rm -rf isofiles os.iso
    exit
fi

cargo build -r
mkdir -p isofiles/boot/grub
cat > isofiles/boot/grub/grub.cfg << EOF
set timeout=5
set default=0

menuentry "DemOS" {
    multiboot2 /boot/demos
    boot
}
EOF
cp target/x86_64-demos/release/demos isofiles/boot/
grub-mkrescue -o os.iso isofiles
qemu-system-x86_64 -cdrom os.iso
