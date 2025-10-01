# Text-Based Testing for vtinput

This directory contains text-based tests for the vtinput terminal event parser,
following the same approach used in `vt-push-parser`.

## Structure

- `common/mod.rs` - Shared test harness that:
  - Parses input files with test cases
  - Runs each test with various chunk sizes to ensure streaming works
  - Generates markdown output files with results in terse format
  - Compares against expected output (or updates with `UPDATE=1`)
- `parser.rs` - Unified test runner that discovers and executes all test suites

### Test Suites

All test suites are run through a unified `parser` test runner:

- **`keyboard_input`** - Basic keyboard input (arrows, function keys, etc.)
- **`utf8_handling`** - UTF-8 encoding validation and error handling
- **`bracketed_paste`** - Bracketed paste mode sequences
- **`control_keys`** - Ctrl key combinations and C0 control codes
- **`alt_keys`** - Alt/Meta key combinations
- **`kitty_keyboard`** - Kitty keyboard protocol (CSI u encoding, progressive enhancement)

Each test suite consists of:
- `.txt` file - Test cases using human-readable escape sequences
- `_result.md` file - Expected output (auto-generated)

The unified test runner (`parser.rs`) discovers and executes all test suites.

## Output Format

Terminal events are formatted tersely for readability:

- **Key events**: `key(press:ctrl-alt-shift-a)` format
  - Event kind: `press`, `repeat`, or `release`
  - Modifiers in conventional order: `ctrl-alt-shift-super-hyper-meta`
  - Key code in lowercase (e.g., `a`, `enter`, `f1`, `up`)
  - Special states like `:keypad` are appended if present

Examples:
- Regular key: `key(press:a)`
- With modifiers: `key(press:ctrl-shift-A)`
- Arrow key: `key(press:up)`
- Function key: `key(press:f1)`
- Multiple modifiers: `key(press:ctrl-alt-shift-up)`

## Test Input Format

Test input files use a simple format:

```
# Test name
<input using escape sequences>

# Another test
more input
```

Escape sequences use angle brackets:
- `<ESC>` - Escape character (0x1B)
- `<CR>` - Carriage return (0x0D)
- `<LF>` - Line feed (0x0A)
- `<TAB>` - Tab (0x09)
- `<SP>` - Space (0x20)
- `<DEL>` - Delete (0x7F)
- C0 control codes: `<SOH>`, `<STX>`, `<ETX>`, `<EOT>`, etc.

Hex byte notation (for invalid UTF-8 testing):
- `<ff>` - Byte 0xFF (case-insensitive)
- `<80>` - Byte 0x80
- `<c3>` - Byte 0xC3
- Any two hex digits: `<00>` through `<ff>`

These are decoded by `vt_push_parser::ascii::decode_string()`.

## Running Tests

Run all test suites:
```bash
cargo test --test parser
```

Run a specific test suite (filter by suite name):
```bash
cargo test --test parser -- keyboard
cargo test --test parser -- utf8
cargo test --test parser -- bracketed
cargo test --test parser -- control
cargo test --test parser -- alt
cargo test --test parser -- kitty_keyboard
```

Update expected output for all tests:
```bash
UPDATE=1 cargo test --test parser
```

Update expected output for a specific suite:
```bash
UPDATE=1 cargo test --test parser -- keyboard
```

Note: The unified test runner automatically discovers all test suites, so there's no need to add new test configurations to `Cargo.toml` when adding new test suites.

## Adding New Tests

1. Create a new `.txt` file with test cases in the `tests/` directory

2. Add the test suite to `tests/parser.rs` in the `discover_test_suites()` function:
   ```rust
   TestSuite {
       name: "your_test".to_string(),
       input: include_str!("your_test.txt").to_string(),
       output_path: "tests/your_test_result.md".to_string(),
       title: "Your Test Suite".to_string(),
   },
   ```

3. Generate initial output:
   ```bash
   UPDATE=1 cargo test --test parser -- your_test
   ```

That's it! No need to create individual test runners or modify `Cargo.toml`.

## How It Works

The test harness:

1. Reads the input file line by line
2. For lines starting with `#`, uses the text as the test name
3. For other non-empty lines, treats them as test input
4. Decodes escape sequences to bytes
5. Parses with `TerminalInputParser` and collects events
6. Re-parses with various chunk sizes to verify streaming behavior
7. Formats events as debug strings
8. Compares against expected output or updates the result file

This approach ensures:
- Tests are human-readable and easy to write
- Parser works correctly with streaming input
- Results are reviewable in version control
- Regressions are caught automatically
