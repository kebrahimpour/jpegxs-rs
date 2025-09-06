
Konversation geöffnet. 1 Nachricht gelesen.

Direkt zum Inhalt
Gmail mit Screenreadern verwenden
1 von 198.619
JPEG XS – MVP Scaffolding (jpegxs-rs) + CI + Instructions
Posteingang
VB


Keyvan Ebrahimpour <k1.ebrahimpour@gmail.com>
08:48 (vor 0 Minuten)
an mich

Hi Keyvan,

Proceeding with defaults:
- License: MIT/Apache-2.0 dual
- MVP pixel format: yuv422p 8-bit (optionally yuv444p later)
- Repo: jpegxs-rs
- FFI with TangKii/jxs prepared, feature-gated and disabled by default

Below is the complete scaffolding package you can paste with Claude Code to generate files, or create manually. It includes workspace layout, initial Rust code stubs, CI workflows, and instructions to initialize and push to GitHub. I’ve also included a minimal CI for your jxs fork.

1) File tree
jpegxs-rs/
Cargo.toml
rust-toolchain.toml
.gitignore
.editorconfig
LICENSE-APACHE
LICENSE-MIT
CODEOWNERS
.github/
workflows/
rust-ci.yml
.github/ISSUE_TEMPLATE/
bug_report.yml
feature_request.yml
.github/PULL_REQUEST_TEMPLATE.md
crates/
jpegxs-core/
Cargo.toml
src/
lib.rs
colors.rs
dwt.rs
quant.rs
entropy.rs
packet.rs
types.rs
jpegxs-io/
Cargo.toml
src/
lib.rs
bitio.rs
yuv.rs
bitstream.rs
jpegxs-cli/
Cargo.toml
src/
main.rs
jpegxs-ffi/
Cargo.toml
build.rs
src/
lib.rs
tests/
integration_cross_impl.rs
roundtrip.rs
benches/
encode_decode.rs
tools/
validate.sh
corpus/
README.md

2) Key files (contents)

Cargo.toml (workspace)
[workspace]
resolver = "2"
members = [
"crates/jpegxs-core",
"crates/jpegxs-io",
"crates/jpegxs-cli",
"crates/jpegxs-ffi",
]

[workspace.package]
edition = "2021"
license = "MIT OR Apache-2.0"
authors = ["Keyvan Ebrahimpour"]

[workspace.dependencies]
anyhow = "1"
thiserror = "1"
clap = { version = "4", features = ["derive"] }
image = "0.25"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
criterion = { version = "0.5", optional = true }

rust-toolchain.toml
[toolchain]
channel = "stable"

.gitignore
/target
**/*.swp
**/*.DS_Store

.editorconfig
root = true
[*]
end_of_line = lf
insert_final_newline = true
charset = utf-8
indent_style = space
indent_size = 4

CODEOWNERS
* @YOUR_GITHUB_USERNAME

LICENSE-MIT
<standard MIT license text>

LICENSE-APACHE
<standard Apache-2.0 license text>

.github/workflows/rust-ci.yml
name: Rust CI
on: [push, pull_request]
jobs:
fmt_clippy:
runs-on: ${{ matrix.os }}
strategy:
matrix:
os: [ubuntu-latest, macos-latest]
steps:
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@stable
- name: Cargo fmt
run: cargo fmt --all -- --check
- name: Clippy
run: cargo clippy --all-targets --workspace -- -D warnings
test:
runs-on: ${{ matrix.os }}
strategy:
matrix:
os: [ubuntu-latest, macos-latest]
steps:
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@stable
- run: cargo test --workspace --all-features -- --nocapture
build_release:
runs-on: ubuntu-latest
steps:
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@stable
- run: cargo build --release -p jpegxs-cli

crates/jpegxs-core/Cargo.toml
[package]
name = "jpegxs-core"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = { workspace = true }
thiserror = { workspace = true }
serde = { workspace = true }

src/lib.rs
pub mod colors;
pub mod dwt;
pub mod quant;
pub mod entropy;
pub mod packet;
pub mod types;

use types::{ImageView8, ImageOwned8, Bitstream, EncoderCfg, DecoderCfg};

pub fn encode_frame(_input: ImageView8, _cfg: &EncoderCfg) -> anyhow::Result<Bitstream> {
// TODO: color transform -> DWT -> quant -> entropy -> packet
Err(anyhow::anyhow!("encoder not implemented (MVP scaffold)"))
}

