use clap::ArgMatches;

pub fn from_args(args: &ArgMatches) {
        if let Some(args) = args.subcommand_matches("test") {
            if args.is_present("port") {
                println!("present port parameter");
            } else {
                println!("no parameter");
            }
        }
}
