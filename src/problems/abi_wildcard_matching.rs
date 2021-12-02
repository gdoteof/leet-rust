
pub fn is_match(s: String, p: String) -> bool {
    enum Pattern{
        SingleAny,
        ManyOrNone,
        Single(u8),
        Empty
    }
    
    fn is_match_bytes(s: &[u8], p: &[u8]) -> bool {
        println!("S:{:?}, P:{:?}", std::str::from_utf8(s),  std::str::from_utf8(p));
        match parse(p){
            (Pattern::Empty, _) => s.is_empty(),
            (Pattern::ManyOrNone, _) if s.is_empty() => is_match_bytes(s, &p[1..]),
            (Pattern::ManyOrNone, _) => is_match_single(s, Pattern::SingleAny, p) || is_match_bytes(&s[1..], p) || is_match_bytes(s, &p[1..]),
            (single, _)  => is_match_single(s, single, p),
        }
    }

    fn is_match_single(s: &[u8], token: Pattern, p: &[u8]) -> bool {
        match s.split_first() {
            Some((head_s, tail_s)) => match token {
                Pattern::Single(c) if *head_s == c => is_match_bytes(tail_s, &p[1..]),
                Pattern::SingleAny => is_match_bytes(tail_s, &p[1..]),
                _ => false
            },
            None => p.is_empty()
        }
    }

    fn parse(pattern: &[u8]) -> (Pattern, &[u8]){
        match pattern.split_first() {
            Some((b'?', p_rest)) => (Pattern::SingleAny, p_rest),
            Some((b'*', p_rest)) => (Pattern::ManyOrNone, p_rest),
            Some((c, p_rest)) => (Pattern::Single(*c), p_rest),
            _ => (Pattern::Empty, pattern)
        }
    }

    println!("entering");
    println!("S:{:?}, P:{:?}", s, p);
    return is_match_bytes(&s.as_bytes(), &p.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testit(s:&str, p:&str, expected:bool){
        assert_eq!(is_match(s.to_string(), p.to_string()), expected);
    }

    #[test]
    fn it_works() {
        testit("aa", "a", false);
        testit("aa", "*", true);
        testit("cb", "?a", false);
        testit("adceb", "*a*b", true);
        testit("acdcb", "a*c?b", false);
    }

    #[test]
    fn not_working() {
//        testit("aaabbbaabaaaaababaabaaabbabbbbbbbbaabababbabbbaaaaba","a*******b", false)
        testit("aaabbbaabaaaaababaabaaabbabbbbbbbbaabababbabbbaaaaba","a****b", false)
    }

    #[test]
    fn debug_tests() {
        testit("bbbba", ".*a*a", true);
    }

}
