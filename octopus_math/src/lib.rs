pub mod loader;

use loader::TssmHeader;
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;
use anyhow::Result;

pub struct TssmModel {
    pub header: TssmHeader,
    pub mmap: Mmap,
}

impl TssmModel {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        if mmap.len() < std::mem::size_of::<TssmHeader>() {
            return Err(anyhow::anyhow!("File too small for TSSM header"));
        }

        let header = unsafe {
            *(mmap.as_ptr() as *const TssmHeader)
        };

        if &header.magic != b"TSSM" {
            return Err(anyhow::anyhow!("Invalid magic number"));
        }

        Ok(Self { header, mmap })
    }

    pub fn get_weights(&self) -> &[u8] {
        &self.mmap[std::mem::size_of::<TssmHeader>()..]
    }
}
