# 🔧 Rust OS (Educational Operating System)

This repository contains the source code for a simple educational operating system written in [Rust](https://www.rust-lang.org), following the [Phil Opp's Writing an OS in Rust](https://os.phil-opp.com/) series.

The goal of this project is to work on low-level systems programming and understand how operating systems work under the hood by building one from scratch.

## 🚀 What This Project Includes

- A minimal bootable kernel written in `no_std` Rust
- Direct access to VGA text buffer for screen output
- Custom panic handler and interrupt handling
- Memory management (coming soon)
- Paging and heap allocation (WIP)
- No external operating system or runtime required

## 🛠️ Technologies Used

- Rust (nightly)
- Cargo
- QEMU (virtual machine for testing)
- `bootimage`, `x86_64`, `lazy_static`, `spin`, and other low-level crates

## 💻 How to Run

Make sure you have the following installed:

- `rustup` with nightly toolchain:  
  ```bash
  rustup install nightly
  rustup component add rust-src --toolchain nightly
