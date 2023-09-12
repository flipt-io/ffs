use ffs::scanner::Scanner;
use ffs::types::language::SupportedLanguage;

#[test]
fn test_scan_go() {
    let results = Scanner::new(SupportedLanguage::Go, Some("examples/go".to_string()))
        .scan()
        .unwrap();

    assert_eq!(results.len(), 6);
}

#[test]
fn test_scan_typescript() {
    let results = Scanner::new(
        SupportedLanguage::Typescript,
        Some("examples/typescript".to_string()),
    )
    .scan()
    .unwrap();

    assert_eq!(results.len(), 3);
}
