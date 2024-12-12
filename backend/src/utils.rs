use std::str::FromStr;

use bech32::{hrp, Bech32, Hrp};
use hdwallet::{DefaultKeyChain, ExtendedPrivKey, KeyChain};
use secp256k1::{ hashes::{ripemd160, sha256, Hash}, PublicKey, Secp256k1};

pub fn pk_to_addr(pk:&PublicKey,prefix:&str) -> Option<String> {
    let pk = pk.serialize();
    let hpk = sha256::Hash::hash(&pk);
    let hpk = ripemd160::Hash::hash(hpk.as_byte_array());
    let hpk = hpk.as_byte_array();
    Some(bech32::encode::<Bech32>(Hrp::parse(prefix).ok()?, hpk).ok()?.to_string())
}

pub fn key_pair_from_mnemonic(mnemonic:&str) -> Option<secp256k1::Keypair> {
    let ctx = Secp256k1::new();
    let seed = bip39::Mnemonic::from_str(mnemonic).ok()?;
    let seed = seed.to_seed("");

    let x = ExtendedPrivKey::with_seed(&seed).ok()?;
    let key_chain = DefaultKeyChain::new(x);
    let pk = key_chain.derive_private_key("m/44'/118'/0'/0/0".into()).ok()?;
    let x = pk.0.private_key.secret_bytes();

    Some(secp256k1::Keypair::from_seckey_slice(&ctx, &x).unwrap())
}


pub fn bech_to_bech(addr:&str, prefix:&str) -> Option<String>{
    let (_, d) = bech32::decode(addr).ok()?;
    bech32::encode::<Bech32>(hrp::Hrp::parse(prefix).ok()?, &d).ok()
}

#[test]
fn test(){
    let m = "clock post desk civil pottery foster expand merit dash seminar song memory figure uniform spice circle try happy obvious trash crime hybrid hood cushion";
    let x = key_pair_from_mnemonic(m).unwrap();
    println!("{:?}", pk_to_addr(&x.public_key(), "cosmos"));
    // pk_to_addr()
}
 