pub(crate) use std::marker::PhantomData;
pub mod combinators;
pub mod prelude;
pub mod str;

pub trait Parser<Iter>
where
    Self: Sized,
    Iter: Iterator + Clone,
{
    type Output;

    /// Tries to parse the provided Iterator. Returns the
    /// unparsed rest of the data and a value on success.
    fn parse(&self, i: Iter) -> Option<(Iter, Self::Output)>;

    /// Runs the Parser repeatedly until it fails. The results are stored in a `Vec`.
    fn many(self) -> combinators::Many<Iter, Self> {
        combinators::Many::new(self)
    }

    /// Make the Parser optional, so that it will succeed even if it fails.
    fn maybe(self) -> combinators::Maybe<Iter, Self> {
        combinators::Maybe::new(self)
    }

    /// Require that either this or the provided Parser succeed.
    /// If both succeed, the one that has consumed the most bytes will be chosen.
    fn or<Other: Parser<Iter>>(self, other: Other) -> combinators::Or<Iter, Self, Other> {
        combinators::Or::new(self, other)
    }

    /// Chain two Parsers together, requiring both to succeed.
    fn then<Other: Parser<Iter>>(self, other: Other) -> combinators::Then<Iter, Self, Other> {
        combinators::Then::new(self, other)
    }

    /// Apply the provided function to the Output of this Parser.
    fn transform<F: Fn(Self::Output) -> Option<T>, T>(
        self,
        f: F,
    ) -> combinators::Transform<Iter, Self, F, T> {
        combinators::Transform::new(self, f)
    }
}

/// Matches for a character.
impl<Iter> Parser<Iter> for char
where
    Iter: Iterator<Item = char> + Clone,
{
    type Output = char;
    fn parse(&self, mut i: Iter) -> Option<(Iter, Self::Output)> {
        if i.next()? == *self {
            Some((i, *self))
        } else {
            None
        }
    }
}

/// Matches for a string.
impl<'a, Iter> Parser<Iter> for &'a str
where
    Iter: Iterator<Item = char> + Clone,
{
    type Output = &'a str;
    fn parse(&self, mut i: Iter) -> Option<(Iter, Self::Output)> {
        let mut x = self.chars();
        while let Some(c) = x.next() {
            if i.clone().next()? == c {
                i.next()?;
            } else {
                return None;
            }
        }
        Some((i, self))
    }
}

impl<Iter, F, T> Parser<Iter> for F
where
    Iter: Iterator + Clone,
    F: Fn(Iter) -> Option<(Iter, T)>,
{
    type Output = T;
    fn parse(&self, i: Iter) -> Option<(Iter, Self::Output)> {
        (self)(i)
    }
}

/// `()` ensures that there is no remaining data.
impl<Iter> Parser<Iter> for ()
where
    Iter: Iterator + Clone,
{
    type Output = ();
    fn parse(&self, i: Iter) -> Option<(Iter, ())> {
        if i.clone().next().is_none() {
            Some((i, ()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn parse_str_positive() {
        assert_eq!(
            "abc".transform(|x| Some(x)).parse_str("abcdef").unwrap(),
            ("def", "abc")
        )
    }

    #[test]
    fn parse_str_negative() {
        assert_eq!("abcdef".transform(|x| Some(x)).parse_str("abc"), None)
    }

    #[test]
    fn parse_char_positive() {
        assert_eq!('👱'.parse_str("👱abc"), Some(("abc", '👱')))
    }

    #[test]
    fn parse_char_negative() {
        assert_eq!('👱'.parse_str("a"), None)
    }

    #[test]
    fn parse_void_positive() {
        assert_eq!(().parse_str(""), Some(("", ())))
    }

    #[test]
    fn parse_void_negative() {
        assert_eq!(().parse_str(" "), None)
    }
}
