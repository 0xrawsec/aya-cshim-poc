As you may know, BPF **CO-RE** is not supported by Aya from Rust code.
Supporting **CO-RE** directly from Rust code is a consequent work as it requires
modifying the `rustc` target to compile to bpf and this is in the pipe of Aya team.
However, a work around can be made by defining so-called shim functions in C
to access structures you need **CO-RE** support for. These functions are
then linked to your eBPF binary using `bpf-linker`, but this is totally
transparent to you. The only things you need to do are:
1. defining your shim functions in C
1. create a `build.rs` to compile the shim to eBPF
1. use it in Rust via `extern C` functions. 

This repo gives you examples on how to define shim functions.

## Prerequisites

1. Install a rust stable toolchain: `rustup install stable`
1. Install a rust nightly toolchain with the rust-src component: `rustup toolchain install nightly --component rust-src`
1. Install custom bpf-linker (see next section)

## Install custom `bpf-linker`

**!!!! IMPORTANT: This might not be true for ever as `bpf-linker` will be
updated so that you don't have to do all that custom installation**

Making shim working requires embedding **BTF** information (used for relocation)
inside your eBPF program. However the way Rust generates Debugging Information 
used for BTF (because a part of your program is Rust) makes the Linux kernel 
un-happy when trying to load/relocate your program. So, to generate BTF
information your kernel will be happy with, it requires both LLVM to be patched
and `bpf-linker` to be patched to process LLVM Debugging Information
in such a way your eBPF binary contains BTF information the Linux kernel can
handle.

Basically, to install a `bpf-linker` which will work with this example and with 
C-shim in general, you should follow the instructions here: https://github.com/vadorovsky/aya-btf-maps-experiments. **Right after you can find a set of condensed instructions** to achieve the same goal.

### Instructions to build custom LLVM
1. clone forked llvm: `git clone https://github.com/vadorovsky/llvm-project.git`
1. `cd llvm-project`
1. `git checkout bpf-fixes`
1. `mkdir build && cd build`
1. build llvm: `CC=clang CXX=clang++ cmake -DCMAKE_BUILD_TYPE=Release -DLLVM_PARALLEL_LINK_JOBS=1 -DLLVM_ENABLE_LLD=1 -GNinja ../llvm/`

### Instructions to build custom bpf-linker

**!!! That process will replace your current `bpf-linker`** so if you want
to revert those changes go back to the original procedure of installing `bpf-linker` (i.e. `cargo install bpf-linker`).

1. Make sure you have built a custom LLVM
1. clone forked `bpf-linker`: `git clone https://github.com/vadorovsky/bpf-linker.git`
1. `cd bpf-linker`
1. `git checkout fix-di`
1. Install custom `bpf-linker` : `LLVM_SYS_160_PREFIX=/path/to/llvm-project/build cargo install --path . --no-default-features --features system-llvm bpf-linker`


## Build eBPF

```bash
cargo xtask build-ebpf
```

To perform a release build you can use the `--release` flag.
You may also change the target architecture with the `--target` flag.

## Build Userspace

```bash
cargo build
```

## Run

```bash
RUST_LOG=info cargo xtask run
```
