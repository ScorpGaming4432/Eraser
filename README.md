# Eraser
Markdown editing software built in rust.

# Rust Project Setup Guide

## 1. Install Rust Compiler and Tools
### Windows/Linux/macOS
Run this command in your terminal (requires internet connection):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Press `1` and Enter when prompted to proceed with default installation
- Restart your terminal after installation completes

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
git clone https://github.com/username/projectname.git
cd projectname
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
- Or find the binary in `target/release/projectname`

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
    bash
    rustup self uninstall
