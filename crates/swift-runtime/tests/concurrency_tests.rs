use swift_runtime::concurrency;

#[test]
fn test_main_executor() {
    let exec = concurrency::main_executor().expect("Should get main executor");
    assert!(
        !exec.identity.is_null() || !exec.implementation.is_null(),
        "Main executor should be non-null"
    );
}

#[test]
fn test_is_main_executor() {
    let exec = concurrency::main_executor().unwrap();
    let is_main = concurrency::is_main_executor(&exec);
    assert_eq!(is_main, Some(true));
}

#[test]
fn test_current_executor() {
    // Outside a Swift task, this should still return Ok
    let _exec = concurrency::current_executor();
}

#[test]
fn test_continuous_time() {
    let t = concurrency::continuous_time().expect("Should read continuous clock");
    assert!(
        t.seconds > 0 || t.nanoseconds > 0,
        "Time should be non-zero"
    );
}

#[test]
fn test_suspending_time() {
    let t = concurrency::suspending_time().expect("Should read suspending clock");
    assert!(
        t.seconds > 0 || t.nanoseconds > 0,
        "Time should be non-zero"
    );
}

#[test]
fn test_time_monotonic() {
    let t1 = concurrency::continuous_time().unwrap();
    let t2 = concurrency::continuous_time().unwrap();
    let ns1 = t1.seconds * 1_000_000_000 + t1.nanoseconds;
    let ns2 = t2.seconds * 1_000_000_000 + t2.nanoseconds;
    assert!(ns2 >= ns1, "Continuous clock should be monotonic");
}
