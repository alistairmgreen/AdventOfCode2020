// Based on https://bodil.lol/Matcher-combinators/

pub type MatchResult<'a> = Result<&'a str, &'a str>;

pub trait Matcher<'a> {
    fn matches(&self, input: &'a str) -> MatchResult<'a>;

    fn exactly_matches(&self, input: &'a str) -> bool {
        match self.matches(input) {
            Ok(remaining) => remaining.is_empty(),
            Err(_) => false
        }
    }
}

impl<'a, F> Matcher<'a> for F
where
    F: Fn(&'a str) -> MatchResult<'a>,
{
    fn matches(&self, input: &'a str) -> MatchResult<'a> {
        self(input)
    }
}

pub struct BoxedMatcher<'a> {
    matcher: Box<dyn Matcher<'a> + 'a>,
}

impl<'a> BoxedMatcher<'a> {
    pub fn new<P>(matcher: P) -> Self
    where
        P: Matcher<'a> + 'a,
    {
        BoxedMatcher {
            matcher: Box::new(matcher),
        }
    }
}

impl<'a> Matcher<'a> for BoxedMatcher<'a> {
    fn matches(&self, input: &'a str) -> MatchResult<'a> {
        self.matcher.matches(input)
    }
}

pub fn literal<'a>(value: char) -> impl Fn(&'a str) -> MatchResult<'a> {
    move |input| {
        if input.starts_with(value) {
            Ok(&input[value.len_utf8()..])
        } else {
            Err(input)
        }
    }
}

pub fn either<'a, P1, P2>(a: P1, b: P2) -> impl Matcher<'a>
where
    P1: Matcher<'a>,
    P2: Matcher<'a>,
{
    move |input| match a.matches(input) {
        Ok(value) => Ok(value),
        Err(_) => b.matches(input),
    }
}

pub fn pair<'a, P1, P2>(a: P1, b: P2) -> impl Matcher<'a>
where
    P1: Matcher<'a>,
    P2: Matcher<'a>,
{
    move |input| {
        a.matches(input)
            .and_then(|rest| b.matches(rest))
            .map_err(|_| input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_literal_ok() {
        let result = literal('a')("abc");
        assert_eq!(result, Ok("bc"));
    }

    #[test]
    fn test_literal_error() {
        let result = literal('b')("abc");
        assert_eq!(result, Err("abc"));
    }

    #[test]
    fn test_match_either() {
        let result = either(literal('a'), literal('b')).matches("bcde");
        assert_eq!(result, Ok("cde"));
    }

    #[test]
    fn test_match_pair_ok() {
        let result = pair(literal('a'), literal('b')).matches("abcde");
        assert_eq!(result, Ok("cde"));
    }

    #[test]
    fn test_match_pair_error() {
        let result = pair(literal('a'), literal('c')).matches("abcde");
        assert_eq!(result, Err("abcde"));
    }

    #[test]
    fn test_boxed() {
        let a = BoxedMatcher::new(literal('a'));
        let b = BoxedMatcher::new(literal('b'));
        let p = pair(a, b);
        assert_eq!(Ok("cde"), p.matches("abcde"));
        assert_eq!(Err("acbde"), p.matches("acbde"));
    }
}
