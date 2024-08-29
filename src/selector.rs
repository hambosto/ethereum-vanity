use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::cli::Cli;
use crate::wallet::Wallet;

use ethers::types::Address;

pub fn create_selector<'a>(
    args: &'a Cli,
) -> Result<Box<dyn Fn(&Wallet) -> bool + Send + 'a>, io::Error> {
    if let Some(filename) = &args.input {
        Ok(Box::new(select_from_file(filename)?))
    } else if let Some(suffix) = &args.suffix {
        Ok(Box::new(move |wallet: &Wallet| {
            addr_has_suffix(&wallet.address, suffix)
        }))
    } else if let Some(prefix) = &args.prefix {
        Ok(Box::new(move |wallet: &Wallet| {
            addr_has_prefix(&wallet.address, prefix)
        }))
    } else {
        Ok(Box::new(|_| true))
    }
}

fn select_from_file<P: AsRef<Path>>(filename: P) -> io::Result<impl Fn(&Wallet) -> bool> {
    let prefixes: HashSet<String> = load_prefixes(filename)?;
    Ok(move |wallet: &Wallet| addr_matches_map(&prefixes, &wallet.address))
}

fn load_prefixes<P: AsRef<Path>>(filename: P) -> io::Result<HashSet<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut prefixes = HashSet::new();

    for line in reader.lines() {
        let word = line?;
        if word.len() != 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Input file must contain only 4 letter words: {:?} is {} characters",
                    word,
                    word.len()
                ),
            ));
        }
        prefixes.insert(word);
    }

    eprintln!("Read {} prefixes", prefixes.len());
    Ok(prefixes)
}

fn addr_matches_map(prefixes: &HashSet<String>, addr: &Address) -> bool {
    let address = format!("{:?}", addr);
    (prefixes.contains(&address[2..6]) && prefixes.contains(&address[38..42]))
        || is_all_same_char(&address[2..6]) && is_all_same_char(&address[38..42])
}

fn is_all_same_char(s: &str) -> bool {
    s.chars().all(|c| c == s.chars().next().unwrap())
}

fn addr_has_prefix(addr: &Address, prefix: &str) -> bool {
    let address = format!("{:?}", addr);
    address[2..].starts_with(prefix)
}

fn addr_has_suffix(addr: &Address, suffix: &str) -> bool {
    let address = format!("{:?}", addr);
    address[..42].ends_with(suffix)
}
