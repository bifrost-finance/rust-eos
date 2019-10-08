use crate::{Checksum256, Read, Write, NumBytes, IdListModes};

#[derive(Clone, Debug, Read, Write, NumBytes, Default, PartialEq)]
#[eosio_core_root_path = "crate"]
pub struct SelectIds {
    pub mode: IdListModes,
    pub pending: u32,
    pub ids: Vec<Checksum256>,
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
