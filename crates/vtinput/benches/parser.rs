//! Parser profiling program for benchmarking TerminalInputParser performance.
//!
//! This program generates various terminal input patterns and feeds them
//! through the parser repeatedly to measure performance under different
//! workloads.

use std::hint::black_box;
use std::time::{Duration, Instant};

use vtinput::parser::TerminalInputParser;

/// Benchmark configuration.
struct BenchConfig {
    name: &'static str,
    iterations: usize,
    data: Vec<u8>,
}

impl BenchConfig {
    fn new(name: &'static str, iterations: usize, data: Vec<u8>) -> Self {
        Self {
            name,
            iterations,
            data,
        }
    }
}

/// Run a single benchmark and return timing information.
fn run_benchmark(config: &BenchConfig) -> Duration {
    let start = Instant::now();

    for _ in 0..config.iterations {
        let mut parser = TerminalInputParser::new();
        let data = black_box(&config.data);

        parser.feed_with(data, &mut |event| {
            black_box(event);
        });
    }

    start.elapsed()
}

/// Generate plain ASCII text.
fn generate_plain_text(size: usize) -> Vec<u8> {
    let text = "The quick brown fox jumps over the lazy dog. ";
    text.as_bytes().iter().cycle().take(size).copied().collect()
}

/// Generate text with mixed case.
fn generate_mixed_case(size: usize) -> Vec<u8> {
    let text = "HeLLo WoRLd! ThIs Is MiXeD CaSe TeXt. ";
    text.as_bytes().iter().cycle().take(size).copied().collect()
}

/// Generate text with Unicode characters.
fn generate_unicode_text(size: usize) -> Vec<u8> {
    let text = "Hello 世界! 🦀 Rust ñ café Ω α β γ. ";
    text.as_bytes().iter().cycle().take(size).copied().collect()
}

/// Generate text with ANSI escape sequences.
fn generate_ansi_sequences(size: usize) -> Vec<u8> {
    let mut result = Vec::new();
    let patterns = [
        b"\x1b[1;31mRed\x1b[0m ".as_slice(),
        b"\x1b[32mGreen\x1b[0m ".as_slice(),
        b"\x1b[1;34mBlue\x1b[0m ".as_slice(),
        b"\x1b[H\x1b[2J".as_slice(),
        b"\x1b[10;20HCursor ".as_slice(),
    ];

    let mut idx = 0;
    while result.len() < size {
        result.extend_from_slice(patterns[idx % patterns.len()]);
        idx += 1;
    }

    result.truncate(size);
    result
}

/// Generate control characters mixed with text.
fn generate_control_chars(size: usize) -> Vec<u8> {
    let mut result = Vec::new();
    let text = b"Hello\r\n\tWorld\x08\x1b";

    while result.len() < size {
        result.extend_from_slice(text);
    }

    result.truncate(size);
    result
}

/// Generate CSI sequences (cursor movement, colors, etc).
fn generate_csi_sequences(size: usize) -> Vec<u8> {
    let mut result = Vec::new();
    let sequences = [
        b"\x1b[A".as_slice(),                 // Up
        b"\x1b[B".as_slice(),                 // Down
        b"\x1b[C".as_slice(),                 // Forward
        b"\x1b[D".as_slice(),                 // Back
        b"\x1b[H".as_slice(),                 // Home
        b"\x1b[2J".as_slice(),                // Clear screen
        b"\x1b[38;5;208m".as_slice(),         // 256 color
        b"\x1b[48;2;100;150;200m".as_slice(), // RGB color
    ];

    let mut idx = 0;
    while result.len() < size {
        result.extend_from_slice(sequences[idx % sequences.len()]);
        result.extend_from_slice(b"text ");
        idx += 1;
    }

    result.truncate(size);
    result
}

/// Generate mouse event sequences.
fn generate_mouse_events(size: usize) -> Vec<u8> {
    let mut result = Vec::new();
    let events = [
        b"\x1b[<0;10;20M".as_slice(),  // Mouse down
        b"\x1b[<0;10;20m".as_slice(),  // Mouse up
        b"\x1b[<32;15;25M".as_slice(), // Mouse drag
        b"\x1b[<64;5;5M".as_slice(),   // Scroll up
        b"\x1b[<65;5;5M".as_slice(),   // Scroll down
    ];

    let mut idx = 0;
    while result.len() < size {
        result.extend_from_slice(events[idx % events.len()]);
        result.extend_from_slice(b"abc ");
        idx += 1;
    }

    result.truncate(size);
    result
}

