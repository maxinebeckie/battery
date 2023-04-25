use systemstat::data::BatteryLife;
use systemstat::{Platform, System};

use fastrand;

use ansi_rgb::{Background, Foreground};
use rgb::RGB8;

use std::io::{stdout, Write};
use std::ops::Range;
use std::time::Duration;

use anyhow::Result;

const FIFTY_MILLIS: Duration = Duration::from_millis(50);
const HEX_RANGE: Range<u8> = 0..255;
const _VAR_RANGE: Range<u8> = 0..20;

fn main() {
    let sys = System::new();

    match sys.battery_life() {
        Ok(battery) => {
            display_progress_bar(&battery).unwrap();
            // print_battery_percent(&battery);
        }
        Err(e) => println!("Error: {}", e),
    }
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
