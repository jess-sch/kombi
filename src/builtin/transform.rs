use crate::Parser;

pub struct Transform<A: Parser, F: Fn(A::Output) -> Option<T>, T> {
    a: A,
    f: F,
    _t: std::marker::PhantomData<T>,
}

impl<A: Parser, F: Fn(A::Output) -> Option<T>, T> Transform<A, F, T> {
    pub(crate) fn new(a: A, f: F) -> Self {
        Self {
            a,
            f,
            _t: Default::default(),
        }
    }
}

impl<A: Parser, F: Fn(A::Output) -> Option<T>, T> Parser for Transform<A, F, T> {
    type Output = T;
    fn parse<'a>(&self, s: &'a str) -> Option<(&'a str, T)> {
        let (s, a) = self.a.parse(s)?;
        Some((s, (self.f)(a)?))
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;
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
                .parse("abc"),
            Some(("c", String::from("ab")))
        )
    }
}
