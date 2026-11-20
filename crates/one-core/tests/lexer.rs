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


#[test]
fn lexer_rejects_source_larger_than_four_mib() {
    let source = " ".repeat(4 * 1024 * 1024 + 1);
    let error = one_core::lexer::lex(&source).expect_err("oversized source must fail closed");
    assert_eq!(error.0[0].code, "E0008");
}

#[test]
fn lexer_rejects_more_than_two_hundred_fifty_thousand_tokens() {
    let source = "a ".repeat(250_001);
    let error = one_core::lexer::lex(&source).expect_err("oversized token stream must fail closed");
    assert_eq!(error.0[0].code, "E0009");
}

#[test]
fn lexer_rejects_more_than_two_hundred_fifty_six_indent_levels() {
    let mut source = String::new();
    for depth in 0..=257 {
        source.push_str(&" ".repeat(depth * 4));
        source.push_str("tampilkan \"x\"\n");
    }
    let error = one_core::lexer::lex(&source).expect_err("excessive indentation must fail closed");
    assert_eq!(error.0[0].code, "E0010");
}
