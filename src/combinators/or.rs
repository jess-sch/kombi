use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Either<A, B> {
    A(A),
    B(B),
}

pub struct Or<Iter: Iterator + Clone, A: Parser<Iter>, B: Parser<Iter>>
where
    Iter: Iterator + Clone,
    A: Parser<Iter>,
    B: Parser<Iter>,
{
    a: A,
    b: B,
    _i: PhantomData<Iter>,
}

impl<Iter, A, B> Or<Iter, A, B>
where
    Iter: Iterator + Clone,
    A: Parser<Iter>,
    B: Parser<Iter>,
{
    pub(crate) fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            _i: Default::default(),
        }
    }
}

impl<Iter, A, B> Parser<Iter> for Or<Iter, A, B>
where
    Iter: Iterator + Clone,
    A: Parser<Iter>,
    B: Parser<Iter>,
{
    type Output = Either<A::Output, B::Output>;
    fn parse(&self, i: Iter) -> Option<(Iter, Self::Output)> {
        match (self.a.parse(i.clone()), self.b.parse(i)) {
            (Some((i, a)), None) => Some((i, Either::A(a))),
            (None, Some((i, b))) => Some((i, Either::B(b))),
            (Some((ia, a)), Some((ib, b))) => {
                if ia.size_hint().1 <= ib.size_hint().1 {
                    Some((ia, Either::A(a)))
                } else {
                    Some((ib, Either::B(b)))
                }
            }
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equal_length_both() {
        assert_eq!(('a'.or('a')).parse_str("abc"), Some(("bc", A('a'))))
    }

    #[test]
    fn equal_length_a() {
        assert_eq!(('a'.or('b')).parse_str("abc"), Some(("bc", A('a'))))
    }

    #[test]
    fn equal_length_b() {
        assert_eq!(('b'.or('a')).parse_str("abc"), Some(("bc", B('a'))))
    }

    #[test]
    fn fail() {
        assert_eq!(('b'.or('c')).parse_str("abc"), None)
    }

    #[test]
    fn use_longest_if_both() {
        assert_eq!(
            ("hell".or("hello")).parse_str("hello world"),
            Some((" world", B("hello")))
        )
    }
}
