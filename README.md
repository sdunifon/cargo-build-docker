# cargo-docker-build

This is a fork of [cargo-docker](https://github.com/DenisKolodin/cargo-docker)

A cargo subcommand to build Rust code using the standard rust docker containers

## Install

To install `cargo-docker-build` you should type the following:
```sh
$ cargo install cargo-docker-build
```


## Usage

To use `cargo-docker-build` you should type the following:

```sh
$ cargo docker [--image rust:1.33.0]
```

Specifying the image is optional, if it's omitted it will use the rust:1.33.0 image

At the moment this runs the following command inside the container

```bash
cargo build --release --lib
```
This will be updated in future to allow arbitrary cargo commands to be passed through