use swift_runtime::demangle;

#[test]
fn test_demangle_int() {
    let result = demangle::demangle(c"$sSiN").expect("Should demangle Int");
    assert!(result.contains("Int"), "Expected 'Int', got: {result}");
}

#[test]
fn test_demangle_string() {
    let result = demangle::demangle(c"$sSSN").expect("Should demangle String");
    assert!(
        result.contains("String"),
        "Expected 'String', got: {result}"
    );
}

#[test]
fn test_demangle_invalid() {
    let result = demangle::demangle(c"not_a_swift_symbol");
    assert!(result.is_none(), "Invalid symbol should return None");
}

#[test]
fn test_demangle_array() {
    let result = demangle::demangle(c"$sSaN").expect("Should demangle Array");
    assert!(
        result.contains("Array"),
        "Expected 'Array', got: {result}"
    );
}
