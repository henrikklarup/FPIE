![Rust](https://github.com/hklsiteimprove/FPIE/workflows/Rust/badge.svg?branch=master)
# File Packer with Include and Exclude support
FPIE can help you create tar files from the specified includefile.

## Installation
Download the binaries which fits your platform from [Latest release](https://github.com/hklsiteimprove/FPIE/releases/latest).

## Usage

### Includefile syntax
See [Includefile syntax readme](Docs/SYNTAX.md)

### Small context size for docker
The FPIE project is mainly for use together with docker, for building small docker contexts to send into the docker build:

```bash
    fpie -c . -i includefile -o - | docker build -f ./dockerfile -
```

You can also use docker instead of the binary:
```bash
    docker run -v <contextdir>:/data hklsiteimprove/fpie -c data -i data/includefile | docker build -f ./dockerfile -
```

## Testing 
Tests are automatically run as part of the CI pipeline, and reported back to any pull-request.
To manually run tests go to the `fpie` folder and run:
```bash
    cargo test
```

## Local development
The source code is located inside the `fpie` folder.

### Development requirements
- Rust 1.41 (stable)

### How to run
To make sure everything compiles run:
```bash
    cargo run -- --help
```
This should print the help menu from `fpie` cli.

## Contributing
See [Contribution readme](Docs/CONTRIBUTE.md)
