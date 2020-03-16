use crate::*;

impl<'a, T, P: Parser<std::str::Chars<'a>, Output = T>> StrParserExt<'a> for P {}
pub trait StrParserExt<'a>: Parser<std::str::Chars<'a>> {
    fn parse_str(&self, s: &'a str) -> Option<(&'a str, Self::Output)> {
        let (i, x) = Parser::parse(self, s.chars())?;
        Some((i.as_str(), x))
    }
}
