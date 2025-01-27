#![no_std]
#![feature(error_in_core)]
extern crate alloc;
extern crate core;

#[cfg(test)]
#[macro_use]
extern crate std;

use crate::errors::Result;
use alloc::string::String;

use ur_registry::pb::protoc;

mod address;
pub mod errors;
mod pb;
mod transaction;
mod utils;

pub use crate::address::get_address;
pub use crate::transaction::parser::{DetailTx, OverviewTx, ParsedTx, TxParser};
use crate::transaction::wrapped_tron::WrappedTron;
use app_utils::keystone;
use transaction::checker::TxChecker;
use transaction::signer::Signer;

pub fn sign_raw_tx(
    raw_tx: protoc::Payload,
    context: keystone::ParseContext,
    seed: &[u8],
) -> Result<(String, String)> {
    let tx = WrappedTron::from_payload(raw_tx, &context)?;
    tx.sign(seed)
}

pub fn parse_raw_tx(raw_tx: protoc::Payload, context: keystone::ParseContext) -> Result<ParsedTx> {
    let tx_data = WrappedTron::from_payload(raw_tx, &context)?;
    tx_data.parse()
}

pub fn check_raw_tx(raw_tx: protoc::Payload, context: keystone::ParseContext) -> Result<()> {
    let tx_data = WrappedTron::from_payload(raw_tx, &context)?;
    tx_data.check(&context)
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec::Vec;
    use core::str::FromStr;
    use hex::FromHex;
    use ur_registry::pb::protobuf_parser::{parse_protobuf, unzip};
    use ur_registry::pb::protoc::{Base, Payload};
    pub fn prepare_parse_context(pubkey_str: &str) -> keystone::ParseContext {
        let master_fingerprint = bitcoin::bip32::Fingerprint::from_str("73c5da0a").unwrap();
        let extended_pubkey = bitcoin::bip32::Xpub::from_str(pubkey_str).unwrap();
        keystone::ParseContext::new(master_fingerprint, extended_pubkey)
    }

    pub fn prepare_payload(hex: &str) -> Payload {
        let bytes = Vec::from_hex(hex).unwrap();
        let unzip_data = unzip(bytes.clone()).unwrap();
        let base: Base = parse_protobuf(unzip_data).unwrap();
        base.data.unwrap()
    }
}
