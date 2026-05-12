use super::*;
use crate::loader::TssmHeader;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_ternary_forward() {
    // 1. Create a dummy model file
    let header = TssmHeader {
        magic: *b"TSSM",
        version: 1,
        vocab_size: 10,
        layers: 1,
        hidden_dim: 4,
        state_dim: 2,
    };

    let mut file = NamedTempFile::new().unwrap();
    let header_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            &header as *const TssmHeader as *const u8,
            std::mem::size_of::<TssmHeader>(),
        )
    };
    file.write_all(header_bytes).unwrap();

    // Write 4x4 matrix packed into bytes (4*4/4 = 4 bytes)
    // Row 0: [1, 0, 0, 0] -> 0b00000001
    // Row 1: [0, 1, 0, 0] -> 0b00000100
    // Row 2: [0, 0,-1, 0] -> 0b00100000
    // Row 3: [0, 0, 0, 1] -> 0b01000000
    let weights = vec![0b00000001, 0b00000100, 0b00100000, 0b01000000];
    file.write_all(&weights).unwrap();

    let model = TssmModel::load(file.path()).unwrap();
    let input = vec![1.0, 2.0, 3.0, 4.0];
    let output = model.layer_forward(&input, 0);

    assert_eq!(output, vec![1.0, 2.0, -3.0, 4.0]);
}

#[test]
fn test_ssm_step() {
    let header = TssmHeader {
        magic: *b"TSSM",
        version: 1,
        vocab_size: 10,
        layers: 1,
        hidden_dim: 2,
        state_dim: 2,
    };
    let mut file = NamedTempFile::new().unwrap();
    let header_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            &header as *const TssmHeader as *const u8,
            std::mem::size_of::<TssmHeader>(),
        )
    };
    file.write_all(header_bytes).unwrap();
    // Identity weights for 2x2: 0b01000001
    let weights = vec![0b01000001]; 
    file.write_all(&weights).unwrap();

    let model = TssmModel::load(file.path()).unwrap();
    let mut state = model.new_state();
    let input = vec![1.0, 2.0];
    
    let output = model.step(&input, &mut state);
    
    // Output should be same as input (identity)
    assert_eq!(output, vec![1.0, 2.0]);
    // State should be updated: h = 0*0.9 + 1.0 = 1.0, h1 = 0*0.9 + 2.0 = 2.0
    assert_eq!(state.hidden_state, vec![1.0, 2.0]);
}
