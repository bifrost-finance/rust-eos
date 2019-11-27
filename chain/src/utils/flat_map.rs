use crate::{Read, Write, NumBytes, UnsignedInt, WriteError, ReadError};
use codec::{Encode, Decode};

#[derive(Clone, Debug, Default, Encode, Decode)]
pub struct FlatMap<K, V> {
    maps: Vec<(K, V)>,
}

impl<K, V> FlatMap<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            maps: vec![(key, value)],
        }
    }

    pub fn size(&self) -> UnsignedInt {
        UnsignedInt::from(self.maps.len())
    }
}

impl<K: NumBytes, V: NumBytes> NumBytes for FlatMap<K, V> {
    fn num_bytes(&self) -> usize {
        let mut size = self.size().num_bytes();
        for map in self.maps.iter() {
            size = size.saturating_add(map.num_bytes());
        }

        size
    }
}

impl<K: Write, V: Write> Write for FlatMap<K, V> {
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        self.size().write(bytes, pos)?;
        for map in self.maps.iter() {
            map.write(bytes, pos)?;
        }

        Ok(())
    }
}

impl <K: Read, V: Read> Read for FlatMap<K, V> {
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let size = UnsignedInt::read(bytes, pos)?;
        let size = usize::from(size);
        let mut maps: Vec<(K, V)> = Vec::with_capacity(size);
        for _ in 0..size {
            let map = <(K, V)>::read(bytes, pos)?;
            maps.push(map);
        }

        Ok(FlatMap { maps })
    }
}

