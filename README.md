# RustSeek

RustSeek is a command-line interface (CLI) application written in Rust that facilitates communication with Ollama instances over a REST API. It provides a simple and efficient way to interact with Ollama's capabilities directly from your terminal.

## Features

- Communicate with Ollama instances using REST API.
- Easy-to-use command-line interface.
- Built with performance and reliability in mind.

## Dependencies

RustSeek uses the following dependencies:

- **[reqwest](https://crates.io/crates/reqwest)**: A simple and powerful HTTP client for Rust.
  ```toml
  reqwest = { version = "0.12", features = ["json"] }
  tokio = { version = "1", features = ["full"] }
    serde = { version = "1.0.217", features = ["derive"] }
  ```
## Installation

To install RustSeek, you need to have [Rust](https://www.rustup.rs) installed on your machine. Once you have Rust set up, you can clone the repository and build the project:

```bash
git clone https://github.com/diljith369/rustseek.git
cd rustseek
cargo build --release
```

## Usage

After building the project, you can run RustSeek from the command line. Here’s a basic example of how to use it:

## TODO
```bash
./target/release/rustseek <command> [options]
```

Replace `<command>` with the specific command you want to execute and provide any necessary options.


## License

This project is licensed under the MIT License.

## Acknowledgments

- Thanks to the Rust community for their support and contributions.
- Special thanks to the developers of the libraries used in this project.

```
