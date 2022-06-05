use std::{
    f64::consts::PI,
    fs::File,
    io::{Error, Write},
    time::Instant,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Bitrate, must be 8 or 16
    #[clap(short, long, default_value_t = 16, parse(try_from_str=check_bitrate))]
    bitrate: u16,

    /// Difference between tones in left/right channels
    #[clap(short, long, default_value_t = 2.0)]
    difference: f64,

    /// Number of seconds of audio to generate
    #[clap(short, long, default_value_t = 30)]
    length: u32,
}

const SAMPLE_RATE: u32 = 44100;
const NUM_CHANNELS: u16 = 2;
const BITS_PER_SAMPLE: u16 = 16;
const SUPPORTED_BITRATES: [u16; 2] = [8, 16];

const SUBCHUNK1_SIZE: u32 = 16;
const AUDIO_FORMAT: u16 = 1; // PCM = 1

fn check_bitrate(s: &str) -> Result<u16, String> {
    let bitrate: u16 = s
        .parse()
        .map_err(|_| format!("`{}` isn't a valid number", s))?;
    if SUPPORTED_BITRATES.contains(&bitrate) {
        Ok(bitrate)
    } else {
        Err(format!(
            "Bitrate not in supported bitrates: {:?}",
            SUPPORTED_BITRATES
        ))
    }
}

fn make_sin_sample(step: f64, freq: f64, bitrate: u16) -> Result<Vec<u8>, &'static str> {
    let mut output = Vec::new();
    let sin = f64::sin(2.0 * PI * freq * step / SAMPLE_RATE as f64);
    match bitrate {
        8 => {
            let s = f64::floor(255.0 * (0.5 * sin + 0.5)) as u8;
            output.push(s);
        }
        16 => {
            let (in_min, in_max) = (-1.0, 1.0);
            let (out_min, out_max) = (i16::MIN as f64, i16::MAX as f64);
            let slope = (out_max - out_min) / (in_max - in_min);
            let scaled = out_min + slope * (sin - in_min);
            output.extend_from_slice(&(scaled as i16).to_le_bytes());
        }
        _ => return Err("Unsupported bits per sample"),
    }
    Ok(output)
}

fn generate_samples(secs: u32, freq: f64, bitrate: u16, diff: f64) -> Vec<u8> {
    let num_samples = secs * SAMPLE_RATE as u32;
    let mut buffer = Vec::with_capacity(num_samples as usize);

    for t in 0..num_samples {
        let t = t as f64;
        let left_sample = make_sin_sample(t, freq, bitrate);
        match left_sample {
            Ok(samples) => buffer.extend(samples),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }

        if NUM_CHANNELS == 2 {
            let freq = freq + diff;
            let right_sample = make_sin_sample(t, freq, bitrate);
            match right_sample {
                Ok(samples) => buffer.extend(samples),
                Err(e) => {
                    println!("{}", e);
                    break;
                }
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

    buffer.extend_from_slice(b"data");
    buffer.extend_from_slice(&subchunk2_size.to_le_bytes());

    buffer
}

fn main() -> Result<(), Error> {
    let timer = Instant::now();
    let args = Args::parse();

    let mut file = File::create("out.wav")?;
    let samples = generate_samples(args.length, 220.0, args.bitrate, args.difference);
    let header = generate_header(samples.len() as u32);
    file.write_all(&header)?;
    file.write_all(&samples)?;

    let elapsed = timer.elapsed().as_secs_f32();
    println!("Took {:.3} seconds.", elapsed);

    Ok(())
}
