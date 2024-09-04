# Rust CHIP8 EMULATOR
# Chip8 Emulator

This project is a Chip8 emulator/interpreter made from scratch written in Rust. This project was made for learning purposes and is still under development. It runs on in the browser via wasm-bindgen and wgpu/webgl

you can test and use the emulator at this link: https://luque667788.github.io/chip8emulator/


## Features

- Emulates the Chip8 system
- load ROMs from files.
- Uses custom solution for rendering with WGPU for fast rendering graphics
- Supports basic sound as chip8 did.
- cpu clock is time based and stabilizes if the running loop is taking too much time 
- compiles to wasm using webpack


## Building the Project
this project is compiled using wasm-pack tooling
after cloning the repo:
run the following line: wasm-pack build --target web --release

for faster build times use the flag --dev instead

then run the website locally you can serve it by runnig npx serve on the root folder of the project

## Desing Phylisosy

I wanted to create this project to get more closer to understanding the hardware. I always wanted to create an emulator and chip8 
is one of the best architerctures to start. OF course to add some challenge i decided to implement the graphics from scratch. I used wgpu for as it allows platform agnostic graphics programmign and works well with rust. For the graphics i tried to optimize and also really understand the render pipline by inovating a little bit. Instead of the program sending all of the vertices of the triangels and etc to the vertex buffer in this project the only data the vertex buffer recevier is the index os the vertex (as alwasy)
 and it has acces to a uniform buffer and this uniform buffer contains the data of the chip8 display. theoretically it cooul be be only 64*32 bits. then i map to the screen and do all the calculations to find the vertex cordinates inside the shader making the program significantly faster then the other method which is used by most emulator that is drawing a texture and then sinding it to the gpu. 
## Chip8 Architecture Support

The emulator supports two architectures: legacy and modern implementations of the Chip8.

## Contributing

Contributions are welcome! 

## License

This project is licensed under the MIT License