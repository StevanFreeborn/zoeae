![logo](./src/images/icon.ico)

# Zoeae

**Zoeae** is a small, native desktop text editor written in Rust. It's a learning project focused on building a fast, minimal editor with a Markdown preview capability — an experimental alternative to lightweight editors like Notepad with built-in preview features.

## Features

- **Lightweight editor:** fast startup and minimal UI surface.
- **Markdown preview:** live or on-demand preview of Markdown files.
- **Cross-platform Rust codebase:** implemented in Rust (see [src/](src/)).

### Install

Pre-built artifacts and installers are published on GitHub Releases. See the [releases](https://github.com/StevanFreeborn/zoeae/releases) page for the latest published assets:

## Usage

Launch the binary and open files from the app UI or run the binary directly from the terminal.

## License

See [LICENSE.txt](LICENSE.txt) for license details.

## Contributing

Contributions, bug reports, and feature requests are welcome. Please open issues or pull requests on GitHub.

## Build

Install Rust toolchain (recommended via `rustup`) with `cargo`. For Windows: have a working C toolchain (MSVC or GNU) if building native dependencies.

### Clone the repository

```pwsh
git clone https://github.com/StevanFreeborn/zoeae.git
cd zoeae
```

### Build (release)

```pwsh
cargo build --release
```

> [!NOTE]
> The built binary will be in `target/release/` (e.g. `target/release/zoeae`).

### Run (debug)

```pwsh
cargo run
```

### Install locally

```pwsh
cargo install --path .
```

> [!NOTE]
> This installs the `zoeae` binary into your Cargo bin directory (usually `~/.cargo/bin`).
