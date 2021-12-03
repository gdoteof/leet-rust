
pub fn is_match(s: String, p: String) -> bool {
    enum Pattern{
        SingleAny,   // "?"
        ManyOrNone,  // "*"
        Single(u8),   
        Empty
    }
    type DpCacheMatrix = Vec<Vec<Option<bool>>>;

    // a p.len()+1 x s.len()+1 cache
    // the value at dp[p.len()][s.len()] corresponds to the output of is_match_bytes(s,p)
    let dp:&mut DpCacheMatrix = &mut vec!(vec!(None; s.len() + 1); p.len() + 1);
    
    
    fn is_match_bytes(s: &[u8], p: &[u8], dp: &mut DpCacheMatrix) -> bool {
        // println!("S:{:?}, P:{:?}", std::str::from_utf8(s),  std::str::from_utf8(p));

        //Return cached result if we have it
        match dp[p.len()][s.len()] {
            Some(b) => return b,
            _ => ()
        };

        //using parse + enum makes this code more readable imo

        let res = match parse(p){
            //If the pattern is empty, it's a match only if the string is empty
            Pattern::Empty => s.is_empty(),  

            //If the Pattern is "*" and the string is empty, it might be *'s all the way down.
            Pattern::ManyOrNone if s.is_empty() => is_match_bytes(s, &p[1..], dp), 

            //If the pattern is "*" and string is not empty...
            Pattern::ManyOrNone => 
                //... "*" may correspond to exactly one item in string
                is_match_single(s, Pattern::SingleAny, p, dp) 
                //... "*" may correspond to many items in the string
                || is_match_bytes(&s[1..], p, dp) 
                //... "*" may correspond to no items in the string
                || is_match_bytes(s, &p[1..], dp),

            // If the Pattern is not a "*" it is a single letter
            single  => is_match_single(s, single, p, dp),

        };

        //set our cache
        dp[p.len()][s.len()] = Some(res);

        res
    }

    fn is_match_single(s: &[u8], token: Pattern, p: &[u8], dp: &mut DpCacheMatrix) -> bool {
        //return cached result if we have it
        match dp[p.len()][s.len()] {
            Some(b) => return b,
            _ => ()
        };

        //Split s into the first char and everything else
        //this function always consumes the first item of both p and s
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
        //Simply map the first character in the pattern to our enum
        match pattern.first() {
            Some(b'?') => Pattern::SingleAny,
            Some(b'*') => Pattern::ManyOrNone,
            Some(c) => Pattern::Single(*c),
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
