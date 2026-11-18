# OS2 - electric boogaloo

I made an [OS](https://github.com/skadewdl3/rust-osdev), then gave up. Now im making another one. Yay.

## How to compile and run?
- Get dependencies:
  - Qemu (qemu-base with GTK backend should suffice).
  - OVMF
  - Rust Nightly (make sure to get rust source with `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu`)
- Clone the repo
- Make `run` executable: `chmod +x run`
- Compile the bootloader: `cargo build`
- Bootstrap UEFI firmware and os image: `./run bootstrap`
- Run in Qemu: `./run`
