use systemstat::data::BatteryLife;
use systemstat::{Platform, System};

use getaddrs::InterfaceAddrs;

use env_logger;

use fastrand;

use ansi_rgb::{Background, Foreground};
use rgb::RGB8;

use anyhow::{Context, Result};

use std::cell::RefCell;
use std::ffi::OsString;
use std::io::{stdout, Write};
use std::ops::Range;
use std::process;
use std::rc::Rc;
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

fn display_network_stats(sys: &System, interface: &str) -> Result<()> {
    let network_stats = sys.network_stats(interface)?;
    println!(
        "recieved: {rec} transmitted: {trans}",
        rec = &network_stats.rx_bytes,
        trans = &network_stats.tx_bytes
    );
    Ok(())
}

///perform intensive computation on all available cores and leak as much memory as possible without getting process killed
fn empty_battery_faster() {
    leak_memory::leak_memory();
    //use all cpu cores
    //disk reads/writes
    //expensive syscalls
}

/// currently a basic memory leak from the rust docs that overflows the stack. might have to mess around with allocators to actually cause any sort of near-crash performance dips
mod leak_memory {
    use super::*;
    use crate::leak_memory::List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            //building up allocations
            let _hehe: Vec<usize> = Vec::with_capacity(1500000000);
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    pub fn leak_memory() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("a next item = {:?}", a.tail());
    }
}

fn run() -> Result<()> {
    let sys = System::new();
    let battery = sys.battery_life()?;
    let mut args = collect_args();
    let display_arg = String::from("-d");
    let empty_arg = String::from("-e");
    let net_arg = String::from("-n");
    let mut args: Vec<String> = args.into_iter().skip(1).collect();
    match args.pop() {
        Some(arg) => {
            if arg == display_arg {
                display_progress_bar(&battery)?;
            } else if arg == empty_arg {
                empty_battery_faster();
            } else if arg == net_arg {
                let mut addrs =
                    InterfaceAddrs::query_system().context("system has no network interface")?;
                let second_interface_name = addrs.nth(1).context("interface error, confusing")?.name;
                println!("{}", second_interface_name);
                loop {
                    display_network_stats(&System::new(), &second_interface_name)?;
                    for _i in 0..55 {
                        std::thread::sleep(FIFTY_MILLIS);
                    }
                }
            } else {
                eprintln!("Arguments supplied are not valid: ");
                dbg!(arg);
                eprintln!(
                    "Battery
                    A command for doing stuff with laptop batteries.

                    USAGE: battery [-d] [-e] [-n]
                    
                    -d: display battery bar
                    -e: empty battery faster
                    -n: display network stats
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
