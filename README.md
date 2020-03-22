![Rust](https://github.com/hklsiteimprove/FPIE/workflows/Rust/badge.svg?branch=master)
# File Packer with Include and Exclude support
FPIE can help you create tar files from the specified includefile.

# Small context size for docker
The FPIE project is mainly for use together with docker, for building small docker contexts to send into the docker build:

```bash
    fpie -c . -i includefile -o - | docker build -f ./dockerfile -
```
