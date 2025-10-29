### [Wersja Polska README](ReadmeEN.md)

# Eraser
Markdown editing software built in rust.

# Rust Project Setup Guide

## 1. Install Rust Compiler and Tools
Follow the Guide from https://www.rust-lang.org/tools/install

### Verify Installation
```bash
rustc --version
cargo --version
rustup --version
```
You should see version numbers for each tool.

---

## 2. Compile a GitHub Project
1. Clone the repository:
```bash
git clone https://github.com/ScorpGaming4432/Eraser.git
cd Eraser
```

2. Build the project:
```bash
cargo build --release
```
- This downloads dependencies and compiles optimized binaries

3. Run the project:
```bash
cargo run --release
```
- Or find the binary in `target/release/Eraser`

---

## Troubleshooting
- **Permission errors**: Prepend commands with `sudo` on Linux/macOS
- **Proxy issues**: Set `http_proxy` and `https_proxy` environment variables
- **Outdated Rust**: Update with `rustup update`
- **Compiler errors**: Ensure you have `build-essential` (Linux) or Xcode (macOS) installed


## Key Notes:
1. The installer automatically includes:
   - `rustc` (compiler)
   - `cargo` (package manager)
   - `rustup` (toolchain manager)

2. Project compilation handles:
   - Dependency resolution
   - Incremental compilation
   - Binary generation

3. The `--release` flag produces optimized builds (slower compilation but faster execution)

4. Uninstall anytime with:
```bash
rustup self uninstall
```

