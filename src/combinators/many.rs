use crate::*;

pub struct Many<Iter, A>
where
    Iter: Iterator + Clone,
    Iter::Item: Clone,
    A: Parser<Iter>,
{
    a: A,
    _i: PhantomData<Iter>,
}

impl<Iter, A> Many<Iter, A>
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

impl<Iter, A> Parser<Iter> for Many<Iter, A>
where
    Iter: Iterator + Clone,
    Iter::Item: Clone,
    A: Parser<Iter>,
{
    type Output = Vec<A::Output>;
    fn parse(&self, mut i: Iter) -> Option<(Iter, Self::Output)> {
        let mut v = Vec::new();
        while let Some(x) = self.a.parse(i.clone()) {
            v.push(x.1);
            i = x.0;
        }
        if v.is_empty() {
            None
        } else {
            Some((i, v))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_positive() {
        let x = 'a'.many().then('b').then(()).parse_str("aaaaab").unwrap().1;
        assert_eq!({ x.0 }.0.len(), 5)
    }

    #[test]
    fn test_negative() {
        assert_eq!('a'.many().then('b').then(()).parse_str("b"), None)
    }
}
