use clap::{arg, builder, command};
use pcap;

use pcap::Capture;
use std::env;
use std::process::exit;
use std::{process::ExitCode, u8};

/// Formal arguments the program can recognize
struct FormalArgs {}

/// User provided command line arguments, and that are parsed successfully
struct UserArgs {}

/// The entire program context is in one struct. Easier to manage, but may cause
/// some borrow-checker and/or concurrency trouble (we will see later).
struct ProgramContext {}

/// Parses, and validates, user provided CLI arguments then returns the
/// structured arguments `UserArgs`.
/// #Arguments
/// * `args` - the user provided arguments strings.
/// * `formal_args` - program defined arguments to validate against
fn parse_args(args: Vec<&str>, formal_args: FormalArgs) -> UserArgs {}

/// Prepares the context `ProgramContext` of the program based on parsed and
/// validated user provided CLI arguments and the environemnt.
/// #Arguments
/// * `user_args` - the parsed and validated user provided CLI arguments
fn prepare(user_args: &UserArgs) -> ProgramContext {}

/// Performs the operations according to the program context and user provided
/// CLI args.
/// #Arguments
/// * `program_context` - the program context to process
fn perform(program_context: &ProgramContext) -> ExitCode {}

fn main() {
    // chain function calls here.
    // Within each function, handel errors, exit code related to their function

    exit(ExitCode::SUCCESS);
}
