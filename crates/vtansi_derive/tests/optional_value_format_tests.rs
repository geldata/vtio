//! Tests for optional fields in value format.
//!
//! This module tests the support for `Option<T>` fields in value format
//! (both named and tuple structs). Optional fields must trail all
//! non-optional fields.

use vtansi_derive::{FromAnsi, ToAnsi};
use vtenc::encode::AnsiEncode;
use vtenc::parse::TryFromAnsi;

// Named struct with no optional fields
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi, ToAnsi)]
#[vtansi(format = "value")]
struct Point2D {
    x: i32,
    y: i32,
}

// Named struct with one trailing optional field
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi, ToAnsi)]
#[vtansi(format = "value")]
struct Point3D {
    x: i32,
    y: i32,
    z: Option<i32>,
}

// Named struct with two trailing optional fields
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi, ToAnsi)]
#[vtansi(format = "value")]
struct Point4D {
    x: i32,
    y: i32,
    z: Option<i32>,
    w: Option<i32>,
}

// Tuple struct with no optional fields
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi, ToAnsi)]
struct Coordinates2D(i32, i32);

// Tuple struct with one trailing optional field
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi, ToAnsi)]
struct Coordinates3D(i32, i32, Option<i32>);

// Tuple struct with two trailing optional fields
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi, ToAnsi)]
struct Coordinates4D(i32, i32, Option<i32>, Option<i32>);

// Struct with optional string field
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi, ToAnsi)]
#[vtansi(format = "value")]
struct LabeledPoint {
    x: i32,
    y: i32,
    label: Option<String>,
}

// Struct with custom delimiter and optional field
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi, ToAnsi)]
#[vtansi(format = "value", delimiter = ",")]
struct CommaSeparatedPoint {
    x: i32,
    y: i32,
    z: Option<i32>,
}

#[test]
fn test_named_struct_no_optional_fields() {
    // Basic test without optional fields
    let point = Point2D { x: 10, y: 20 };
    let encoded = point.encode_ansi().unwrap();
    assert_eq!(encoded, b"10;20");

    let decoded = Point2D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, point);
}

#[test]
fn test_named_struct_one_optional_field_some() {
    let point = Point3D {
        x: 10,
        y: 20,
        z: Some(30),
    };
    let encoded = point.encode_ansi().unwrap();
    assert_eq!(encoded, b"10;20;30");

    let decoded = Point3D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, point);
}

#[test]
fn test_named_struct_one_optional_field_none() {
    let point = Point3D {
        x: 10,
        y: 20,
        z: None,
    };
    let encoded = point.encode_ansi().unwrap();
    // Trailing None should be omitted
    assert_eq!(encoded, b"10;20");

    let decoded = Point3D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, point);
}

#[test]
fn test_named_struct_parse_without_optional() {
    // Parse "10;20" as Point3D
    let decoded = Point3D::try_from_ansi(b"10;20").unwrap();
    assert_eq!(
        decoded,
        Point3D {
            x: 10,
            y: 20,
            z: None
        }
    );
}

#[test]
fn test_named_struct_parse_with_optional() {
    // Parse "10;20;30" as Point3D
    let decoded = Point3D::try_from_ansi(b"10;20;30").unwrap();
    assert_eq!(
        decoded,
        Point3D {
            x: 10,
            y: 20,
            z: Some(30)
        }
    );
}

#[test]
fn test_named_struct_two_optional_fields_both_some() {
    let point = Point4D {
        x: 10,
        y: 20,
        z: Some(30),
        w: Some(40),
    };
    let encoded = point.encode_ansi().unwrap();
    assert_eq!(encoded, b"10;20;30;40");

    let decoded = Point4D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, point);
}

#[test]
fn test_named_struct_two_optional_fields_both_none() {
    let point = Point4D {
        x: 10,
        y: 20,
        z: None,
        w: None,
    };
    let encoded = point.encode_ansi().unwrap();
    // Both trailing None values should be omitted
    assert_eq!(encoded, b"10;20");

    let decoded = Point4D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, point);
}

#[test]
fn test_named_struct_two_optional_fields_first_some_second_none() {
    let point = Point4D {
        x: 10,
        y: 20,
        z: Some(30),
        w: None,
    };
    let encoded = point.encode_ansi().unwrap();
    // Only the last trailing None should be omitted
    assert_eq!(encoded, b"10;20;30");

    let decoded = Point4D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, point);
}

#[test]
fn test_named_struct_two_optional_fields_first_none_second_some() {
    let point = Point4D {
        x: 10,
        y: 20,
        z: None,
        w: Some(40),
    };
    let encoded = point.encode_ansi().unwrap();
    // Middle None should be encoded as empty, last Some should be encoded
    assert_eq!(encoded, b"10;20;;40");

    let decoded = Point4D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, point);
}

#[test]
fn test_named_struct_parse_two_optional_only_first() {
    // Parse "10;20;30" as Point4D (z=Some(30), w=None)
    let decoded = Point4D::try_from_ansi(b"10;20;30").unwrap();
    assert_eq!(
        decoded,
        Point4D {
            x: 10,
            y: 20,
            z: Some(30),
            w: None
        }
    );
}

