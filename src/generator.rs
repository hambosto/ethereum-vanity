use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::Arc;
use std::thread;

use ethers::signers::coins_bip39::{English, Mnemonic};
use rand::thread_rng;

use crate::wallet::Wallet;

pub struct Generator {
    rx: Receiver<Wallet>,
}

impl Generator {
    pub fn new(workers: usize, running: Arc<AtomicBool>) -> Self {
        let (tx, rx) = sync_channel(workers + 1);

        thread::spawn(move || generate_wallets(tx, running, workers));

        Generator { rx }
    }
}

impl Iterator for Generator {
    type Item = Wallet;

    fn next(&mut self) -> Option<Self::Item> {
        self.rx.recv().ok()
    }
}

fn generate_wallets(tx: SyncSender<Wallet>, running: Arc<AtomicBool>, workers: usize) {
    let (worker_tx, worker_rx) = sync_channel(workers);

    for id in 0..workers {
        worker_tx.send(id).unwrap();
    }

    let mnemonic_generator = MnemonicGeneator::new(running);

    for (worker, mnemonic) in worker_rx.iter().zip(mnemonic_generator) {
        let tx = tx.clone();
        let worker_tx = worker_tx.clone();

        thread::spawn(move || {
            if let Ok(wallet) = Wallet::new(mnemonic) {
                tx.send(wallet).unwrap();
            }
            // worker_tx gets dropped when mnemonic_generator is exhausted
            let _ = worker_tx.send(worker);
        });
    }
}

struct MnemonicGeneator {
    running: Arc<AtomicBool>,
}

impl MnemonicGeneator {
    fn new(running: Arc<AtomicBool>) -> Self {
        MnemonicGeneator { running }
    }
}

impl Iterator for MnemonicGeneator {
    type Item = Mnemonic<English>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.running.load(Ordering::Relaxed) {
            return None;
        }

        match Mnemonic::new_with_count(&mut thread_rng(), 12) {
            Ok(mnemonic) => Some(mnemonic),
            Err(e) => {
                eprintln!("Error generating mnemonic: {:?}", e);
                None
            }
        }
    }
}
