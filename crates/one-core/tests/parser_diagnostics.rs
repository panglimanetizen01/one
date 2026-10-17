use one_core::parse;

#[test]
fn hapus_missing_dari_uses_indonesian_diagnostic() {
    let error = parse("hapus apel\n").expect_err("missing 'dari' must be rejected");
    assert_eq!(error.0[0].code, "E1018");
    assert_eq!(error.0[0].message, "'hapus' membutuhkan kata 'dari' sebelum nama daftar.");
}
