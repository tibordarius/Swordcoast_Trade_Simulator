use std::env;
use std::process::{Command, ExitCode};

fn run(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .status()
        .unwrap_or_else(|error| panic!("failed to run {program}: {error}"))
        .success()
}

fn usage() {
    eprintln!(
        "usage: cargo xtask <check|test-tiny|test-v2|pack-validate <scenario-pack.json>>"
    );
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        usage();
        return ExitCode::from(2);
    };

    let ok = match command.as_str() {
        "check" => {
            run("cargo", &["fmt", "--all", "--", "--check"])
                && run(
                    "cargo",
                    &[
                        "clippy",
                        "--workspace",
                        "--all-targets",
                        "--",
                        "-D",
                        "warnings",
                    ],
                )
                && run("cargo", &["test", "--workspace", "--all-targets"])
        }
        "test-v2" => run("cargo", &["test", "-p", "sim-kernel-v2"]),
        "test-tiny" => {
            run("cargo", &["test", "-p", "sim-kernel-v2"])
                && run("cargo", &["test", "-p", "scenario-pack-v2"])
        }
        "pack-validate" => {
            let Some(path) = args.next() else {
                usage();
                return ExitCode::from(2);
            };
            run(
                "cargo",
                &[
                    "run",
                    "-p",
                    "scenario-pack-v2",
                    "--bin",
                    "scenario-pack-validate",
                    "--",
                    &path,
                ],
            )
        }
        _ => {
            usage();
            return ExitCode::from(2);
        }
    };

    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
