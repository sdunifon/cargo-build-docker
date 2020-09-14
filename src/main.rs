//! A cargo subcommand for building rust projects inside docker and getting the results back.

#![crate_name = "cargo_build_docker"]
#![crate_type = "bin"]

#[macro_use]
extern crate clap;

use std::env;
use std::process::Command;

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let app = App::new("cargo-build-docker")
        .bin_name("cargo")
        .setting(AppSettings::TrailingVarArg)
        .subcommand(SubCommand::with_name("build-docker")
            .version(concat!("v", crate_version!()))
            .author("Steven Skone <steven@skone.net>")
            .about("Build Rust code in docker")
            .arg(Arg::with_name("image")
                .short("i")
                .long("image")
                .value_name("image")
                .default_value("rust:1.33.0")
                .help("Image to use for guilding")
                .takes_value(true))
            .arg(Arg::with_name("pass through args")
                .help("Any barguments you wish to pass to the binary being profiled.")
                .last(true)
                .multiple(true)
            ));

    //modify this so that we can pass through the args from the command line

    let m = app.get_matches();

    if let Some(matches) = m.subcommand_matches("build-docker") {
        let p = env::current_dir().unwrap();

        let image = matches.value_of("image").unwrap();
        let pass_through = matches.values_of("pass through args").unwrap();

        let mut docker_build_command = Command::new("docker")
            // Run new container
            .arg("build")
            //remove container after using
            .arg("--rm")
            //set user and group
            //.arg("--user")
            //.arg(r#""$(id -u)":"$(id -g)"#)
            // Allocate pseudo-tty
            .arg("-t")
            .arg("tmp-rust")
            .arg(".")
            .spawn()
            .expect("failed to execute docker");

        docker_build_command.wait().expect("docker failed");

        let mut docker_run_command = Command::new("docker")
            .arg("run")
            // Attach virtual volume with sources
            .args(&["-v", &format!("{}:/usr/src/myapp", p.display())])
            .args(&["-w", "/usr/src/myapp"])
            .arg("-t")
            .arg("tmp-rust")
            // .arg(image)
            //.args(&["cargo", "build", "--release", "--lib"])
            .arg("cargo")
            .args(pass_through)
            .spawn()
            .expect("failed to execute docker");

        docker_run_command.wait().expect("docker failed");
    }
}
