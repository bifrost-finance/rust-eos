#![allow(dead_code)]
use alloc::vec::Vec;
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

pub fn get_proof(position: usize, ids: Vec<Checksum256>) -> crate::Result<Vec<Checksum256>> {
    let mut ids = ids;
    let mut position = position;
    let mut paths: Vec<Checksum256> = Vec::new();
    let is_right_node = |i| i % 2 == 1;

    if 0 == ids.len()  {
        return Ok(Default::default());
    }

    while ids.len() > 1  {
        if ids.len() % 2 != 0 {
            ids.push(ids[ids.len() - 1]);
        }

        if is_right_node(position) {
            paths.push(make_canonical_left(&ids[position - 1]));
        } else {
            paths.push(make_canonical_right(&ids[position + 1]));
        }
        position /= 2;

        for i in 0..(ids.len() / 2) {
            ids[i] = Checksum256::hash(make_canonical_pair(&ids[2 * i], &ids[(2 * i) + 1]))?;
        }

        ids.resize(ids.len() / 2, Default::default());
    }

    Ok(paths)
}

pub fn verify_proof(paths: &Vec<Checksum256>, leaf: Checksum256, expected_root: Checksum256) -> bool {
    let mut current: Checksum256 = leaf;
    let mut left: Checksum256;
    let mut right: Checksum256;

    for path in paths.iter() {
        if is_canonical_right(&path) {
            left = current;
            right = *path;
        } else {
            left = *path;
            right = current;
        }
        left = make_canonical_left(&left);
        right = make_canonical_right(&right);

        current = Checksum256::hash(make_canonical_pair(&left, &right)).unwrap();
    }

    current == expected_root
}

#[cfg(test)]
mod tests {
    use crate::{Checksum256, Digest, TransactionReceipt, TrxKinds, Transaction};
    use super::*;
    use std::{
        error::Error,
        fs::File,
        io::Read,
        path::Path,
    };
    use std::convert::TryFrom;

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

    #[test]
    fn merkle_verify_proof_should_work() {
        let paths: Vec<Checksum256> = vec![
            "0000259943aeb714e885c783bc79487cd025bb687b39d9de755d73a7fea000dd".into(),
            "804c48aed6b4f21b9d13bd3cc260411dc8d7e442f0430659e9bbcc70af95c8aa".into(),
            "80f39c9cda67aa2c1e4ec3a6c2ed6182dbb87b30d2d82b44a2a2a76d37f74aae".into(),
            "29eb5e917272918a6da86be0aaec2275bef5b66062c7f717b738b92b01e24faa".into(),
            "07d415864f60c2ca1318d4ebf4fd46e446697076d4f38abc3105531830da815e".into(),
            "9006d928623a944863b1bef8a6df59fcb9c4790d8fe8b49c2fd4b0f88f48566c".into(),
            "efc734fa150a9cfa74402a7d50fae265f36037c70af9b078bee7c3332fe62768".into(),
            "3e2f1f8b53ec4b22ffe724ba11f1cb676a675a0a6cf097ed1d8a30d766008f76".into(),
            "43e4b272895404d72bdb14f7a06c19342cbdaa132bf3538bb20be67b28db5fc8".into(),
            "9e3a7f7e635ea41663de6855b81eda28320ae3d2ba669e2a8e1e1d4d8969cb5c".into(),
            "2cba7c7ee5c1d8ba97ea1a841707fbb2147e883b56544ba821814aebe086383e".into(),
            "a081325a023dd7018dd99d1d4192348c73d445f4a4fd4ca40a99c1914c3b30b3".into(),
            "8394f7a83fda4dc1fb026aec143ccb4c9ce69c21f23ab3a8af0a741f8597df96".into(),
            "2fa502d408f5bdf1660fa9fe3a1fcb432462467e7eb403a8499392ee5297d8d1".into(),
        ];
        let leaf: Checksum256 = "0000259a7cc27f04467b6c7362a936a143a5d9f324075b4c0d291c3974f80720".into();
        let expected_root: Checksum256 = "1894edef851c070852f55a4dc8fc50ea8f2eafc67d8daad767e4f985dfe54071".into();
        let result = verify_proof(&paths, leaf, expected_root);
        assert!(result);
    }

    #[test]
    fn merkle_get_proof_should_work() {
        let paths: Vec<Checksum256> = vec![
            "c64bf0f3bb9eba5f9cb413fe2de1004b7a00de3b31d021db9260594bb6b2cc19".into(),
            "c30805e721f51ceb274ed10309efb36e34672874598330846a01947457bbfbe7".into(),
            "78e8ce65c904ee9b28bd795e6c3ee052889c0836e8540b6a8629038980711063".into(),
            "6c86969856869eedf432b4bf0c5606482a39844b378b39b8c11a2837ffa0b29a".into(),
            "08c24a718fddc5d1fb56c9766e3e2d610733cece63a2298d368a3d0089b45c8d".into(),
            "1201ff298e82f144dfbed39d86b848952db9ff3620b7a1a3ef5a3202bc2f9c02".into(),
            "158baf523bcf4e96f293e6fa206115988a2555caaed35f5ff550455835fe36e8".into(),
            "45e49f92dc2f69f13af5b0eb833649c6c1671e22721d4bc317fe0062ffce94cf".into(),
            "471e2318b830b600a017d0873c0826f8d1209bc1e94e941f6bb7d1b2066e4f02".into(),
            "59597f3acead00edb3d5eddb77655f9f0d77ccf03b7739d6c487b11b0b6502bd".into(),
            "99be302b60aa6900adb2f7fb163d3ed60143d75ce82579186a0afb96aebc2175".into(),
            "82fe262f4cb290659b3c3ec45afe4e469f53f9a2dc3dbca9e77f0012a3403e15".into(),
            "6da75bf2f9c415cc1c5f30dcc05835919bbf76e78e00b9bac8b6636f09d37b47".into(),
            "00f374e2a86c731208ea791c58c92adc61830123979dcde5fbc0e9088830d89f".into(),
            "e6fcbe2e3aeb101ec3a7b433503a390b22551a9ed583e32f455d21511d28827a".into(),
            "4abf63e7877f37417988c23ac1bdf45ad73607490375f476c3a8e0eb27de72be".into(),
            "2752ea608ea2166cb7f2fc98a2be51fe4419a2beb06cef9c24caab328e98251e".into(),
            "033eddc28ad2d6286c6c352a187a1d18b6973e559553678d03e7a383cfabb737".into(),
            "b1483c708ef118531ff330ae8d9d4e6ada07533285af3a2d2c36f39bccc9ae29".into(),
        ];
        let result = get_proof(15, paths);
        let paths = result.unwrap();

        let leaf: Checksum256 = "4abf63e7877f37417988c23ac1bdf45ad73607490375f476c3a8e0eb27de72be".into();
        let expected_root: Checksum256 = "ba22d6d0fd25ca443ce136d8163cc3ea25d8fc17aa53c971d5a093d61524e6c1".into();
        let result = verify_proof(&paths, leaf, expected_root);
        assert!(result);
    }
}
