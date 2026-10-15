use one_core::{lex, TokenKind};

#[test]
fn lexes_indonesian_data_statement() {
    let tokens = lex("nama adalah \"Budi\"\n").expect("source should lex");
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].kind, TokenKind::Keyword);
    assert_eq!(tokens[2].kind, TokenKind::Text);
}

#[test]
fn reports_tab_indentation_with_stable_lexical_code() {
    let error = lex("kalau benar\n\ttampilkan \"x\"\n").expect_err("tab indentation must be rejected");
    assert_eq!(error.0[0].code, "E0001");
}
