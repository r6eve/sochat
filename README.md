sochat
======
[![Build Status](https://travis-ci.org/r6eve/sochat.svg?branch=master)](https://travis-ci.org/r6eve/sochat)

`sochat` is a chat program using TCP/UDP communication.

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
