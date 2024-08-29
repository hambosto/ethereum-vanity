mod cli;
mod generator;
mod selector;
mod wallet;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use structopt::StructOpt;

use crate::cli::Cli;
use crate::generator::Generator;
use crate::selector::create_selector;
use crate::wallet::Wallet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::from_args();
    let workers = if args.workers < 1 {
        num_cpus::get() - 1
    } else {
        args.workers
    };

    let running = Arc::new(AtomicBool::new(true));
    setup_ctrl_c_handler(running.clone());

    let selector = create_selector(&args)?;
    let generator = Generator::new(workers, running.clone());

    eprintln!("Starting {} worker threads", workers);
    process_wallets(generator, selector)?;

    Ok(())
}

fn setup_ctrl_c_handler(running: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        eprintln!("Stopping search");
        running.store(false, Ordering::Relaxed);
    })
    .expect("Could not set Ctrl-C handler");
}

fn process_wallets<'a>(
    generator: Generator,
    selector: Box<dyn Fn(&Wallet) -> bool + Send + 'a>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut count = 0u32;
    let mut start = Instant::now();

    for wallet in generator {
        count += 1;
        if count % 1000 == 0 {
            eprint!(".");
        }
        if selector(&wallet) {
            print_match(&wallet, count, start.elapsed());
            count = 0;
            start = Instant::now();
        }
    }

    print_final_stats(count, start.elapsed());
    Ok(())
}

fn print_match(wallet: &Wallet, count: u32, duration: std::time::Duration) {
    eprintln!();
    eprintln!(
        "{} wallets since last match; {:.2} wallets per second checked",
        count,
        (count as f64) / duration.as_secs_f64()
    );
    println!("{}: {}", wallet.address, wallet.phrase);
}

fn print_final_stats(count: u32, duration: std::time::Duration) {
    eprintln!(
        "Workers terminated; {:2} wallets per second checked",
        (count as f64) / duration.as_secs_f64()
    );
}
