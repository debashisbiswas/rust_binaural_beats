use std::{fs::File, io::{Result, Write}};

const SAMPLE_RATE: u32 = 44100;
const NUM_CHANNELS: u16 = 1;
const BITS_PER_SAMPLE: u16 = 16;

// TODO: subchunk1_size should always be 16 for PCM, remove param
fn generate_header(subchunk1_size: u32, subchunk2_size: u32) -> Vec<u8> {
    let mut buffer = Vec::new();

    // RIFF header
    buffer.extend_from_slice(b"RIFF");
    let chunk_size = 4 + (8 + subchunk1_size) + (8 + subchunk2_size);
    buffer.extend_from_slice(&chunk_size.to_ne_bytes());
    buffer.extend_from_slice(b"WAVE");

    // Format chunk
    buffer.extend_from_slice(b"fmt ");
    buffer.extend_from_slice(&subchunk1_size.to_le_bytes()); // chunk size
    buffer.extend_from_slice(&1_u16.to_le_bytes()); // audio format
    buffer.extend_from_slice(&NUM_CHANNELS.to_le_bytes()); // num channels
    buffer.extend_from_slice(&SAMPLE_RATE.to_le_bytes()); // sample rate

    let block_align: u16 = NUM_CHANNELS * BITS_PER_SAMPLE / 8;
    let byte_rate: u32 = SAMPLE_RATE * (block_align as u32);
    buffer.extend_from_slice(&byte_rate.to_le_bytes()); // byte rate
    buffer.extend_from_slice(&block_align.to_le_bytes()); // block align
    buffer.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes()); // bit depth
    buffer
}

fn main() -> Result<()> {
    let mut file = File::create("test.wav")?;
    file.write(&generate_header(16, 0))?;

    Ok(())
}
