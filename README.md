# AWA5.RS

An [AWA5.0](https://github.com/TempTempai/AWA5.0) CLI tool written in Rust (btw)

Runs as an AWA5.0 interpreter for Awatisms with file extension `.awasm` and Awatalk with file extension `.awa`

Can also run as an assembler for Awatisms and Awatalk to object files, and assembled object files with extension `.o` can be run by the interpreter

## Installation

To install or build from source, you will need to have `rust` or `rustup` installed

```bash
# using Arch Linux's package manager
$ sudo pacman -S rust # or rustup
```

If you installed `rustup`, you need to install a toolchain

```bash
$ rustup toolchain install latest
```

### With Cargo

You can install from crates.io using cargo

```bash
$ cargo install awa5_rs
```

And then run from the command line

```bash
$ awa5_rs --help
```

### From source

Or clone this repository and build it from source using cargo

```bash
$ git clone https://github.com/liraymond04/awa5_rs.git
$ cd awa5_rs
$ cargo build
$ ./target/debug/awa5_rs # you can also build and run with `cargo run`, and you can pass flags with `cargo run -- --help` for example
```

## Usage

```
Usage: awa5_rs [OPTIONS] [input]

Arguments:
  [input]  File to interpret or convert

Options:
  -o, --output <output>  Output to file with new format .awasm .awa .o
  -s, --string <string>  String to interpret or convert
      --awasm            Parse string as awasm
      --awa              Parse string as awatalk
  -p, --path <path>      Search paths separated by ';' for shared libraries
  -h, --help             Print help
  -V, --version          Print version
```
