use ffs::scanner::Scanner;
use ffs::types::language::SupportedLanguage;

#[test]
fn test_scan_go() {
    let results = Scanner::new(SupportedLanguage::Go, Some("examples/go".to_string()))
        .scan()
        .unwrap();

    assert_eq!(results.len(), 6);
    assert!(results.iter().any(|f| f.key.as_ref().unwrap() == "foo"));
}
