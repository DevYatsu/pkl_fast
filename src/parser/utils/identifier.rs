use crate::{
    parser::PklParser,
    prelude::{ParsingError, ParsingResult, PklToken},
};

pub fn parse_identifier<'source>(parser: &mut PklParser<'source>) -> ParsingResult<&'source str> {
    let token = parser.lexer.next();

    if let Some(Ok(PklToken::Identifier(value))) = token {
        Ok(value)
    } else {
        if token.is_some() {
            Err(ParsingError::invalid_id(parser))
        } else {
            Err(ParsingError::eof(parser, "an identifier"))
        }
    }
}
