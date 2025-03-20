@echo off
echo building raytracer...
cargo.exe build --color=always --message-format=json-diagnostic-rendered-ansi --all --all-targets --profile dev

echo writing image
cd D:\Things\GitHub\RustRaytracer\target\debug\
Raytracer.exe > image.ppm