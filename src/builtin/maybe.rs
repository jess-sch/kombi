use crate::Parser;

pub struct Maybe<A: Parser> {
    a: A,
}

impl<A: Parser> Maybe<A> {
    pub(crate) fn new(a: A) -> Self {
        Self { a }
    }
}

impl<A: Parser> Parser for Maybe<A> {
    type Output = Option<A::Output>;
    fn parse<'a>(&self, s: &'a str) -> Option<(&'a str, Self::Output)> {
        if let Some((s, a)) = self.a.parse(s) {
            Some((s, Some(a)))
        } else {
            Some((s, None))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn positive() {
        assert_eq!(
            "abc".maybe().then('a').parse("abca"),
            Some(("", (Some("abc"), 'a')))
        )
    }

    #[test]
    fn negative() {
        assert_eq!("abc".maybe().then('a').parse("a"), Some(("", (None, 'a'))))
    }

    #[test]
    fn fail() {
        assert_eq!("abc".maybe().then('a').parse("b"), None)
    }
}
