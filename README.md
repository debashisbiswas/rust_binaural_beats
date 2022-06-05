# Binaural Beats Generator
This is an experimental project which creates a wave file with two channels.
The difference in frequency between channels is configurable, allowing for
custom binaural beats.

Currently, the base frequency is an A3 (220 Hz).

## Usage
The project is written in Rust. Use `cargo run --release` to run the project
with the default settings.

### Arguments
Run with the `-h` or `--help` flag to see available arguments. If running with
`cargo`, you can pass arguments using `--`, for example, by using
`cargo run -- -h`.

The bitrate, difference between the two frequencies, and file length are
configurable.

#### Bitrate
`-b`, `--bitrate`

Default is 16 bits per sample. Can be 8 or 16, but you probably want the
default 16 here. 8 was included for completeness with the wave file spec.

#### Difference
`-d`, `--difference`

Default is 2 Hz. Use this argument to specify the difference in frequencies
between the two tones. This will be the frequency of the resulting difference
tone.

#### File Length
`-l`, `--length`

Default is 30 seconds. Specify the number of seconds you would like for the
length of the output file.