/// Generate worst-case scenario: many incomplete sequences.
fn generate_pathological(size: usize) -> Vec<u8> {
    let mut result = Vec::new();

    while result.len() < size {
        result.push(b'\x1b');
        result.push(b'[');
        result.extend_from_slice(b"1;2;3;4;5");
        result.push(b'm');
        result.push(b'x');
    }

    result.truncate(size);
    result
}

fn main() {
    println!("VT Parser Profiler");
    println!("==================\n");

    // Small buffer size for tight loops
    const SMALL_SIZE: usize = 1024;
    const SMALL_ITERS: usize = 100_000;

    // Medium buffer size
    const MEDIUM_SIZE: usize = 64 * 1024;
    const MEDIUM_ITERS: usize = 10_000;

    // Large buffer size for throughput testing
    const LARGE_SIZE: usize = 1024 * 1024;
    const LARGE_ITERS: usize = 1_000;

    let benchmarks = vec![
        // Small buffer benchmarks
        BenchConfig::new(
            "Plain ASCII (1KB)",
            SMALL_ITERS,
            generate_plain_text(SMALL_SIZE),
        ),
        // BenchConfig::new(
        //     "Mixed case (1KB)",
        //     SMALL_ITERS,
        //     generate_mixed_case(SMALL_SIZE),
        // ),
        BenchConfig::new(
            "Unicode text (1KB)",
            SMALL_ITERS,
            generate_unicode_text(SMALL_SIZE),
        ),
        BenchConfig::new(
            "Control chars (1KB)",
            SMALL_ITERS,
            generate_control_chars(SMALL_SIZE),
        ),
        BenchConfig::new(
            "ANSI sequences (1KB)",
            SMALL_ITERS,
            generate_ansi_sequences(SMALL_SIZE),
        ),
        BenchConfig::new(
            "CSI sequences (1KB)",
            SMALL_ITERS,
            generate_csi_sequences(SMALL_SIZE),
        ),
        BenchConfig::new(
            "Mouse events (1KB)",
            SMALL_ITERS,
            generate_mouse_events(SMALL_SIZE),
        ),
        BenchConfig::new(
            "Pathological (1KB)",
            SMALL_ITERS,
            generate_pathological(SMALL_SIZE),
        ),
        // Medium buffer benchmarks
        BenchConfig::new(
            "Plain ASCII (64KB)",
            MEDIUM_ITERS,
            generate_plain_text(MEDIUM_SIZE),
        ),
        BenchConfig::new(
            "Mixed case (64KB)",
            MEDIUM_ITERS,
            generate_mixed_case(MEDIUM_SIZE),
        ),
        BenchConfig::new(
            "ANSI sequences (64KB)",
            MEDIUM_ITERS,
            generate_ansi_sequences(MEDIUM_SIZE),
        ),
        BenchConfig::new(
            "CSI sequences (64KB)",
            MEDIUM_ITERS,
            generate_csi_sequences(MEDIUM_SIZE),
        ),
        // Large buffer benchmarks
        BenchConfig::new(
            "Plain ASCII (1MB)",
            LARGE_ITERS,
            generate_plain_text(LARGE_SIZE),
        ),
        BenchConfig::new(
            "Mixed case (1MB)",
            LARGE_ITERS,
            generate_mixed_case(LARGE_SIZE),
        ),
        BenchConfig::new(
            "ANSI sequences (1MB)",
            LARGE_ITERS,
            generate_ansi_sequences(LARGE_SIZE),
        ),
    ];

    for config in &benchmarks {
        let elapsed = run_benchmark(config);
        let total_bytes = config.data.len() * config.iterations;
        let throughput_mbs = (total_bytes as f64 / 1_000_000.0) / elapsed.as_secs_f64();

        println!(
            "{:<25} {:>8} iters  {:>8.2} ms  {:>10.2} MB/s",
            config.name,
            config.iterations,
            elapsed.as_secs_f64() * 1000.0,
            throughput_mbs
        );
    }

    println!("\nProfile complete!");
}
