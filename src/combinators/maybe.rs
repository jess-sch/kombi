use crate::*;

pub struct Maybe<Iter, A>
where
    Iter: Iterator + Clone,
    Iter::Item: Clone,
    A: Parser<Iter>,
{
    a: A,
    _i: PhantomData<Iter>,
}

impl<Iter, A> Maybe<Iter, A>
where
    Iter: Iterator + Clone,
    Iter::Item: Clone,
    A: Parser<Iter>,
{
    pub(crate) fn new(a: A) -> Self {
        Self {
            a,
            _i: Default::default(),
        }
    }
}

impl<Iter, A> Parser<Iter> for Maybe<Iter, A>
where
    Iter: Iterator + Clone,
    Iter::Item: Clone,
    A: Parser<Iter>,
{
    type Output = Option<A::Output>;
    fn parse(&self, i: Iter) -> Option<(Iter, Self::Output)> {
        if let Some((i, a)) = self.a.parse(i.clone()) {
            Some((i, Some(a)))
        } else {
            Some((i, None))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn positive() {
        assert_eq!(
            "abc".maybe().then('a').parse_str("abca"),
            Some(("", (Some("abc"), 'a')))
        )
    }

    #[test]
    fn negative() {
        assert_eq!("abc".maybe().then('a').parse_str("a"), Some(("", (None, 'a'))))
    }

    #[test]
    fn fail() {
        assert_eq!("abc".maybe().then('a').parse_str("b"), None)
    }
}
