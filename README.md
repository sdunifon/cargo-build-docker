# cargo-docker-builder

This is a fork of [cargo-docker](https://github.com/DenisKolodin/cargo-docker)

A cargo subcommand to build Rust code using the standard rust docker containers

## Install

To install `cargo-docker-builder` you should type the following:
```bash
$ cargo install cargo-docker-builder
```


## Usage

To use `cargo-docker-builder` you should type the following:

```bash
$ cargo build-docker [--image rust:1.33.0] -- <commands to send to docker>
```

Specifying the image is optional, if it's omitted it will use the rust:1.33.0 image

An example is 
```bash
cargo build-docker -- --release --lib
```


## Build from source

If you want to modify this example clone the repo

```bash
git clone git@github.com:skone/cargo-build-docker.git
```

Make the changes you want to make and then install your newly develop subcommand
Ensure that you're in the root of the cargo-build-docker repo and then run

```bash
cargo install --path .
```


