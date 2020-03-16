use crate::Parser;

pub struct Then<A: Parser, B: Parser> {
    a: A,
    b: B,
}

impl<A: Parser, B: Parser> Then<A, B> {
    pub(crate) fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A: Parser, B: Parser> Parser for Then<A, B> {
    type Output = (A::Output, B::Output);
    fn parse<'a>(&self, s: &'a str) -> Option<(&'a str, Self::Output)> {
        let (s, a) = self.a.parse(s)?;
        let (s, b) = self.b.parse(s)?;
        Some((s, (a, b)))
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;
    #[test]
    fn positive() {
        assert_eq!(('a'.then('b')).parse("abc"), Some(("c", ('a', 'b'))))
    }

    #[test]
    fn negative() {
        assert_eq!(('a'.then('b')).parse("cab"), None);
    }
}
