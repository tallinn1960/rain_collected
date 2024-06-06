# Demo repository for mixing and benchmarking Rust, C++, Swift and Zig

Various implementations of a leetcode problem solution in Rust, C++, Swift and Zig 
are mixed together and called from each of those languages. This repository
provides an example how to do that using cmake or cargo. Benchmarks for those functions
are included using either the criterion crate or Google benchmark 
(that does not include Zig, which is benchmarked by criterion only).

The following instructions assume you have Rust installed by `rustup`.

## macOS installation

To build and run the examples on macOS, one needs cmake, the ninja build tool and 
google benchmark  installed along with Xcode 15. Make sure that your Swift 
compiler version is at least 5.9. As now Zig is included as well, make sure to have
Zig installed. All is available by HomeBrew.

cmake, ninja, Zig and Google benchmark can be installed by HomeBrew:

```
brew install cmake
brew install ninja
brew install google-benchmark
brew install zig
```

## Linux installation

Linux installation is a bit more complicated. The code relies on the Apple version
of clang compatible with Swift and the most recent Swift version (at least 5.9).
Also, Google benchmark isn't available as a package in general, at least not in a suitable
version. Furthermore the ninja build system is required for Swift support.

### Install ninja

On Ubuntu: 

`sudo apt install ninja-build`

### Install Swift and clang

Head over to [Swift.org](https://www.swift.org) and download the tarball for
Linux containing Swift and clang. Unpack it at a position of your choice and 
include the usr/bin directory at the unpacked location in your `PATH`. Note that
any clang package available for your Linux distribution won't work. One needs
a clang version with the Swift extensions created by Apple.

However, since the samples use bindgen, which requires libclang.so, which isn't 
included in the Swift distribution avaiable at swift.org, it is necessary to install 
the libclang-dev ressources as well as a package. On Ubuntu this is

`sudo apt install libclang-dev`

### Install cmake

To run everything with cmake one needs at least cmake 3.28. This isn't the standard
at least in Ubuntu, so it is better to install cmake by tarball from [cmake.org](https://cmake.org/download/).
Unpack the tarball anywhere and add its `bin` directory to your `PATH`.

### Install Google benchmark on Linux

Head over to the [Google Benchmark Repository](https://github.com/google/benchmark.git)
and install the benchmark library from source. Make sure to use clang and clang++ 
from the Swift tarball to build the library. Build the Release version of the
library and install it following the instructions in the README.md file.

### Install Zig
Nothing special here. Just follow the instructions on the [Zig website](https://ziglang.org).

## Run the demo

To run the demo using rust benchmarks the command is

macOS : `cargo bench`

Linux: `CC=clang CXX=clang++ cargo bench`

On Linux it is important to use clang from the Swift distribution. GCC won't work.

To run the c++ benchmarks the invocation is

```'sh
cmake -DCMAKE_C_COMPILER:FILEPATH=clang -DCMAKE_CXX_COMPILER:FILEPATH=clang++ -S . -B build -G "Ninja Multi-Config"

cmake --build build --config Release --target trap_bench

build/cpp/Release/trap_bench
```


# Caveats

This rep requires clang++ to build, but that doesn't do the trap_cp_dpp solution
no good: it is much slower then compiled by clang++  than the rust solution. But that 
is a flaw of clang++, not of the c++ solution. The c++ solution is as fast as the 
rust solution when compiled with g++.