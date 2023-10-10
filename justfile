# Build for bare metal/non-OS target
bbm:
    cargo build --target thumbv7em-none-eabihf

# build crate as a freestanding executable on Linux
bl:
    cargo rustc -- -C link-arg=-nostartfiles

# build crate as a freestanding executable on Windows
# subsystem may be console OR windows!
bw:
    cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"

# build crate as a freestanding executable on Mac
bm:
    cargo rustc -- -C link-args="-e __start -static -nostartfiles"

build:
    cargo clean && cargo build



docs:
    cargo doc --open