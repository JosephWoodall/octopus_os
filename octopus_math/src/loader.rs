use std::fs::File;
use std::io::{Write, Result};
use std::path::Path;

/// Simple TSSM Header
/// Magic: "TSSM" (4 bytes)
/// Version: u32
/// Vocab Size: u32
/// Layers: u32
/// Hidden Dim: u32
/// State Dim: u32
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TssmHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub vocab_size: u32,
    pub layers: u32,
    pub hidden_dim: u32,
    pub state_dim: u32,
}

pub fn generate_dummy_tssm<P: AsRef<Path>>(path: P) -> Result<()> {
    let header = TssmHeader {
        magic: *b"TSSM",
        version: 1,
        vocab_size: 32000,
        layers: 12,
        hidden_dim: 768,
        state_dim: 128,
    };

    let mut file = File::create(path)?;
    
    // Write header
    let header_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            &header as *const TssmHeader as *const u8,
            std::mem::size_of::<TssmHeader>(),
        )
    };
    file.write_all(header_bytes)?;

    // Write some dummy bit-packed weights
    // For simplicity, let's just write some random data for now
    // In a real scenario, this would be packed ternary weights
    let weight_size = (header.layers * header.hidden_dim * header.hidden_dim) / 4; // 2 bits per weight
    let dummy_weights = vec![0u8; weight_size as usize];
    file.write_all(&dummy_weights)?;

    Ok(())
}
