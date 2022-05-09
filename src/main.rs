use std::{fs::File, io::{Result, Write}};

const SAMPLE_RATE: u32 = 44100;
const NUM_CHANNELS: u16 = 1;
const BITS_PER_SAMPLE: u16 = 8;

fn generate_samples(secs: u32, freq: f64) -> Vec<u8> {
    let num_samples = secs * SAMPLE_RATE;
    let mut buffer = Vec::with_capacity(num_samples as usize);
    let two_pi = 2.0 * std::f64::consts::PI;
    for t in 0..num_samples {
        let w = two_pi * freq * t as f64 / SAMPLE_RATE as f64;
        let s = f64::sin(w);
        let s = f64::floor(255.0 * (0.5 * s + 0.5)) as u8;
        buffer.push(s);
    }
    buffer
}

fn generate_header(subchunk2_size: u32) -> Vec<u8> {
    let mut buffer = Vec::new();
    let subchunk1_size = 16;

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

    // TODO: feels like this should be put somewhere else
    // Data chunk
    buffer.extend_from_slice(b"data");
    buffer.extend_from_slice(&subchunk2_size.to_le_bytes());

    buffer
}

fn main() -> Result<()> {
    let mut file = File::create("out.wav")?;
    let samples = generate_samples(5, 220.0);
    let header = generate_header(samples.len() as u32);
    file.write_all(&header)?;
    file.write_all(&samples)?;

    Ok(())
}
