@echo off
cargo build --release ^
    --target x86_64-pc-windows-msvc ^
    -Z build-std=std,panic_abort ^
    -Z build-std-features=default,optimize_for_size