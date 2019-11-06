use crate::Checksum256;

fn make_canonical_left(val: Checksum256) -> Checksum256 {
    let mut canonical_l: Checksum256 = val;
    canonical_l.set_hash0(canonical_l.hash0() & 0xFFFFFFFFFFFFFF7Fu64);
    canonical_l
}

fn  make_canonical_right(val: Checksum256) -> Checksum256 {
    let mut canonical_r: Checksum256 = val;
    canonical_r.set_hash0(canonical_r.hash0() | 0x0000000000000080u64);
    canonical_r
}

fn is_canonical_left(val: Checksum256) -> bool {
    (val.hash0() & 0x0000000000000080u64) == 0
}

fn is_canonical_right(val: Checksum256) -> bool {
    (val.hash0() & 0x0000000000000080u64) != 0
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
            let l = make_canonical_left(ids[2 * i]);
            let r = make_canonical_right(ids[(2 * i) + 1]);

            assert!(is_canonical_left(l));
            assert!(is_canonical_right(r));

            let pair = (l, r);
            ids[i] = Checksum256::hash(pair)?;
        }

        ids.resize(ids.len() / 2, Default::default());
    }

    Ok(ids[0])
}

#[cfg(test)]
mod tests {
    use crate::Checksum256;
    use super::*;

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
}
