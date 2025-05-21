# STEGA

[![crates.io](https://img.shields.io/crates/v/stega)](https://crates.io/crates/stega)
[![license](https://img.shields.io/crates/l/stega)](https://crates.io/crates/stega)

A simple tool and library to conceal and reveal UTF-8 encoded data within PNG images.

## Disclaimer

This tool and/or library does not guarantee the confidentiality of the data
concealed in the resulting carrier images. Use this crate under your own risk.

## Library

Consult the [documentation](https://docs.rs/stega) for more information.

## Installation

You must install [Rust](https://www.rust-lang.org/tools/install) on
your system for any of the next installation methods to work:

### From crates.io

```shell
$ cargo install stega
```

### From GitHub

```shell
$ cargo install --git https://github.com/septum/stega
```

### From source

```shell
$ git clone https://github.com/septum/stega.git
$ cd stega
$ cargo install --path .
```

## Usage

STEGA has two subcommands available:

### Conceal

Using this subcommand will conceal UTF-8 encoded data into a PNG image:

```shell
$ stega conceal <IMAGE_PATH> [DATA]
```

On success, it will save the data-concealed PNG image in the same location as
the the original image with the filename `carrier.png`, overwriting an already
existing file.

#### Arguments

- `<IMAGE_PATH>`: Valid PNG image path
- `[DATA]`: Optional UTF-8 encoded text argument (with a fallback through STDIN)

#### Examples

```shell
$ stega conceal ferris.png "🦀"
```

```shell
$ cat hello_world.txt | stega conceal image.png
```

### Reveal

Using this subcommand will reveal UTF-8 encoded data concealed in a PNG image:

```shell
$ stega reveal <IMAGE_PATH>
```

On success, it will print the data to STDOUT.

#### Arguments

- `<IMAGE_PATH>`: Valid PNG image path

#### Examples

```shell
$ stega reveal carrier.png
```

```shell
$ stega reveal carrier.png > data.txt
```

## License

This project is dual-licensed under either:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Contributing

Contributions are very much welcome! If you find a bug, want a new feature or
see an improvement opportunity, please open an issue or submit a PR.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
