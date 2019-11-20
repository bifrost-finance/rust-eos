#![allow(dead_code)]
use crate::Checksum256;

fn make_canonical_left(val: &Checksum256) -> Checksum256 {
    let mut canonical_l: Checksum256 = *val;
    canonical_l.set_hash0(canonical_l.hash0() & 0xFFFFFFFFFFFFFF7Fu64);
    canonical_l
}

fn  make_canonical_right(val: &Checksum256) -> Checksum256 {
    let mut canonical_r: Checksum256 = *val;
    canonical_r.set_hash0(canonical_r.hash0() | 0x0000000000000080u64);
    canonical_r
}

fn is_canonical_left(val: &Checksum256) -> bool {
    (val.hash0() & 0x0000000000000080u64) == 0
}

fn is_canonical_right(val: &Checksum256) -> bool {
    (val.hash0() & 0x0000000000000080u64) != 0
}

pub fn make_canonical_pair(l: &Checksum256, r: &Checksum256) -> (Checksum256, Checksum256) {
    (make_canonical_left(l), make_canonical_right(r))
}

pub fn merkle(ids: Vec<Checksum256>) -> crate::Result<Checksum256> {
    let mut ids = ids;

    if 0 == ids.len()  {
        return Ok(Default::default());
    }

    while ids.len() > 1  {
        if ids.len() % 2 != 0 {
            ids.push(ids[ids.len() - 1]);
        }

        for i in 0..(ids.len() / 2) {
            ids[i] = Checksum256::hash(make_canonical_pair(&ids[2 * i], &ids[(2 * i) + 1]))?;
        }

        ids.resize(ids.len() / 2, Default::default());
    }

    Ok(ids[0])
}

#[cfg(test)]
mod tests {
    use crate::{Checksum256, TransactionReceipt};
    use super::*;
    use std::{
        error::Error,
        fs::File,
        io::Read,
        path::Path,
    };

    fn read_json_from_file(json_name: impl AsRef<str>) -> Result<String, Box<dyn Error>> {
        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/test_data/")).join(json_name.as_ref());
        let mut file = File::open(path)?;
        let mut json_str = String::new();
        file.read_to_string(&mut json_str)?;
        Ok(json_str)
    }

    #[test]
    fn merkle_zero_id_should_work() {
        let ids: Vec<Checksum256> = vec![];
        let result = merkle(ids);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Checksum256::default());
    }

    #[test]
    fn merkle_one_id_should_work() {
        let ids: Vec<Checksum256> = vec![[1u8; 32].into()];
        let result = merkle(ids);
        assert!(result.is_ok());
        let expect: Checksum256 = [1u8; 32].into();
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn merkle_should_work() {
        let ids: Vec<Checksum256> = vec![
            [0u8; 32].into(),
            [0u8; 32].into(),
            [1u8; 32].into(),
            [1u8; 32].into(),
        ];

        let result = merkle(ids);
        let expect: Checksum256 = [0xf1, 0x4f, 0xfa, 0x19, 0xa8, 0xc5, 0xe2, 0xa3, 0xf0, 0x42, 0x66, 0xde, 0x1b, 0xb2, 0x8c, 0x5b, 0xc0, 0x29, 0xa7, 0xe3, 0xf8, 0x87, 0x1c, 0x23, 0xd5, 0x9e, 0x15, 0x74, 0x93, 0x5e, 0x40, 0x8c].into();
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn verify_transaction_mroot() {
        let json = "transactions.json";
        let trxs_str = read_json_from_file(json);
        assert!(trxs_str.is_ok());
        let trxs: Result<Vec<TransactionReceipt>, _> = serde_json::from_str(&trxs_str.unwrap());
        assert!(trxs.is_ok());
        let trxs = trxs.unwrap();

        let mut trxs_digests: Vec<Checksum256> = Vec::with_capacity(trxs.len());
        for trx in trxs {
            trxs_digests.push(trx.digest().unwrap());
        }
        let merkle_root = merkle(trxs_digests.clone()).unwrap();
        // the correct transaction_mroot is right in file many_transactions_in_block.json
        assert_eq!(merkle_root.to_string(), "ba5b2ff707951223e948a6a684a8abecd26391f4ee62ed58b1477970c43886df");
    }

    #[test]
    fn verify_invalid_transaction_mroot() {
        let json = "invalid_transactions.json";
        let trxs_str = read_json_from_file(json);
        assert!(trxs_str.is_ok());
        let trxs: Result<Vec<TransactionReceipt>, _> = serde_json::from_str(&trxs_str.unwrap());
        assert!(trxs.is_ok());
        let trxs = trxs.unwrap();

        let mut trxs_digests: Vec<Checksum256> = Vec::with_capacity(trxs.len());
        for trx in trxs {
            trxs_digests.push(trx.digest().unwrap());
        }
        let merkle_root = merkle(trxs_digests.clone()).unwrap();
        // the correct transaction_mroot is right in file many_transactions_in_block.json
        // change a field net_usage_words from 0 to 42 in file invalid_transactions.json
        assert_ne!(merkle_root.to_string(), "ba5b2ff707951223e948a6a684a8abecd26391f4ee62ed58b1477970c43886df");
    }
}
