use crate::*;

pub struct NaturalNumber;

impl<Iter> Parser<Iter> for NaturalNumber
where
    Iter: Iterator<Item = char> + Clone,
{
    type Output = u128;
    fn parse(&self, mut i: Iter) -> Option<(Iter, Self::Output)> {
        let mut val = None;
        let mut i_peek = i.clone();
        while let Some(x) = i_peek.next() {
            let x = match x {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => break,
            };
            val = val.or(Some(0)).map(|o| o * 10 + x);
            i.next();
        }
        Some((i, val?))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn positive() {
        assert_eq!(
            crate::str::NaturalNumber.parse_str("1234500b"),
            Some(("b", 1234500))
        )
    }

    #[test]
    fn negative() {
        assert_eq!(crate::str::NaturalNumber.parse_str("b1234500"), None)
    }
}
