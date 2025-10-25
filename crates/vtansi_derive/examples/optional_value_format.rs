//! Example demonstrating optional fields in value format.
//!
//! This example shows how to use optional fields with value format in both
//! named structs and tuple structs.

use vtansi_derive::{FromAnsi, ToAnsi};
use vtenc::encode::AnsiEncode;
use vtenc::parse::TryFromAnsi;

/// A 2D point with no optional fields.
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi, ToAnsi)]
#[vtansi(format = "value")]
struct Point2D {
    x: i32,
    y: i32,
}

/// A 3D point with one optional trailing field.
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi, ToAnsi)]
#[vtansi(format = "value")]
struct Point3D {
    x: i32,
    y: i32,
    z: Option<i32>,
}

/// A 4D point with two optional trailing fields.
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi, ToAnsi)]
#[vtansi(format = "value")]
struct Point4D {
    x: i32,
    y: i32,
    z: Option<i32>,
    w: Option<i32>,
}

/// A tuple struct with optional trailing fields.
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi, ToAnsi)]
struct Coordinates(i32, i32, Option<i32>);

/// A point with custom delimiter and optional field.
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi, ToAnsi)]
#[vtansi(format = "value", delimiter = ",")]
struct CommaSeparatedPoint {
    x: i32,
    y: i32,
    z: Option<i32>,
}

fn main() {
    println!("=== Optional Fields in Value Format ===\n");

    // Basic 2D point (no optional fields)
    let point2d = Point2D { x: 10, y: 20 };
    let encoded = point2d.encode_ansi().unwrap();
    println!("Point2D: {:?}", point2d);
    println!("Encoded: {:?}", String::from_utf8_lossy(&encoded));
    println!();

    // 3D point with optional z coordinate
    let point3d_with_z = Point3D {
        x: 10,
        y: 20,
        z: Some(30),
    };
    let encoded = point3d_with_z.encode_ansi().unwrap();
    println!("Point3D with z: {:?}", point3d_with_z);
    println!("Encoded: {:?}", String::from_utf8_lossy(&encoded));

    let point3d_without_z = Point3D {
        x: 10,
        y: 20,
        z: None,
    };
    let encoded = point3d_without_z.encode_ansi().unwrap();
    println!("Point3D without z: {:?}", point3d_without_z);
    println!("Encoded: {:?} (trailing None omitted)", String::from_utf8_lossy(&encoded));
    println!();

    // Parsing with optional fields
    println!("=== Parsing ===\n");

    let parsed: Point3D = Point3D::try_from_ansi(b"10;20").unwrap();
    println!("Parse '10;20' as Point3D: {:?}", parsed);
    assert_eq!(parsed.z, None);

    let parsed: Point3D = Point3D::try_from_ansi(b"10;20;30").unwrap();
    println!("Parse '10;20;30' as Point3D: {:?}", parsed);
    assert_eq!(parsed.z, Some(30));
    println!();

    // Multiple optional fields
    println!("=== Multiple Optional Fields ===\n");

    let point4d_all_none = Point4D {
        x: 1,
        y: 2,
        z: None,
        w: None,
    };
    let encoded = point4d_all_none.encode_ansi().unwrap();
    println!("Point4D (all None): {:?}", point4d_all_none);
    println!("Encoded: {:?}", String::from_utf8_lossy(&encoded));

    let point4d_first_some = Point4D {
        x: 1,
        y: 2,
        z: Some(3),
        w: None,
    };
    let encoded = point4d_first_some.encode_ansi().unwrap();
    println!("Point4D (z=Some, w=None): {:?}", point4d_first_some);
    println!("Encoded: {:?}", String::from_utf8_lossy(&encoded));

    let point4d_second_some = Point4D {
        x: 1,
        y: 2,
        z: None,
        w: Some(4),
    };
    let encoded = point4d_second_some.encode_ansi().unwrap();
    println!("Point4D (z=None, w=Some): {:?}", point4d_second_some);
    println!("Encoded: {:?} (empty for middle None)", String::from_utf8_lossy(&encoded));

    let point4d_both_some = Point4D {
        x: 1,
        y: 2,
        z: Some(3),
        w: Some(4),
    };
    let encoded = point4d_both_some.encode_ansi().unwrap();
    println!("Point4D (both Some): {:?}", point4d_both_some);
    println!("Encoded: {:?}", String::from_utf8_lossy(&encoded));
    println!();

    // Tuple struct
    println!("=== Tuple Structs ===\n");

    let coords_with_z = Coordinates(10, 20, Some(30));
    let encoded = coords_with_z.encode_ansi().unwrap();
    println!("Coordinates with z: {:?}", coords_with_z);
    println!("Encoded: {:?}", String::from_utf8_lossy(&encoded));

    let coords_without_z = Coordinates(10, 20, None);
    let encoded = coords_without_z.encode_ansi().unwrap();
    println!("Coordinates without z: {:?}", coords_without_z);
    println!("Encoded: {:?}", String::from_utf8_lossy(&encoded));
    println!();

    // Custom delimiter
    println!("=== Custom Delimiter ===\n");

    let comma_point = CommaSeparatedPoint {
        x: 10,
        y: 20,
        z: Some(30),
    };
    let encoded = comma_point.encode_ansi().unwrap();
    println!("CommaSeparatedPoint: {:?}", comma_point);
    println!("Encoded: {:?}", String::from_utf8_lossy(&encoded));

    let parsed: CommaSeparatedPoint = CommaSeparatedPoint::try_from_ansi(b"10,20").unwrap();
    println!("Parse '10,20': {:?}", parsed);
    assert_eq!(parsed.z, None);
    println!();

    // Roundtrip testing
    println!("=== Roundtrip Testing ===\n");

    let original = Point4D {
        x: 100,
        y: 200,
        z: Some(300),
        w: None,
    };
    let encoded = original.encode_ansi().unwrap();
    let decoded = Point4D::try_from_ansi(&encoded).unwrap();
    println!("Original: {:?}", original);
    println!("Encoded: {:?}", String::from_utf8_lossy(&encoded));
    println!("Decoded: {:?}", decoded);
    assert_eq!(original, decoded);
    println!("✓ Roundtrip successful!");
}