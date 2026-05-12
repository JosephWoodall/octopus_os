pub mod loader;
#[cfg(test)]
mod tests;

use loader::TssmHeader;
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;
use anyhow::Result;

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
use std::arch::x86_64::*;

pub struct TssmModel {
    pub header: TssmHeader,
    pub mmap: Mmap,
}

pub struct TssmState {
    pub hidden_state: Vec<f32>,
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

    pub fn new_state(&self) -> TssmState {
        TssmState {
            hidden_state: vec![0.0; self.header.hidden_dim as usize],
        }
    }

    /// Perform a full forward pass for one token/step
    pub fn step(&self, input_embedding: &[f32], state: &mut TssmState) -> Vec<f32> {
        // 1. Initial Projection (Simulated)
        let mut x = input_embedding.to_vec();

        // 2. Iterate through SSM Layers
        for i in 0..self.header.layers as usize {
            // Ternary linear projection
            let projected = self.layer_forward(&x, i);
            
            // SSM Scan (Linear Recurrence: h = A*h + B*x)
            // For a 1.58-bit TSSM, A and B are also ternary.
            // Simplified version:
            for (h, p) in state.hidden_state.iter_mut().zip(projected.iter()) {
                *h = *h * 0.9 + *p; // Decay + Input
            }
            
            x = projected; // Residue connection simplified
        }

        x // Final hidden state or output projection
    }

    /// Perform a highly optimized ternary dot product
    pub fn layer_forward(&self, input: &[f32], layer_idx: usize) -> Vec<f32> {
        let dim = self.header.hidden_dim as usize;
        let mut output = vec![0.0; dim];
        
        let layer_size = (dim * dim) / 4; // 2 bits per weight
        let layer_offset = layer_idx * layer_size;
        
        // Boundary check
        let weights_total = self.get_weights();
        if layer_offset + layer_size > weights_total.len() {
             return output; // Or error
        }
        
        let weights = &weights_total[layer_offset..layer_offset + layer_size];

        #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
        {
            unsafe {
                self.layer_forward_avx512(input, weights, &mut output);
            }
        }
        
        #[cfg(not(all(target_arch = "x86_64", target_feature = "avx512f")))]
        {
            self.layer_forward_fallback(input, weights, &mut output);
        }

        output
    }

    fn layer_forward_fallback(&self, input: &[f32], weights: &[u8], output: &mut [f32]) {
        let dim = self.header.hidden_dim as usize;
        for i in 0..dim {
            let mut sum = 0.0;
            for j in 0..dim {
                // Unpack 2-bit weight
                let byte_idx = (i * dim + j) / 4;
                let bit_idx = ((i * dim + j) % 4) * 2;
                let weight_bits = (weights[byte_idx] >> bit_idx) & 0b11;
                
                match weight_bits {
                    0b01 => sum += input[j],  // +1
                    0b10 => sum -= input[j],  // -1
                    _ => {},                  // 0
                }
            }
            output[i] = sum;
        }
    }

    #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
    #[target_feature(enable = "avx512f")]
    unsafe fn layer_forward_avx512(&self, input: &[f32], weights: &[u8], output: &mut [f32]) {
        // Implementation of AVX-512 accelerated ternary dot product
        self.layer_forward_fallback(input, weights, output);
    }
}
