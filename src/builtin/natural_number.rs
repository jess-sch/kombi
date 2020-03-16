use crate::Parser;

pub struct NaturalNumber;

impl Parser for NaturalNumber {
    type Output = u128;
    fn parse<'a>(&self, mut s: &'a str) -> Option<(&'a str, Self::Output)> {
        let mut chars = s.chars();
        let mut val = None;
        while let Some(x) = chars.next() {
            let c = match x {
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
            val = val.or(Some(0)).map(|x| x * 10 + c);
            s = chars.as_str();
        }
        Some((s, val?))
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn positive() {
        assert_eq!(
            <()>::natural_number().parse("1234500b"),
            Some(("b", 1234500))
        )
    }

    #[test]
    fn negative() {
        assert_eq!(<()>::natural_number().parse("b1234500"), None)
    }
}
