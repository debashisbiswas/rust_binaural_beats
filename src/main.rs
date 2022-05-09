use std::{
    fs::File,
    io::{Result, Write},
};

const SAMPLE_RATE: u32 = 44100;
const NUM_CHANNELS: u16 = 1;
const BITS_PER_SAMPLE: u16 = 16;

const SUBCHUNK1_SIZE: u32 = 16;
const AUDIO_FORMAT: u16 = 1; // PCM = 1

fn generate_samples(secs: u32, freq: f64) -> Vec<u8> {
    let num_samples = secs * SAMPLE_RATE;
    let mut buffer = Vec::with_capacity(num_samples as usize);
    let two_pi = 2.0 * std::f64::consts::PI;
    for t in 0..num_samples {
        let w = two_pi * freq * t as f64 / SAMPLE_RATE as f64;
        let s = f64::sin(w);
        match BITS_PER_SAMPLE {
            8 => {
                let s = f64::floor(255.0 * (0.5 * s + 0.5)) as u8;
                buffer.push(s);
            },
            16 => {
                let w = two_pi * freq * t as f64 / SAMPLE_RATE as f64;
                let s = f64::sin(w);
                let (in_min, in_max) = (-1.0, 1.0);
                let (out_min, out_max) = (i16::MIN as f64, i16::MAX as f64);
                let slope = (out_max - out_min) / (in_max - in_min);
                let scaled = out_min + slope * (s - in_min);
                buffer.extend_from_slice(&(scaled as i16).to_le_bytes());
            },
            _ => {
                println!("Unsupported bits per sample: {}", BITS_PER_SAMPLE);
                break;
            }
        }
    }
    buffer
}

fn generate_header(subchunk2_size: u32) -> Vec<u8> {
    let mut buffer = Vec::new();

    // RIFF header
    let chunk_size = 4 + (8 + SUBCHUNK1_SIZE) + (8 + subchunk2_size);
    buffer.extend_from_slice(b"RIFF");
    buffer.extend_from_slice(&chunk_size.to_ne_bytes());
    buffer.extend_from_slice(b"WAVE");

    // Format chunk
    buffer.extend_from_slice(b"fmt ");
    buffer.extend_from_slice(&SUBCHUNK1_SIZE.to_le_bytes()); // chunk size
    buffer.extend_from_slice(&AUDIO_FORMAT.to_le_bytes()); // audio format
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
