
pub fn is_match(s: String, p: String) -> bool {
    enum Pattern{
        SingleAny,
        ManyOrNone,
        Single(u8),
        Empty
    }

    // a patternsize+1 x candidatesize+1 cache
    let dp:&mut Vec<Vec<Option<bool>>> = &mut vec!(vec!(None; s.len() + 1); p.len() + 1);
    
    fn is_match_bytes(s: &[u8], p: &[u8], dp: &mut Vec<Vec<Option<bool>>>) -> bool {
        // println!("S:{:?}, P:{:?}", std::str::from_utf8(s),  std::str::from_utf8(p));
        match dp[p.len()][s.len()] {
            Some(b) => return b,
            _ => ()
        };

        let res = match parse(p){
            Pattern::Empty => s.is_empty(),
            Pattern::ManyOrNone if s.is_empty() => is_match_bytes(s, &p[1..], dp),
            Pattern::ManyOrNone => is_match_single(s, Pattern::SingleAny, p, dp) || is_match_bytes(&s[1..], p, dp) || is_match_bytes(s, &p[1..], dp),
            single  => is_match_single(s, single, p, dp),
        };

        dp[p.len()][s.len()] = Some(res);

        res
    }

    fn is_match_single(s: &[u8], token: Pattern, p: &[u8], dp: &mut Vec<Vec<Option<bool>>>) -> bool {
        match dp[p.len()][s.len()] {
            Some(b) => return b,
            _ => ()
        };

        match s.split_first() {
            Some((head_s, tail_s)) => match token {
                Pattern::Single(c) if *head_s == c => is_match_bytes(tail_s, &p[1..], dp),
                Pattern::SingleAny => is_match_bytes(tail_s, &p[1..], dp),
                _ => false
            },
            None => p.is_empty()
        }
    }

    fn parse(pattern: &[u8]) -> Pattern{
        match pattern.split_first() {
            Some((b'?', _)) => Pattern::SingleAny,
            Some((b'*', _)) => Pattern::ManyOrNone,
            Some((c, _)) => Pattern::Single(*c),
            _ => (Pattern::Empty)
        }
    }

    return is_match_bytes(&s.as_bytes(), &p.as_bytes(), dp)
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
