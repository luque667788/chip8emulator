# Rust CHIP8 EMULATOR
# Chip8 Emulator

This project is a Chip8 emulator/interpreter made from scratch written in Rust. This project was made for learning purposes and is still under development.


## Features

- Emulates the Chip8 system
- load ROMs from files.
- Uses WGPU for rendering graphics
- Runs the CPU and timing clock on a separate thread.
- Supports basic sound as chip8 did.


## Building the Project
this project is compiled using wasm-pack tooling
after cloning the repo:
run the following line: wasm-pack build --target web --release

for faster build times use the flag --dev instead

then run the website locally you can serve it by runnig npx serve on the root folder of the project

## Architecture Support

The emulator supports two architectures: legacy and modern implementations of the Chip8.

## Contributing

Contributions are welcome! 

## License

This project is licensed under the MIT License