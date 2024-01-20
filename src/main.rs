use clap::{command, Arg, Command};
use pcap::*;
use std::collections::HashMap;
use std::{
    env,
    net::{IpAddr, Ipv4Addr},
    process::exit,
    u16,
};

/// User provided command line arguments, and that are parsed successfully
struct CLIArgs {
    port: u16,
    address: IpAddr,
}

impl CLIArgs {
    pub fn parse(&self) -> CLIArgs {
        let matches = command!()
            .about(
                "A CLI utility to listen to a connected or a specified network device, collecting packets going in and out of the interface.",
            )
            .author("Mohammed S. Al-Mahfoudh AND Devon Stewart")
            .arg(
                Arg::new("port_number")
                    .short('p')
                    .long("port")
                    .required(true)
                    .help("specify a port to listen on for incoming/outgoing packets")
                    .aliases(["port-no", "port-num", "l", "local-port"])
                    .default_value("80"),
            )
            .arg(
                Arg::new("host")
                    .short('h')
                    .long("host")
                    .required(false)
                    .aliases(["host-address", "address"])
                    .default_value("localhost")
                    .help("The IP v4/v6 address of the host to listen to."),
            ).get_matches();
        //TODO just to shut the compiler up
        CLIArgs {
            port: 80,
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        }
    }

    // TODO implement the validation logic
    fn validate(&self, cmd: Command) -> bool {
        // TODO implement validate method
        return false;
    }
    fn store_to_context(&self, context: ProgramContext) -> ProgramContext {
        // TODO implement store_to_context method
        return context;
    }
}

/// The entire program context is in one struct. Easier to manage, but may cause
/// some borrow-checker and/or concurrency trouble (we will see later).
struct ProgramContext<'a> {
    /// validated command line arguments
    cli_args: CLIArgs,
    /// collected packets per channel, each channel is marked by an IP Address
    /// (destination/source -- ask Devon later)
    collected_packets: HashMap<IpAddr, Vec<Packet<'a>>>,
}

/// Parses, and validates, user provided CLI arguments then updates the
/// structured arguments `CLIArgs`.
/// #Arguments
/// * `args` - the user provided arguments strings.
/// * `formal_args` - program defined arguments to validate against
fn parse_args(args: Vec<&str>, mut cli_args: &CLIArgs) {}

/// Prepares the context `ProgramContext` of the program based on parsed and
/// validated user provided CLI arguments and the environemnt.
/// #Arguments
/// * `user_args` - the parsed and validated user provided CLI arguments
fn prepare(user_args: &CLIArgs) -> ProgramContext {
    //TODO just to shut the compiler up
    ProgramContext {
        cli_args: CLIArgs {
            port: 80,
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        },
        collected_packets: HashMap::new(),
    }
}

/// Performs the operations according to the program context, which includes
/// user provided CLI args.
/// #Arguments
/// * `program_context` - the program context to process
fn perform(program_context: &ProgramContext) -> u32 {
    return 0;
}

fn main() {
    // chain function calls here.
    // Within each function, handel errors, exit code related to their function

    let args = CLIArgs {
        port: 80,
        address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
    }
    .parse();

    exit(0);
}