pub fn decode_frame(_bs: &Bitstream, _cfg: &DecoderCfg) -> anyhow::Result<ImageOwned8> {
// TODO: unpack -> entropy -> dequant -> inverse DWT -> color transform
Err(anyhow::anyhow!("decoder not implemented (MVP scaffold)"))
}

// Other modules (colors.rs, dwt.rs, quant.rs, entropy.rs, packet.rs, types.rs)
// will contain type definitions and stubbed functions with TODOs.

crates/jpegxs-io/Cargo.toml
[package]
name = "jpegxs-io"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = { workspace = true }
serde = { workspace = true }

src/lib.rs
pub mod bitio;
pub mod yuv;
pub mod bitstream;

src/bitio.rs
// Simple bit reader/writer stubs with TODOs

src/yuv.rs
// Planar YUV 4:2:2 loader/writer accepting width/height from CLI

src/bitstream.rs
// Bitstream container type used between core and CLI

crates/jpegxs-cli/Cargo.toml
[package]
name = "jpegxs-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = { workspace = true }
clap = { workspace = true }
jpegxs-core = { path = "../jpegxs-core" }
jpegxs-io = { path = "../jpegxs-io" }

src/main.rs
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "jpegxs")]
#[command(about = "JPEG XS CLI (MVP scaffold)")]
struct Cli { #[command(subcommand)] cmd: Cmd }
#[derive(Subcommand)]
enum Cmd {
Encode { in_path: String, out_path: String, width: u32, height: u32, #[arg(default_value="yuv422p")] fmt: String },
Decode { in_path: String, out_path: String },
Validate { in_path: String, #[arg(long, default_value="jxs")] compare_with: String },
}
fn main() -> anyhow::Result<()> { let _cli = Cli::parse(); println!("MVP CLI scaffold"); Ok(()) }

crates/jpegxs-ffi/Cargo.toml
[package]
name = "jpegxs-ffi"
version = "0.1.0"
edition = "2021"

[features]
ffi_ref = []

[dependencies]
anyhow = { workspace = true }

build.rs
// Configure bindgen/cbindgen here later; no-op for now

src/lib.rs
#[cfg(feature = "ffi_ref")]
pub mod jxs_ref; // to be added later

tests/roundtrip.rs
#[test]
fn roundtrip_placeholder() { assert!(true); }

tests/integration_cross_impl.rs
#[test]
fn cross_impl_placeholder() { assert!(true); }

benches/encode_decode.rs
fn main() { /* criterion to be added later */ }

tools/validate.sh
#!/usr/bin/env bash
set -euo pipefail
# Placeholder: will call CLI and compute PSNR once implemented

echo "Validation scaffold"

corpus/README.md
Minimal corpus placeholder. Add small public-domain images and synthetic generators.

3) jxs fork CI (add to your TangKii/jxs fork)
.github/workflows/jxs-ci.yml
name: jxs CI
on: [push, pull_request]
jobs:
build_linux:
runs-on: ubuntu-latest
steps:
- uses: actions/checkout@v4
- name: Install deps
run: sudo apt-get update && sudo apt-get install -y build-essential cmake ninja-build pkg-config
- name: Configure
run: cmake -B build -G Ninja -DCMAKE_BUILD_TYPE=Release
- name: Build
run: ninja -C build
- name: Package artifacts
uses: actions/upload-artifact@v4
with:
name: jxs-bin
path: build/**

4) Commands to initialize and push
# Create repo and push
cd ~/dev
mkdir jpegxs-rs && cd jpegxs-rs
# Create files as above, or paste this email into Claude Code to generate them
git init
git add -A
git commit -m "feat: MVP scaffolding for jpegxs-rs workspace"
# Create a new GitHub repo named jpegxs-rs, then:
git remote add origin git@github.com:YOUR_GITHUB_USERNAME/jpegxs-rs.git
git branch -M main
git push -u origin main

# For the jxs fork, enable Actions and add the workflow file.

5) Next steps
- Wire up jpegxs-io bit I/O and YUV loader
- Implement 5/3 DWT scalar path
- Add minimal quant + packetizer and a no-op decoder loop
- Introduce PSNR helper and roundtrip test
- Optional: enable ffi_ref and bind to jxs for cross-validation

e to tailor this for yuv444p first, or add Windows CI, l adjust.




