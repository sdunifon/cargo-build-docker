//! A cargo subcommand for building rust projects inside docker and getting the results back.

#![crate_name = "cargo_build_docker"]
#![crate_type = "bin"]

#[macro_use]
extern crate clap;

use std::env;
use std::process::Command;

use clap::{App, AppSettings, SubCommand, Arg};

fn main() {
    let app = App::new("cargo-build-docker")
        .bin_name("cargo")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("build-docker")
            .version(concat!("v", crate_version!()))
            .author("Steven Skone <steven@skone.net>")
            .about("Build Rust code in docker")
            .arg(Arg::with_name("image")
                .short("i")
                .long("image")
                .value_name("image")
                .default_value("rust:1.33.0")
                .help("Image to use for building")
                .takes_value(true)));

    //modify this so that we can pass through the args from the command line

    let m = app.get_matches();

    if let Some(matches) = m.subcommand_matches("build-docker") {


        let p = env::current_dir().unwrap();

        let image = matches.value_of("image").unwrap();

        let mut command = Command::new("docker")
            // Run new container
            .arg("run")
            //remove container after using
            .arg("--rm")
            //set user and group
            //.arg("--user")
            //.arg(r#""$(id -u)":"$(id -g)"#)
            // Allocate pseudo-tty
            .arg("-t")
            // Attach virtual volume with sources
            .args(&["-v", &format!("{}:/usr/src/myapp", p.display())])
            .args(&["-w", "/usr/src/myapp"])
            .arg(image)
            .args(&["cargo", "build", "--release", "--lib"])
            .spawn()
            .expect("failed to execute docker");

        command.wait().expect("docker failed");
    }
}
