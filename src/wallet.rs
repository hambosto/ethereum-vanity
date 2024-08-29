use ethers::prelude::Signer;
use ethers::signers::{
    coins_bip39::{English, Mnemonic},
    MnemonicBuilder,
};
use ethers::types::Address;

pub struct Wallet {
    pub address: Address,
    pub phrase: String,
}

impl Wallet {
    pub fn new(mnemonic: Mnemonic<English>) -> Result<Self, Box<dyn std::error::Error>> {
        let phrase = mnemonic.to_phrase();
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(phrase.as_str())
            .build()?;
        Ok(Wallet {
            address: wallet.address(),
            phrase,
        })
    }
}
