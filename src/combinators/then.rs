use crate::*;

pub struct Then<Iter, A, B>
where
    Iter: Iterator + Clone,
    A: Parser<Iter>,
    B: Parser<Iter>,
{
    a: A,
    b: B,
    _i: PhantomData<Iter>,
}

impl<Iter, A, B> Then<Iter, A, B>
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

impl<Iter, A, B> Parser<Iter> for Then<Iter, A, B>
where
    Iter: Iterator + Clone,
    A: Parser<Iter>,
    B: Parser<Iter>,
{
    type Output = (A::Output, B::Output);
    fn parse(&self, i: Iter) -> Option<(Iter, Self::Output)> {
        let (i, a) = self.a.parse(i)?;
        let (i, b) = self.b.parse(i)?;
        Some((i, (a, b)))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn positive() {
        assert_eq!(('a'.then('b')).parse_str("abc"), Some(("c", ('a', 'b'))))
    }

    #[test]
    fn negative() {
        assert_eq!(('a'.then('b')).parse_str("cab"), None);
    }
}
