# kombi [![Foo](https://docs.rs/kombi/badge.svg)](https://docs.rs/kombi)

An Iterator-based Parser Combinator Library for Rust.

Included batteries:
* `Parser` trait
    * Implementation for `&str`
    * Implementation for `char`
    * Implementation for `<T, Iter: Iterator<Item: Clone> + Clone> Fn(Iter) -> Option<(Iter, T)>`
* `()` to ensure that there is nothing left to parse
* `Many` for repeated occurrences
* `Maybe` for optionals (e.g. `' '.maybe()` to allow, but not require, a space)
* `NaturalNumber` for whole numbers between 0 and `u128::max_value()`
* `Or` for choosing between two `Parser`s
* `Then` for chaining multiple `Parser`s together
* `Transform` for transforming the output of a `Parser` into another value.

```
("true".or("True"))
.or("false".or("False"))
.transform(|x|match x {
    Either::A(_) => true,
    Either::B(_) => false,
})
.parse_str("false!")? == ("!", false)
```