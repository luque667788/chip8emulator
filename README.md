# Chip8 Emulator

This project is a Chip8 emulator/interpreter written from scratch in Rust. It was created for learning purposes and is still under development. The emulator runs in the browser via `wasm-bindgen` and `wgpu/webgl`.

You can test and use the emulator at this link: [Chip8 Emulator](https://luque667788.github.io/chip8emulator/).

## Features

- Emulates the Chip8 system.
- Loads ROMs from files.
- Uses a custom solution for rendering with WGPU for fast graphics.
- Supports basic sound as the original Chip8 did.
- The CPU clock is time-based and stabilizes if the running loop takes too much time.
- Compiles to WebAssembly using wasm-pack.

## Building the Project

This project is compiled using `wasm-pack` tooling. After cloning the repository, run the following command:

```sh
wasm-pack build --target web --release
```
For faster build times, use the --dev flag instead:
```sh
wasm-pack build --target web --dev
```

To run the website locally, you can serve it by running the following command in the root folder of the project:
 ```sh
  npx serve
```

## Design Philosophy
This project was created for learning purposes to gain a deeper understanding of hardware architectures. Chip8 is one of the best architectures to start with for creating an emulator. To add some challenge, I decided to implement the graphics from scratch using WGPU, which allows for platform-agnostic graphics programming and works well with Rust.

For the graphics, I aimed to optimize and understand the render pipeline by innovating a bit. Instead of sending all the vertices of the triangles to the vertex buffer, the only data the vertex buffer receives is the index of the vertex (as it normally does). It has  also access to a uniform buffer containing the data of the Chip8 display, theoretically it can be only 64x32 bits of size. I then map this to the screen and perform all calculations to find the vertex coordinates inside the shader, making the program significantly faster than the common method of drawing a texture on the CPU and sending it to the GPU or by sending lots of data as vertices by the vertex buffer.

## Contributing
Feel free to fork this repository and submit pull requests. Any contributions to improve the code quality and add new features are welcome!

## License
This project is licensed under the MIT License.

## Extra
Thank you for checking out this project! If you have any questions or feedback, feel free to open an issue or contact me directly on [Linkedin](https://www.linkedin.com/in/luiz-henrique-salles-de-oliveira-mendon%C3%A7a-3963b928b/).