#[test]
fn test_tuple_struct_no_optional_fields() {
    let coords = Coordinates2D(10, 20);
    let encoded = coords.encode_ansi().unwrap();
    assert_eq!(encoded, b"10;20");

    let decoded = Coordinates2D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, coords);
}

#[test]
fn test_tuple_struct_one_optional_field_some() {
    let coords = Coordinates3D(10, 20, Some(30));
    let encoded = coords.encode_ansi().unwrap();
    assert_eq!(encoded, b"10;20;30");

    let decoded = Coordinates3D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, coords);
}

#[test]
fn test_tuple_struct_one_optional_field_none() {
    let coords = Coordinates3D(10, 20, None);
    let encoded = coords.encode_ansi().unwrap();
    // Trailing None should be omitted
    assert_eq!(encoded, b"10;20");

    let decoded = Coordinates3D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, coords);
}

#[test]
fn test_tuple_struct_two_optional_fields_both_some() {
    let coords = Coordinates4D(10, 20, Some(30), Some(40));
    let encoded = coords.encode_ansi().unwrap();
    assert_eq!(encoded, b"10;20;30;40");

    let decoded = Coordinates4D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, coords);
}

#[test]
fn test_tuple_struct_two_optional_fields_both_none() {
    let coords = Coordinates4D(10, 20, None, None);
    let encoded = coords.encode_ansi().unwrap();
    // Both trailing None values should be omitted
    assert_eq!(encoded, b"10;20");

    let decoded = Coordinates4D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, coords);
}

#[test]
fn test_tuple_struct_two_optional_fields_first_some_second_none() {
    let coords = Coordinates4D(10, 20, Some(30), None);
    let encoded = coords.encode_ansi().unwrap();
    // Only the last trailing None should be omitted
    assert_eq!(encoded, b"10;20;30");

    let decoded = Coordinates4D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, coords);
}

#[test]
fn test_tuple_struct_two_optional_fields_first_none_second_some() {
    let coords = Coordinates4D(10, 20, None, Some(40));
    let encoded = coords.encode_ansi().unwrap();
    // Middle None should be encoded as empty, last Some should be encoded
    assert_eq!(encoded, b"10;20;;40");

    let decoded = Coordinates4D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, coords);
}

#[test]
fn test_optional_string_field_some() {
    let point = LabeledPoint {
        x: 10,
        y: 20,
        label: Some("origin".to_string()),
    };
    let encoded = point.encode_ansi().unwrap();
    assert_eq!(encoded, b"10;20;origin");

    let decoded = LabeledPoint::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, point);
}

#[test]
fn test_optional_string_field_none() {
    let point = LabeledPoint {
        x: 10,
        y: 20,
        label: None,
    };
    let encoded = point.encode_ansi().unwrap();
    assert_eq!(encoded, b"10;20");

    let decoded = LabeledPoint::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, point);
}

#[test]
fn test_custom_delimiter_with_optional() {
    let point = CommaSeparatedPoint {
        x: 10,
        y: 20,
        z: Some(30),
    };
    let encoded = point.encode_ansi().unwrap();
    assert_eq!(encoded, b"10,20,30");

    let decoded = CommaSeparatedPoint::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, point);
}

#[test]
fn test_custom_delimiter_with_optional_none() {
    let point = CommaSeparatedPoint {
        x: 10,
        y: 20,
        z: None,
    };
    let encoded = point.encode_ansi().unwrap();
    assert_eq!(encoded, b"10,20");

    let decoded = CommaSeparatedPoint::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, point);
}

#[test]
fn test_parse_error_too_few_fields() {
    // Try to parse "10" as Point3D (needs at least 2 fields)
    let result = Point3D::try_from_ansi(b"10");
    assert!(result.is_err());
}

#[test]
fn test_parse_error_too_many_fields() {
    // Try to parse "10;20;30;40" as Point3D (max 3 fields)
    let result = Point3D::try_from_ansi(b"10;20;30;40");
    assert!(result.is_err());
}

#[test]
fn test_roundtrip_all_variants() {
    // Test all combinations for Point4D
    let test_cases = vec![
        Point4D {
            x: 1,
            y: 2,
            z: None,
            w: None,
        },
        Point4D {
            x: 1,
            y: 2,
            z: Some(3),
            w: None,
        },
        Point4D {
            x: 1,
            y: 2,
            z: None,
            w: Some(4),
        },
        Point4D {
            x: 1,
            y: 2,
            z: Some(3),
            w: Some(4),
        },
    ];

    for point in test_cases {
        let encoded = point.encode_ansi().unwrap();
        let decoded = Point4D::try_from_ansi(&encoded).unwrap();
        assert_eq!(decoded, point);
    }
}

#[test]
fn test_negative_numbers_with_optional() {
    let point = Point3D {
        x: -10,
        y: -20,
        z: Some(-30),
    };
    let encoded = point.encode_ansi().unwrap();
    assert_eq!(encoded, b"-10;-20;-30");

    let decoded = Point3D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, point);
}

#[test]
fn test_zero_values_with_optional() {
    let point = Point3D {
        x: 0,
        y: 0,
        z: Some(0),
    };
    let encoded = point.encode_ansi().unwrap();
    assert_eq!(encoded, b"0;0;0");

    let decoded = Point3D::try_from_ansi(&encoded).unwrap();
    assert_eq!(decoded, point);
}
