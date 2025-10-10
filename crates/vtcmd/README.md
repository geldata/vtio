# vtcmd

Terminal command sequences implementing common VT escape codes.

This crate provides types that implement the `Encode` trait from `vtansi` to
render terminal control sequences for cursor movement, screen manipulation,
and terminal state queries.

## Features

- **Cursor Movement**: Move cursor to specific positions, lines, or columns
- **Screen Manipulation**: Clear screen, scroll, alternate screen buffer
- **Cursor Control**: Show/hide cursor, set shape, save/restore position
- **Terminal Queries**: Request cursor position, device attributes, feature status
- **Mode Control**: Enable/disable bracketed paste, focus reporting, etc.
- **Window Control**: Set title, resize window
- **Type-safe**: All commands use the `Encode` trait with proper error handling
- **Modular**: Organized into logical modules for easy discovery

## Usage

All commands implement the `Encode` trait from `vtansi`, allowing them to be
encoded into a byte buffer:

```rust
use vtcmd::{ClearAll, cursor::MoveTo};
use vtansi::Encode;
use std::io::Write;

let mut buf = [0u8; 64];

// Clear the screen
let len = ClearAll.encode(&mut buf)?;
std::io::stdout().write_all(&buf[..len])?;

// Move cursor to row 10, column 20
let len = MoveTo { row: 10, col: 20 }.encode(&mut buf)?;
std::io::stdout().write_all(&buf[..len])?;
```

## Modules

Commands are organized into logical modules:

- **`clear`** - Screen clearing commands
- **`cursor`** - Cursor movement and control
- **`screen`** - Screen control (scrolling, alternate screen, line wrapping)
- **`window`** - Window control (title, resize)
- **`mode`** - Mode control (bracketed paste, focus reporting, etc.)
- **`query`** - Terminal query commands

Commonly used types are re-exported at the crate root for convenience.

## Examples

### Basic Screen Manipulation

```rust
use vtcmd::{ClearAll, screen::EnterAlternateScreen};
use vtansi::Encode;

let mut buf = [0u8; 64];

// Enter alternate screen
let len = EnterAlternateScreen.encode(&mut buf)?;
// Write to stdout...

// Clear the screen
let len = ClearAll.encode(&mut buf)?;
// Write to stdout...
```

### Cursor Control

```rust
use vtcmd::cursor::{MoveTo, HideCursor, ShowCursor, SetCursorShape, CursorShape};
use vtansi::Encode;

let mut buf = [0u8; 64];

// Hide cursor during drawing
let len = HideCursor.encode(&mut buf)?;

// Move to position and draw
let len = MoveTo { row: 5, col: 10 }.encode(&mut buf)?;

// Set cursor shape
let len = SetCursorShape(CursorShape::BlinkingBar).encode(&mut buf)?;

// Show cursor again
let len = ShowCursor.encode(&mut buf)?;
```

### Terminal Queries

```rust
use vtcmd::query::{RequestCursorPosition, RequestDeviceAttributes};
use vtansi::Encode;

let mut buf = [0u8; 64];

// Request cursor position (terminal responds with CPR)
let len = RequestCursorPosition.encode(&mut buf)?;

// Request device attributes (terminal responds with DA1)
let len = RequestDeviceAttributes.encode(&mut buf)?;
```

## Command Reference

### Screen Clearing (`clear` module)

- `ClearAll` - Clear entire screen
- `ClearFromCursorDown` - Clear from cursor to end of screen
- `ClearFromCursorUp` - Clear from cursor to beginning of screen
- `ClearLine` - Clear current line
- `ClearUntilNewLine` - Clear from cursor to end of line
- `ClearScrollback` - Purge scrollback buffer

### Cursor Movement (`cursor` module)

- `MoveTo { row, col }` - Move to specific row and column (1-indexed)
- `MoveUp(n)` - Move cursor up by n lines
- `MoveDown(n)` - Move cursor down by n lines
- `MoveLeft(n)` - Move cursor left by n columns
- `MoveRight(n)` - Move cursor right by n columns
- `MoveToNextLine(n)` - Move to beginning of line n lines down
- `MoveToPreviousLine(n)` - Move to beginning of line n lines up
- `MoveToColumn(col)` - Move to specific column on current line

### Cursor Control (`cursor` module)

- `ShowCursor` - Show the cursor
- `HideCursor` - Hide the cursor
- `EnableCursorBlinking` - Enable cursor blinking
- `DisableCursorBlinking` - Disable cursor blinking
- `SetCursorShape(shape)` - Set cursor shape (DECSCUSR)
- `SaveCursorPosition` - Save cursor position (DECSC)
- `RestoreCursorPosition` - Restore cursor position (DECRC)

#### Cursor Shapes

- `CursorShape::Default` - Terminal default
- `CursorShape::BlinkingBlock`
- `CursorShape::SteadyBlock`
- `CursorShape::BlinkingUnderline`
- `CursorShape::SteadyUnderline`
- `CursorShape::BlinkingBar`
- `CursorShape::SteadyBar`

### Screen Control (`screen` module)

- `ScrollUp(n)` - Scroll up by n lines
- `ScrollDown(n)` - Scroll down by n lines
- `EnableLineWrap` - Enable line wrapping
- `DisableLineWrap` - Disable line wrapping
- `EnterAlternateScreen` - Enter alternate screen buffer
- `LeaveAlternateScreen` - Leave alternate screen buffer

### Terminal Queries (`query` module)

- `RequestCursorPosition` - Query current cursor position (CPR)
- `RequestTerminalSize` - Query terminal dimensions
- `RequestDeviceAttributes` - Query primary device attributes (DA1)
- `RequestSecondaryDeviceAttributes` - Query secondary device attributes (DA2)
- `RequestTertiaryDeviceAttributes` - Query tertiary device attributes (DA3)
- `RequestFeature(feature)` - Query specific feature status (DECRQM)
- `RequestDefaultForeground` - Query default foreground color
- `RequestDefaultBackground` - Query default background color
- `RequestCursorShape` - Query cursor shape (DECRQSS)
- `RequestTextAttributes` - Query text attributes (DECRQSS)
- `RequestScrollingRegion` - Query scrolling region top/bottom
- `RequestScrollingColumns` - Query scrolling region left/right

### Mode Control (`mode` module)

- `EnableBracketedPaste` - Enable bracketed paste mode
- `DisableBracketedPaste` - Disable bracketed paste mode
- `EnableFocusReporting` - Enable focus in/out reporting
- `DisableFocusReporting` - Disable focus reporting
- `EnableApplicationKeypad` - Enable application keypad mode (DECKPAM)
- `DisableApplicationKeypad` - Disable application keypad mode (DECKPNM)
- `BeginSynchronizedUpdate` - Begin synchronized update
- `EndSynchronizedUpdate` - End synchronized update

### Window Control (`window` module)

- `SetTitle(title)` - Set terminal window title
- `SetSize { rows, cols }` - Resize terminal window

## Design

This crate uses the `Encode` trait pattern from `vtansi` for efficient, allocation-free
rendering of terminal commands. All escape sequences are generated using the `vtansi`
macros (`csi!`, `osc!`, `dcs!`) to ensure correctness and consistency.

Commands are organized into modules by function, with commonly used types re-exported
at the crate root for convenience. This provides both ergonomic usage and clear
organization for discovery.

## License

MIT