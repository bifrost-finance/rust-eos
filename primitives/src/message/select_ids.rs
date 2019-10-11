use crate::{Checksum256, IdListModes, NumBytes, Read, Write};

#[derive(Clone, Debug, Read, Write, NumBytes, Default, PartialEq)]
#[eosio_core_root_path = "crate"]
pub struct SelectIds {
    pub mode: IdListModes,
    pub pending: u32,
    pub ids: Vec<Checksum256>,
}

impl SelectIds {
    pub fn new(mode: IdListModes, pending: u32, ids: Vec<Checksum256>) -> Self {
        Self {
            mode,
            pending,
            ids,
        }
    }
}

impl SelectIds {
    pub fn empty(&self) -> bool {
        self.mode == IdListModes::None || self.ids.is_empty()
    }
}

impl core::fmt::Display for SelectIds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "\nmode: {}\n\
            pending: {}\n\
            ids: {:?}\n",
            self.mode,
            self.pending,
            self.ids,
        )
    }
}
