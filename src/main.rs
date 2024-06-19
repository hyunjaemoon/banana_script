extern crate winapi;

mod enum_windows_proc;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 0)]
    hours: u32,

    #[arg(short, long, default_value_t = 0)]
    minutes: u32,

    #[arg(short, long, default_value_t = 0)]
    seconds: u32,

    #[arg(short, long, default_value_t = 2000)]
    delay_time: u64,

    #[arg(long)]
    infinite: bool,
}

fn main() {
    let args = Args::parse();

    let (hours, minutes, seconds, delay_time, is_infinite) =
        (args.hours, args.minutes, args.seconds, args.delay_time, args.infinite);

    enum_windows_proc::run(hours, minutes, seconds, delay_time, is_infinite);
}
