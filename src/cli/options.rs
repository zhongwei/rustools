use clap::ArgMatches;
use super::super::server;

pub fn from_args(args: &ArgMatches) {
        if let Some(args) = args.subcommand_matches("server") {
            if args.is_present("port") {
                println!("aaaaaaaaaaaaaaaaaaaaaaaaaaa");
            server::start();
            } else {
                println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
            server::start();
            }
        }
}
