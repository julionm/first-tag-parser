#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>
}

/*
    * Parsing is a process of deriving 
    * structure from a stream of data
 */

type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>
 {

    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }

}

// ? Fn(&str) -> Result<(&str, Element), &str>

fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {

    match input.chars().next() {
        Some('a') => Ok((&input['a'.len_utf8()..], ())),
        _ => Err(input)
    }

}

// ? THIS IS A HIGHER ORDER FUNCTION (HOF)
fn match_literal(expected: &'static str) -> 
    impl Fn(&str) -> Result<(&str, ()), &str> 
{
    // TODO try to remove this cumbersome get
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => {
            Ok((&input[expected.len()..], ()))
        }
        _ => Err(input)
    }
}

fn identifier(input: &str) -> Result<(&str, String), &str> {

    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input)
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' {
            matched.push(next)
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}

// * basically will match the '<' char, and after, the identifier
fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) ->
    impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>
{
    move |input| 
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2.parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })

}

// * this is a functor
fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> 
    impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B // ? |a| b
{
    move |input| 
        parser.parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result))) 
    
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn literal_parser() {
        let parse_joe = match_literal("Hello Joe!");

        assert_eq!(
            Ok(("", ())),
            parse_joe("Hello Joe!")
        );

        assert_eq!(
            Ok((" Hello Robert!", ())),
            parse_joe("Hello Joe! Hello Robert!")
        );

        assert_eq!(
            Err("Hello Mike!"),
            parse_joe("Hello Mike!")
        );
    }

    #[test]
    fn test_identifier_parser() {
        assert_eq!(
            Ok(("", "i-am-an-identifier".to_string())),
            identifier("i-am-an-identifier"));
        assert_eq!(
            Ok((" entirely", "not".to_string())),
            identifier("not entirely"));
        assert_eq!(
            Err("!no identifier"),
            identifier("!no identifier"));
    }

    #[test]
    fn test_pair_combinator() {
        let tag_opener = pair(match_literal("<"), identifier);

        // assert_eq!(
        //     Ok(("/>", ((), "my-first-element".to_string()))),
        //     tag_opener("<my-first-element/>")
        // );

        // assert_eq!(
        //     Err("oops"),
        //     tag_opener("oops")
        // );

        // assert_eq!(
        //     Err("!oops"),
        //     tag_opener("<!oops")
        // );
    }

}

