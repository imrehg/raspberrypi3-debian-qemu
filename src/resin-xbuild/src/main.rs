#[macro_use]
extern crate log;

use std::env;
use std::process;
use std::process::Command;

fn cross_build_start() {

    match std::fs::remove_file("/bin/sh") {
        Ok(_) => {}
        Err(err) => {
            warn!("Error removing /bin/sh: {}", err);
        }
    }

    match std::os::unix::fs::symlink("/usr/bin/resin-xbuild", "/bin/sh") {
        Ok(_) => {}
        Err(err) => {
            warn!("Error with symlinking: {}", err);
        }
    }

}

fn cross_build_end() {

    match std::fs::remove_file("/bin/sh") {
        Ok(_) => {}
        Err(err) => {
            warn!("Error removing /bin/sh: {}", err);
        }
    }

    match std::os::unix::fs::symlink("/usr/bin/sh.real", "/bin/sh") {
        Ok(_) => {}
        Err(err) => {
            warn!("Error with symlinking: {}", err);
        }
    }
}

fn run_shell() {
    let args: Vec<String> = env::args().collect();

    let status = Command::new("/usr/bin/qemu-arm-static")
        .args(&["-0", "/bin/sh", ".bin/sh"])
        .args(args.skip(1))
        .status();
    return status.code();
}

fn bin_sh() {
    println!("Shell");
    let code = 0;
    cross_build_end();

    // 		if err := runShell(); err != nil {
    // 			code = 1
    // 			if exiterr, ok := err.(*exec.ExitError); ok {
    // 				if status, ok := exiterr.Sys().(syscall.WaitStatus); ok {
    // 					code = status.ExitStatus()
    // 				}
    // 			}
    // 		}


    cross_build_start();
    process::exit(code);
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    match args[0].as_ref() {
        "cross-build-start" => cross_build_start(),
        "cross-build-end" => cross_build_end(),
        "/bin/sh" => bin_sh(),
        _ => process::exit(1),
    }
}
