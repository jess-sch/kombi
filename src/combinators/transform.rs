use crate::*;

pub struct Transform<Iter, A, F, T>
where
    Iter: Iterator + Clone,
    Iter::Item: Clone,
    A: Parser<Iter>,
    F: Fn(A::Output) -> Option<T>,
{
    a: A,
    f: F,
    _i: PhantomData<(Iter, T)>,
}

impl<Iter, A, F, T> Transform<Iter, A, F, T>
where
    Iter: Iterator + Clone,
    Iter::Item: Clone,
    A: Parser<Iter>,
    F: Fn(A::Output) -> Option<T>,
{
    pub(crate) fn new(a: A, f: F) -> Self {
        Self {
            a,
            f,
            _i: Default::default(),
        }
    }
}

impl<Iter, A, F, T> Parser<Iter> for Transform<Iter, A, F, T>
where
    Iter: Iterator + Clone,
    Iter::Item: Clone,
    A: Parser<Iter>,
    F: Fn(A::Output) -> Option<T>,
{
    type Output = T;
    fn parse(&self, i: Iter) -> Option<(Iter, Self::Output)> {
        let (i, a) = self.a.parse(i)?;
        Some((i, (self.f)(a)?))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test() {
        assert_eq!(
            ('a'.then('b'))
                .transform(|x| {
                    let mut s = String::new();
                    s.push(x.0);
                    s.push(x.1);
                    Some(s)
                })
                .parse_str("abc"),
            Some(("c", String::from("ab")))
        )
    }
}
