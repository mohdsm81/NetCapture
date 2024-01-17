use clap::{arg, builder, command};
use pcap;

use pcap::Capture;
use std::env;
use std::{process::ExitCode, u8};

/**
 Formal argument that the program can recognize
*/
struct FormalArgs {}

/**
 User provided command line arguments, and that are parsed successfully
*/
struct UserArgs {}

struct ProgramContext {}

fn parse_args(args: Vec<&str>, formal_args: FormalArgs) -> UserArgs {}

fn prepare(user_args: &UserArgs) -> ProgramContext {}

fn perform(program_context: &ProgramContext) -> ExitCode {}

fn main() {
    // chain function calls here.
    // Within each function, handel errors, exit code related to their function
}
