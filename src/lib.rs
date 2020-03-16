pub mod builtin;
pub use builtin::Either;

#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn parse_str() {
        assert_eq!(
            "abc".transform(|x| Some(x)).parse("abcdef").unwrap(),
            ("def", "abc")
        );
        assert_eq!("abcdef".transform(|x| Some(x)).parse("abc"), None);
    }

    #[test]
    fn parse_char() {
        assert_eq!('👱'.parse("a"), None);
        assert_eq!('👱'.parse("👱abc"), Some(("abc", '👱')));
    }

    #[test]
    fn parse_void() {
        assert_eq!(().parse(""), Some(("", ())));
        assert_eq!(().parse(" "), None);
    }
}

pub trait Parser
where
    Self: Sized,
{
    type Output;

    /// Tries to parse the provided string. Returns the
    /// unparsed rest of the string and a value on success.
    fn parse<'a>(&self, s: &'a str) -> Option<(&'a str, Self::Output)>;

    /// Chain two Parsers together, requiring both to succeed.
    fn then<Other: Parser>(self, other: Other) -> builtin::Then<Self, Other> {
        builtin::Then::new(self, other)
    }

    /// Apply the provided function to the Output of this Parser.
    fn transform<F: Fn(Self::Output) -> Option<T>, T>(
        self,
        f: F,
    ) -> builtin::Transform<Self, F, T> {
        builtin::Transform::new(self, f)
    }

    /// Require that either this or the provided Parser succeed.
    /// If both succeed, the one that has consumed the most bytes will be chosen.
    fn or<Other: Parser>(self, other: Other) -> builtin::Or<Self, Other> {
        builtin::Or::new(self, other)
    }

    /// Make the Parser optional, so that it will succeed even if it fails.
    fn maybe(self) -> builtin::Maybe<Self> {
        builtin::Maybe::new(self)
    }

    /// Runs the Parser repeatedly until it fails. The results are stored in a `Vec`.
    fn many(self) -> builtin::Many<Self>{
        builtin::Many::new(self)
    }

    /// Match for whole numbers >=0
    fn natural_number() -> builtin::NaturalNumber {
        builtin::NaturalNumber
    }
}

/// Implements Parser for every `Fn(&str)->Option<(&str,T)>`.
impl<T, F: Fn(&str) -> Option<(&str, T)>> Parser for F {
    type Output = T;
    fn parse<'a>(&self, s: &'a str) -> Option<(&'a str, Self::Output)> {
        (self)(s)
    }
}

/// Matches for a character.
impl Parser for char {
    type Output = char;
    fn parse<'a>(&self, s: &'a str) -> Option<(&'a str, Self::Output)> {
        #[cfg(feature = "str_strip")]
        {
            Some((s.strip_prefix(*self)?, *self));
        }
        #[cfg(not(feature = "str_strip"))]
        {
            let mut chars = s.chars();
            if chars.next()? == *self {
                Some((chars.as_str(), *self))
            } else {
                None
            }
        }
    }
}

/// Matches for a string.
impl<'x> Parser for &'x str {
    type Output = &'x str;
    fn parse<'a>(&self, s: &'a str) -> Option<(&'a str, Self::Output)> {
        #[cfg(feature = "str_strip")]
        {
            Some((s.strip_prefix(*self)?, *self))
        }
        #[cfg(not(feature = "str_strip"))]
        {
            if s.len() < self.len() {
                return None;
            }
            if *self == &s[..self.len()] {
                Some((&s[self.len()..], self))
            } else {
                None
            }
        }
    }
}

/// `()` ensures that there is no remaining data.
impl Parser for () {
    type Output = ();
    fn parse<'a>(&self, s: &'a str) -> Option<(&'a str, ())> {
        if s.is_empty() {
            Some((s, ()))
        } else {
            None
        }
    }
}
