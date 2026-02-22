use std::collections::HashSet;
use std::io;
use std::net::IpAddr;
use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::thread;

use anyhow::anyhow;
use clap::{Arg, ArgAction, Command, crate_description, crate_name, crate_version, value_parser};
use color_print::cprintln;
use rayon::ThreadPoolBuilder;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::display;
use crate::lookup;

fn build() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .long_about(crate_description!())
        .hide_possible_values(true)
        .args([
            Arg::new("hosts")
                .required(true)
                .num_args(1..)
                .value_name("HOSTNAME")
                .help("Hosts to lookup")
                .long_help("Hosts to lookup"),
            Arg::new("db")
                .short('d')
                .long("database")
                .required(true)
                .action(ArgAction::Append)
                .value_parser(value_parser!(PathBuf))
                .value_name("PATH")
                .help("Path(s) to GeoIP databases; can be used multiple times")
                .long_help("Path(s) to GeoIP databeses; can be used multiple times"),
            Arg::new("jobs")
                .short('j')
                .long("jobs")
                .required(false)
                .hide_default_value(true)
                .value_parser(value_parser!(usize))
                .value_name("N")
                .help("Number of parallel jobs (defaults to the number of CPUs)")
                .long_help("Number of parallel jobs (defaults to the number of CPUs)"),
            Arg::new("resolve-host")
                .short('n')
                .required(false)
                .action(ArgAction::SetFalse)
                .help("Do not resolve hostnames")
                .long_help("Do not resolve hostnames"),
            Arg::new("verify-db")
                .short('c')
                .long("no-verify")
                .action(ArgAction::SetFalse)
                .help("Do not verify database integrity before lookups")
                .long_help("Do not verify database integrity before lookups"),
        ])
}

pub fn run() -> anyhow::Result<()> {
    let args = build().get_matches();

    match env_logger::try_init() {
        Ok(_) => {}
        Err(e) => {
            // Just inform that logs won't be printed.
            eprintln!("error: Failed to initialize logger: {e}; Log messages won't be printed");
        }
    }

    // Build a pool of IPs to lookup
    let mut addrs = HashSet::new();
    // The `hosts` argument is required so it's okay to unwrap
    for host in args.get_many::<String>("hosts").unwrap() {
        if !args.get_flag("resolve-host") {
            let addr = host
                .parse::<IpAddr>()
                .map_err(|e| anyhow!("{host} is not a valid IP address: {e}"))?;
            addrs.insert(addr);
        } else {
            format!("{host}:0")
                .to_socket_addrs()
                .map_err(|e| anyhow!("Failed to resolve hostname {host}: {e}"))?
                .for_each(|addr| {
                    addrs.insert(addr.ip());
                });
        }
    }

    log::debug!("Queued {} address(es) to lookup", addrs.len());

    // As with `hosts`, at least one database must be specified, so it's okay
    // to unwrap
    let dbs: HashSet<PathBuf> =
        HashSet::from_iter(args.get_many::<PathBuf>("db").unwrap().cloned());

    log::debug!("Queued {} database path(s)", dbs.len());

    let jobs = args.get_one::<usize>("jobs").copied().unwrap_or_else(|| {
        match thread::available_parallelism() {
            Ok(n) => n.get(),
            // Rather a rare case
            Err(e) => {
                log::error!("Failed to retrieve the number of CPUs: {e}; Continuing with 1");
                1
            }
        }
    });

    // Open databases
    let lookuper = lookup::open(dbs.iter())?;

    log::debug!("Initializing worker pool with {jobs} workers");

    let workers = ThreadPoolBuilder::new()
        .num_threads(jobs)
        .build()
        .map_err(|e| anyhow!("Failed to build workers pool: {e}"))?;

    if args.get_flag("verify-db") {
        lookuper.verify_dbs()?;
    }

    // Lookup each address and print the results
    workers.install(|| {
        addrs.into_par_iter().for_each(|addr| {
            let result = lookuper.get(addr);

            // Lock so the output isn't a mess
            let mut stdout = io::stdout().lock();

            match result {
                Ok(result) => {
                    if result.is_empty() {
                        cprintln!("<s>No results found for <g>{addr}</></>");
                    } else {
                        match display::print_lookup_result(&mut stdout, &addr, &result) {
                            Ok(_) => {}
                            Err(e) => log::error!("error: Error printing to stdout: {e}"),
                        }
                    }
                }
                Err(e) => {
                    log::error!("{}: {}", addr, e);
                }
            }
        });
    });

    Ok(())
}
