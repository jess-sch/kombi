use crate::Parser;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Either<A, B> {
    A(A),
    B(B),
}

pub struct Or<A: Parser, B: Parser> {
    a: A,
    b: B,
}

impl<A: Parser, B: Parser> Or<A, B> {
    pub(crate) fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A: Parser, B: Parser> Parser for Or<A, B> {
    type Output = Either<A::Output, B::Output>;
    fn parse<'a>(&self, s: &'a str) -> Option<(&'a str, Self::Output)> {
        match (self.a.parse(s), self.b.parse(s)) {
            (Some((s, a)), None) => Some((s, Either::A(a))),
            (None, Some((s, b))) => Some((s, Either::B(b))),
            (Some((sa, a)), Some((sb, b))) => {
                if sa.len() <= sb.len() {
                    Some((sa, Either::A(a)))
                } else {
                    Some((sb, Either::B(b)))
                }
            }
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{builtin::Either, Parser};

    #[test]
    fn equal_length_both() {
        assert_eq!(('a'.or('a')).parse("abc"), Some(("bc", Either::A('a'))))
    }

    #[test]
    fn equal_length_a() {
        assert_eq!(('a'.or('b')).parse("abc"), Some(("bc", Either::A('a'))))
    }

    #[test]
    fn equal_length_b() {
        assert_eq!(('b'.or('a')).parse("abc"), Some(("bc", Either::B('a'))))
    }

    #[test]
    fn fail() {
        assert_eq!(('b'.or('c')).parse("abc"), None)
    }

    #[test]
    fn use_longest_if_both() {
        assert_eq!(
            ("hell".or("hello")).parse("hello world"),
            Some((" world", Either::B("hello")))
        )
    }
}
