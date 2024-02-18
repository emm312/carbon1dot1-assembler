use std::process::exit;

use codespan_reporting::{
    diagnostic::Diagnostic,
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use lalrpop_util::{lalrpop_mod, ParseError};

use crate::ast::TopLvl;

lalrpop_mod!(parser);

pub fn parse(input: &str, fname: &str) -> TopLvl {
    let res = parser::TopLvlParser::new().parse(input);
    match res {
        Ok(v) => v,
        Err(e) => {
            let writer = StandardStream::stderr(ColorChoice::Always);
            let config = codespan_reporting::term::Config::default();
            let mut diagnostic: Diagnostic<()> = Diagnostic::error();
            match e {
                ParseError::InvalidToken { location } => {
                    diagnostic = diagnostic.with_message("Invalid token");
                    diagnostic = diagnostic.with_labels(vec![
                        codespan_reporting::diagnostic::Label::primary((), location..location)
                            .with_message("Invalid token"),
                    ]);
                }
                ParseError::ExtraToken { token } => {
                    diagnostic = diagnostic.with_message("Extra token");
                    diagnostic = diagnostic.with_labels(vec![
                        codespan_reporting::diagnostic::Label::primary((), token.0..token.2)
                            .with_message("Extra token"),
                    ]);
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    diagnostic = diagnostic.with_message("Unexpected end of file");
                    diagnostic = diagnostic.with_labels(vec![
                        codespan_reporting::diagnostic::Label::primary((), location..location)
                            .with_message(format!("Expected one of: {}", expected.join(", "))),
                    ]);
                }
                ParseError::UnrecognizedToken { token, expected } => {
                    diagnostic = diagnostic.with_message("Unrecognized token");
                    diagnostic = diagnostic.with_labels(vec![
                        codespan_reporting::diagnostic::Label::primary((), token.0..token.2)
                            .with_message(format!("Expected one of: {}", expected.join(", "))),
                    ]);
                }
                _ => unreachable!(),
            }
            term::emit(
                &mut writer.lock(),
                &config,
                &SimpleFile::new(&fname, &input),
                &diagnostic,
            )
            .unwrap();
            exit(-1);
        }
    }
}
