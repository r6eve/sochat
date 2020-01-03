sochat
======
[![Build Status][]][CI Results]

`sochat` is a chat program using TCP/UDP communication.

## Demo

![sochat-demo][]

## Installation

```console
$ git clone https://github.com/r6eve/sochat.git
$ cd sochat
$ cargo install
```

## Usage

```
Usage: sochat [options]

Options:
    -u, --username NAME set username
    -p, --port PORT     set port number (default: 8080)
    -h, --help          print this help menu
    -v, --version       print version
```

- Verbose Options

```console
$ RUST_LOG=warn sochat [options]
```

- More Verbose Options

```console
$ RUST_LOG=info sochat [options]
```

[Build Status]: https://github.com/r6eve/sochat/workflows/main/badge.svg
[CI Results]: https://github.com/r6eve/sochat/actions
[sochat-demo]: https://raw.githubusercontent.com/r6eve/screenshots/master/sochat/sochat.gif
