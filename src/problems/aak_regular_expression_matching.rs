pub fn is_match(s: String, p: String) -> bool {
    fn is_match_str(s: &str, p: &str) -> bool {
       if s.len() == 0 && p.len() == 0 {
           return true;
       } 

       let mut p_chars = p.chars().peekable(); 
       println!("s:{}   p:{}", s, p);

       match p_chars.next() {
           Some(c) => {
               match p_chars.peek() {
                   Some('*') => {
                       if s.len() > 0 && (c == '.' || s.starts_with(c)){
                           //try all: 
                           // - consuming the char from s and keep the pattern
                           // - consuming the char from s and drop the pattern
                           // - NOT consuming the char from s and drop the pattern
                           is_match_str(&s[1..], p) || is_match_str(&s[1..], &p[2..]) || is_match_str(s, &p[2..])
                       }
                       else {
                           //no char to consume, so drop the <char>* from the p
                           is_match_str(s, &p[2..])
                       }
                   },
                   _ => {
                       if s.len() > 0 && (c == '.' || s.starts_with(c)){
                           //consume the char from s and p
                           is_match_str(&s[1..], &p[1..])
                       } else {
                           return false;
                       }
                   }
               }
           },
           _ => { return false; } //no pattern left, but still string
       }
    }

    is_match_str(&s, &p)
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
        testit("aa", "a*", true);
        testit("aa", "a*a", true);
        testit("ab", ".*p", false);
        testit("aa", "a", false);
        testit("aa", "a*", true);
        testit("ab", ".*", true);
        testit("aab", "c*a*b", true);
        testit("mississippi", "mis*is*p*", false);
        testit("ab", ".*p", false);
        testit("ab", ".*", true);
        testit("ab", ".*p", false);
        testit("mississippi", "mis*is*ip*.", true);
    }

    #[test]
    fn not_working() {
        testit("a", "a*a", true);
    }

    #[test]
    fn debug_tests() {
        testit("bbbba", ".*a*a", true);
    }

}
