// Copyright 2026 Oxide Computer Company

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn assert_enum_round_trips<T>(inputs: &[&str])
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    for input in inputs {
        let value: T = serde_json::from_str(input).unwrap();
        assert_eq!(serde_json::to_string(&value).unwrap(), *input);
    }
}

#[test]
fn test_large_unsigned_enum_round_trips() {
    assert!(LargeUnsignedEnum::try_from(u64::MAX).is_ok());
    assert_enum_round_trips::<LargeUnsignedEnum>(&["9223372036854775807", "18446744073709551615"]);
}

#[test]
fn test_adjacent_integer_enum_round_trips() {
    assert_enum_round_trips::<AdjacentIntegerEnum>(&["9007199254740992", "9007199254740993"]);
}

#[test]
fn test_large_signed_enum_round_trips() {
    assert!(LargeSignedEnum::try_from(-9007199254740993_i64).is_ok());
    assert_enum_round_trips::<LargeSignedEnum>(&["-9007199254740993", "9007199254740993"]);
}

#[test]
fn test_wide_integer_enum_round_trips() {
    assert!(WideIntegerEnum::try_from(i128::from(u64::MAX)).is_ok());
    assert_enum_round_trips::<WideIntegerEnum>(&["-9223372036854775808", "18446744073709551615"]);
}

#[test]
fn test_nullable_integer_enum_round_trips() {
    assert_enum_round_trips::<Option<NullableIntegerEnum>>(&["null", "9007199254740993"]);
}

#[test]
fn test_integer_enum_rejects_non_members() {
    for input in ["9007199254740992", "9007199254740994", "-9007199254740992"] {
        assert!(
            serde_json::from_str::<LargeSignedEnum>(input).is_err(),
            "{input}"
        );
    }
    for input in ["9223372036854775806", "18446744073709551614", "-1"] {
        assert!(
            serde_json::from_str::<LargeUnsignedEnum>(input).is_err(),
            "{input}"
        );
    }
}

#[test]
fn test_fractional_enum() {
    assert_enum_round_trips::<FractionalEnum>(&["1.5"]);
    assert!(serde_json::from_str::<FractionalEnum>("1").is_ok());
    assert!(serde_json::from_str::<FractionalEnum>("2").is_err());
}
