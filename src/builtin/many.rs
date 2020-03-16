use crate::Parser;

pub struct Many<A: Parser> {
    a: A,
}

impl<A: Parser> Many<A> {
    pub(crate) fn new(a: A) -> Self {
        Self { a }
    }
}

impl<A: Parser> Parser for Many<A> {
    type Output = Vec<A::Output>;
    fn parse<'a>(&self, mut s: &'a str) -> Option<(&'a str, Self::Output)> {
        let mut v = Vec::new();
        while let Some(x) = self.a.parse(s) {
            v.push(x.1);
            s = x.0;
        }
        if v.is_empty() {
            None
        } else {
            Some((s, v))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn test_positive() {
        let x = 'a'.many().then('b').then(()).parse("aaaaab").unwrap().1;
        assert_eq!({ x.0 }.0.len(), 5)
    }

    #[test]
    fn test_negative() {
        assert_eq!('a'.many().then('b').then(()).parse("b"), None)
    }
}
