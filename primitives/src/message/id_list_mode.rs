use crate::{Read, Write, NumBytes, ReadError, WriteError};

#[derive(Clone, Debug, PartialEq)]
pub enum IdListModes {
    None,
    CatchUp,
    LastIrrCatchUp,
    Normal,
}

impl Default for IdListModes {
    fn default() -> Self {
        Self::None
    }
}

impl NumBytes for IdListModes {
    fn num_bytes(&self) -> usize {
        4
    }
}

impl From<u32> for IdListModes {
    fn from(mode: u32) -> Self {
        match mode {
            0 => IdListModes::None,
            1 => IdListModes::CatchUp,
            2 => IdListModes::LastIrrCatchUp,
            3 => IdListModes::Normal,
            _ => IdListModes::None,
        }
    }
}

impl From<IdListModes> for u32 {
    fn from(mode: IdListModes) -> Self {
        match mode {
            IdListModes::None           => 0,
            IdListModes::CatchUp        => 1,
            IdListModes::LastIrrCatchUp => 2,
            IdListModes::Normal         => 3,
        }
    }
}

impl Read for IdListModes {
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        u32::read(bytes, pos).map(|res| IdListModes::from(res))
    }
}

impl Write for IdListModes {
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        u32::from(self.clone()).write(bytes, pos)
    }
}

impl core::fmt::Display for IdListModes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let reason_str = match self {
            IdListModes::None => "none",
            IdListModes::CatchUp => "catch up",
            IdListModes::LastIrrCatchUp => "last irreversible",
            IdListModes::Normal => "normal",
        };
        write!(f, "{}", reason_str)
    }
}
