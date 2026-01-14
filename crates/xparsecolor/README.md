# xparsecolor

A Rust implementation of X11's `XParseColor` functionality for parsing and representing colors in various color spaces.

## Features

- Parse X11 color specification strings into typed color values
- Support for multiple color spaces as defined by X11/Xcms
- Efficient encoding back to X11 color specification format
- ~750 named X11 colors from `rgb.txt` (case-insensitive, spaces optional)
- Integration with the [`palette`](https://crates.io/crates/palette) crate for color space conversions

## Supported Color Spaces

| Format | Example | Description |
|--------|---------|-------------|
| `rgb:` | `rgb:ffff/8080/0000` | RGB Device (16-bit components) |
| `rgbi:` | `rgbi:1.0/0.5/0.0` | RGB Intensity (0.0–1.0) |
| `#` | `#ff8000` | Legacy sharp syntax (4/8/12/16-bit) |
| `CIEXYZ:` | `CIEXYZ:0.5/0.3/0.2` | CIE 1931 XYZ |
| `CIEuvY:` | `CIEuvY:0.2/0.3/0.5` | CIE 1976 u'v'Y |
| `CIExyY:` | `CIExyY:0.3/0.3/0.5` | CIE xyY chromaticity |
| `CIELab:` | `CIELab:50/25/-25` | CIE 1976 L\*a\*b\* |
| `CIELuv:` | `CIELuv:50/25/-25` | CIE 1976 L\*u\*v\* |
| `TekHVC:` | `TekHVC:180/50/25` | Tektronix HVC |
| Named | `dark slate gray` | X11 named colors |

## Usage

```rust
use xparsecolor::XColor;

// Parse from string
let color: XColor = "rgb:ffff/8080/0000".parse().unwrap();
let color: XColor = "#ff8000".parse().unwrap();
let color: XColor = "dark slate gray".parse().unwrap();

// Parse from bytes (no allocation for ASCII input)
let color = XColor::try_from_bytes(b"rgbi:1.0/0.5/0.0").unwrap();

// Convert to 8-bit RGB
let (r, g, b) = color.to_rgb8();

// Encode back to X11 format
let encoded = color.encode();
```

## References

- [Xlib Color Structures](https://tronche.com/gui/x/xlib/color/structures.html)
- [X11 rgb.txt](https://gitlab.freedesktop.org/xorg/app/rgb)
