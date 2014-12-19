#![feature(phase)]

extern crate docopt;
#[phase(plugin)]
extern crate docopt_macros;
extern crate serialize;
extern crate curiosity;

use curiosity::Stats;

docopt!(Args deriving Show, "
Usage:
    curiosity -h
    curiosity --version
    curiosity [options] <crate-entry>

Options:
    -h, --help          Show this message and exit.
    --version           Show this program version and exit.
", arg_crate_entry: Option<String>)

fn print_stats(stats: Stats) {
    for metric in stats.metrics.into_iter() {
        println!("{}: {}", metric.name, metric.value);
    }
}

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("curiosity v{}", curiosity::version());
        return;
    }

    if let Some(filename) = args.arg_crate_entry {
        let path = Path::new(filename);

        if let Some(stats) = curiosity::parse(&path) {
            print_stats(stats);
        }
    }
}
