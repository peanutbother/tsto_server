# TSTO Server

[![Build Status](https://github.com/peanutbother/tsto_server/actions/workflows/release.yml/badge.svg?branch=main)](https://github.com/peanutbother/tsto_server/releases/latest) ![GitHub Downloads (all assets, latest release)](https://img.shields.io/github/downloads/peanutbother/tst_server/latest/total?label=Downloads)

A fullstack Dioxus application built with Rust and Nix, supporting macOS, Linux and Windows.

## 🚀 Features

Play your town again like nothing happened.

- anonymous login
- email account with signup (work in progress)
- change the current event (work in progress)
- play with friends (not yet implemented)

### 🔧 Technical details

- Fullstack using dioxus
- Built using Rust and Nix
- Cross-platform builds (Linux, macOS, Windows)
- GitHub Actions for CI/CD

## 📥 Installation

You can download the latest pre-built binaries from the [release](https://github.com/peanutbother/tsto_server/releases/latest) page.

## 📖 Usage

On first launch the server creates config and data directories depending on if portable mode is activated.
This means that if you run `tsto_server --portable` it will create configs and data next to the executable.
IF `--portable` is not set, the server will use the platform's default config and data path.

### ENV Variables

You can also override configuration with environment variables.
As long as no cli arg is set, these variables will take precendence.
So the hierarchy is cli args -> env variables -> configuration.

The following ENV variables are supported:
`DATABASE`, `DLC_FOLDER`, `LOG_ASSETS`*, `PORT`, `SERVER_ADDRESS`

*`LOG_ASSETS` will be parsed as enabled if the value equals either to `true` (case ignored) or to `1`.

### Non-Portable Config and Data Paths

The server stores configuration and data in a platform-agnostic way using the crate [project-dirs]() if `--portable` is not set.

### config dir

| OS | Path |
| --- | --- |
|Linux | /home/`username`/.config/tsto_server |
|Windows | C:\Users\\`username`\AppData\Local\peanutbother\tsto_server |
|macOS | /Users/`username`/Library/Application Support/de.peanutbother.tsto_server |

### data dir

| OS | Path |
| --- | --- |
|Linux | /home/`username`/.local/share/tsto_server |
|Windows | C:\Users\\`username`\AppData\Local\peanutbother\tsto_server\data |
|macOS | /Users/`username`/Library/Application Support/de.peanutbother.tsto_server |

## 🛠️ Building the Project

### Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Node.js & npm](https://nodejs.org/)
- [Nix (optional)](https://nixos.org/download.html)
- [dioxus-cli](https://dioxuslabs.com/learn/0.6/getting_started/)
- [wasm-bindgen-cli](https://github.com/rustwasm/wasm-bindgen)

### Cloning the Repository

```sh
git clone https://github.com/peanutbother/tsto_server.git
cd tsto_server
```

### Install dioxus-cli and wasm-bindgen-cli

```sh
cargo install dioxus-cli wasm-bindgen-cli
```

### Install npm dependencies

```sh
npm install
```

### Using dioxus-cli

```sh
dx bundle --fullstack --release --out-dir dist
```

This will build the project and put the resulting binary and dashboard into `dist`.

### Using Nix (Linux/macOS)

```sh
nix build .#default --print-build-logs
```

This will build the project and put the resulting binary and dashboard into `result/bin`.

## 🏗️ CI/CD Pipeline (GitHub Actions)

This project includes a GitHub Actions workflow to:

- Build the project for macOS, Linux, and Windows
- Archive build artifacts
- Create a GitHub release with the compiled binaries

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE.md) file for details.

## 🤝 Contributing

Pull requests are welcome! For major changes, please open an issue first.

## 📬 Contact

For any questions, reach out via GitHub Issues.

---

Made with ❤️ using Rust, Dioxus, and Nix.
