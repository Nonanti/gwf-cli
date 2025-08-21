#[test]
fn test_version() {
    assert_eq!(env!("CARGO_PKG_VERSION"), "0.1.0");
}

#[test] 
fn test_crate_name() {
    assert_eq!(env!("CARGO_PKG_NAME"), "gwf");
}

#[test]
fn test_authors() {
    let authors = env!("CARGO_PKG_AUTHORS");
    assert!(authors.contains("Nonanti"));
}