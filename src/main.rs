use systemstat::data::BatteryLife;
use systemstat::{Platform, System};

use env_logger;

use fastrand;

use ansi_rgb::{Background, Foreground};
use rgb::RGB8;

use anyhow::Result;

use std::ffi::OsString;
use std::io::{stdout, Write};
use std::ops::Range;
use std::process;
use std::time::Duration;

const FIFTY_MILLIS: Duration = Duration::from_millis(50);
const HEX_RANGE: Range<u8> = 0..255;
const _VAR_RANGE: Range<u8> = 0..20;

fn collect_args() -> Vec<String> {
    let args = std::env::args_os();
    let mut ret = Vec::new();
    for a in args {
        let strd = unsafe { std::mem::transmute::<OsString, String>(a) };
        ret.push(strd);
    }
    ret
}

///perform intensive computation on all available cores and leak as much memory as possible without getting process killed
fn empty_battery_faster() {
    todo!();
}

fn run() -> Result<()> {
    let sys = System::new();
    let battery = sys.battery_life()?;
    let mut args = collect_args();
    let display_arg = String::from("-d");
    let empty_arg = String::from("-e");
    let mut args: Vec<String> = args.into_iter().skip(1).collect();
    match args.pop() {
        Some(arg) => {
            if arg == display_arg {
                display_progress_bar(&battery)?;
            } else if arg == empty_arg {
                empty_battery_faster();
            } else {
                eprintln!("Arguments supplied are not valid: ");
                dbg!(arg);
                eprintln!("Battery
                    A command for doing stuff with laptop batteries.

                    USAGE: battery [-d] [-e]
                    
                    -d: display battery bar
                    -e: empty battery faster
                    "
                );
            }
        }
        None => display_progress_bar(&battery)?,
    }
    Ok(())
}

fn main() {
    env_logger::init();

    process::exit(match run() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("Fatal, RKO: {e}");
            1
        }
    });
}

fn _print_battery_percent(battery: &BatteryLife) {
    println!(
        "{}%, {}h{}m left",
        battery.remaining_capacity * 100.0,
        battery.remaining_time.as_secs() / 3600,
        battery.remaining_time.as_secs() % 60
    );
}

/// a - b but does not go negative
fn _diff_truncate(a: u8, b: u8) -> u8 {
    if a >= b {
        a - b
    } else {
        0
    }
}

fn random_color() -> RGB8 {
    let r = fastrand::u8(HEX_RANGE);
    let g = fastrand::u8(HEX_RANGE);
    let b = fastrand::u8(HEX_RANGE);
    RGB8::new(r, g, b)
}

fn display_progress_bar(battery: &BatteryLife) -> Result<()> {
    let battery_capacity = (battery.remaining_capacity * 100.0) as usize;
    print!("{}", "[".fg(random_color()).bg(random_color()));
    stdout().flush()?;
    for _i in 0..(battery_capacity / 2) {
        std::thread::sleep(FIFTY_MILLIS);
        print!("{}", "#".fg(random_color()).bg(random_color()));
        stdout().flush()?;
    }
    let rem_capacity = 100 - battery_capacity;
    if rem_capacity != 0 {
        for _i in 0..(rem_capacity / 2) {
            std::thread::sleep(FIFTY_MILLIS);
            print!("{}", "-".fg(random_color()).bg(random_color()));
            stdout().flush()?;
        }
    }
    std::thread::sleep(FIFTY_MILLIS);
    println!("{}", "]".fg(random_color()).bg(random_color()));
    Ok(())
}
