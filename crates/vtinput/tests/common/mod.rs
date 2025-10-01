use pretty_assertions::assert_eq;
use std::fmt::Write;
use vt_push_parser::ascii::{decode_string, encode_string};
use vtinput::event::TerseDisplay;
use vtinput::{TerminalInputEvent, TerminalInputParser};

pub struct TestConfig<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
    pub title: &'a str,
    pub filter: &'a str,
}

struct TerseFormatter<'a, T: TerseDisplay>(&'a T);

impl<'a, T: TerseDisplay> std::fmt::Display for TerseFormatter<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.terse_fmt(f)
    }
}

fn format_event(event: &TerminalInputEvent<'_>) -> String {
    match event {
        TerminalInputEvent::Focus(focused) => {
            format!("Focus({})", if *focused { "true" } else { "false" })
        }
        TerminalInputEvent::Key(key_event) => {
            format!("{}", TerseFormatter(key_event))
        }
        TerminalInputEvent::Mouse(mouse_event) => {
            format!("Mouse({:?})", mouse_event)
        }
        TerminalInputEvent::Paste(text) => {
            format!("Paste({})", encode_string(text))
        }
        TerminalInputEvent::Resize(cols, rows) => {
            format!("Resize({}, {})", cols, rows)
        }
        #[cfg(unix)]
        TerminalInputEvent::CursorPosition(row, col) => {
            format!("CursorPosition({}, {})", row, col)
        }
        #[cfg(unix)]
        TerminalInputEvent::KeyboardEnhancementFlags(flags) => {
            format!("KeyboardEnhancementFlags({:?})", flags)
        }
        #[cfg(unix)]
        TerminalInputEvent::PrimaryDeviceAttributes => "PrimaryDeviceAttributes".to_string(),
        #[cfg(unix)]
        TerminalInputEvent::KeyboardEnhancementFlagsPush(flags) => {
            if let Some(flags) = flags {
                format!("KeyboardEnhancementFlagsPush({:?})", flags)
            } else {
                "KeyboardEnhancementFlagsPush(None)".to_string()
            }
        }
        #[cfg(unix)]
        TerminalInputEvent::KeyboardEnhancementFlagsPop(count) => {
            format!("KeyboardEnhancementFlagsPop({})", count)
        }
        #[cfg(unix)]
        TerminalInputEvent::KeyboardEnhancementFlagsQuery => {
            "KeyboardEnhancementFlagsQuery".to_string()
        }
        TerminalInputEvent::LowLevel(vt_event) => {
            format!("LowLevel({:?})", vt_event)
        }
    }
}

fn parse(data: &[&[u8]]) -> String {
    let mut parser = TerminalInputParser::new();
    let mut result = String::new();
    let mut events = Vec::new();

    for chunk in data {
        parser.feed_with(chunk, &mut |event: TerminalInputEvent<'_>| {
            events.push(event.to_owned());
        });
    }

    parser.idle(&mut |event: TerminalInputEvent<'_>| {
        events.push(event.to_owned());
    });

    for event in events {
        match &event {
            vtinput::TerminalInputEventOwned::LowLevel(vt_event) => {
                writeln!(result, "LowLevel({:?})", vt_event).unwrap();
            }
            _ => {
                writeln!(result, "{}", format_event(&event.borrow())).unwrap();
            }
        }
    }

    result
}

pub fn run_tests<'a>(config: TestConfig<'a>) {
    let mut output = String::new();
    let mut failures = 0;
    output.push_str(&format!("# {}\n", config.title));

    let filter = config.filter;

    let mut test_name = String::new();
    for line in config.input_file.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(stripped_name) = line.trim().strip_prefix("# ") {
            test_name = stripped_name.to_owned();
            continue;
        }

        if !filter.is_empty() && !test_name.contains(filter) {
            continue;
        }

        let decoded = decode_string(line);
        println!("  running {:?} ...", test_name);
        let test_name_clone = test_name.clone();
        let line_clone = line.to_string();
        let Ok(test_output) = std::panic::catch_unwind(move || {
            let mut output = String::new();
            test(&mut output, &test_name_clone, &line_clone, &decoded);
            output
        }) else {
            eprintln!("  test {:?} panicked", test_name);
            failures += 1;
            continue;
        };
        output.push_str(&test_output);
    }

    println!();

    if failures > 0 {
        eprintln!("{} tests failed", failures);
        std::process::exit(1);
    }

    if filter.is_empty() {
        if std::env::var("UPDATE").is_ok() {
            std::fs::write(config.output_file, output).unwrap();
        } else {
            let expected = std::fs::read_to_string(config.output_file).unwrap();
            assert_eq!(expected, output);
            println!("all tests passed");
        }
    }
}

fn test(output: &mut String, test_name: &str, line: &str, decoded: &[u8]) {
    let result = parse(&[decoded]);

    // Ensure that the result is the same when parsing in various
    // chunk sizes
    for chunk_size in 1..=decoded.len() {
        let mut byte_by_byte = Vec::new();
        for b in decoded.chunks(chunk_size) {
            byte_by_byte.push(b);
        }
        let result_byte_by_byte = parse(&byte_by_byte);
        assert_eq!(
            result,
            result_byte_by_byte,
            "Failed to parse in chunks of size {chunk_size} ({:02X?})",
            decoded.chunks(chunk_size).collect::<Vec<_>>()
        );
    }

    output.push_str(&format!("## {test_name}\n```\n{}\n```\n\n", line));
    output.push_str("```\n");
    output.push_str(&result);
    output.push_str("```\n");
    output.push_str("---\n");
}
