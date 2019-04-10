//! A cargo subcommand for building fortanix projects inside docker and getting the results back.

#[macro_use]
extern crate clap;

use std::env;
use std::process::Command;

use clap::{App, AppSettings, SubCommand};

fn main() {
    let app = App::new("cargo-count")
        .bin_name("cargo")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("fortanix")
            .version(concat!("v", crate_version!()))
            .author("Steven Skone <steven@skone.net>")
            .about("Build Rust code targetting fortanix in docker"));

    let m = app.get_matches();

    if m.subcommand_matches("fortanix").is_some() {
        let p = env::current_dir().unwrap();
        let source_folder = format!("{}:/source", p.display());
        let target_folder = format!("{}/{}:/source/target", p.display(), "target");

        let mut command = Command::new("docker")
            // Run new container
            .arg("run")
            // Allocate pseudo-tty
            .arg("-t")
            // Remove container after using
            .arg("--rm")
            // Attach virtual volume with sources
            .args(&["-v", &source_folder])
            .args(&["-v", &target_folder])
            .arg("sskone/rust-builder-fortanix")
            .args(&["build", "--all", "--release", "--target", "x86_64-fortanix-unknown-sgx"])
            .spawn()
            .expect("failed to execute docker");

        command.wait().expect("docker failed");
    }
}
