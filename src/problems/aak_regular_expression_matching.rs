#[derive(PartialEq)]
enum ConsumeMode {
    OneAtATime,
    ManyAtATime,
}

#[derive(PartialEq)]
enum CharMatchMode {
    AnyChar,
    SpecificChar,
}

pub fn is_match(s: String, p: String) -> bool {
    let s_bytes = s.as_bytes();
    let mut p_chars = p.chars().peekable();


    //index to traverse bytes of the string
    let mut i = 0; 

    while let Some(p_char) = p_chars.next() {
        let consume_mode = match p_chars.peek(){
            Some('*') => {p_chars.next(); ConsumeMode::ManyAtATime}, 
            _ => ConsumeMode::OneAtATime
        };

        let (char_match_mode, char_to_match) = match p_char {
            '.' => (CharMatchMode::AnyChar, '?'),
            c => (CharMatchMode::SpecificChar, c)
        };

        while i < s.len() {
            if consume_mode == ConsumeMode::ManyAtATime {
                if char_match_mode == CharMatchMode::SpecificChar{
                    if s_bytes[i] as char  == char_to_match {
                        i += 1;
                        continue;
                    } else {
                        i += 1;
                        break;
                    }
                } else {
                    i += 1;
                    continue;
                }
            }

            if consume_mode == ConsumeMode::OneAtATime {
                if s_bytes[i] as char  == char_to_match {
                    i += 1;
                    break;
                } else {
                    return false;
                }
            }

        }
    }
    return i == s.len();
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
        testit("aa", "a", false);
        testit("aa", "a*", true);
        testit("ab", ".*", true);
        testit("aab", "c*a*b", true);
        testit("mississippi", "mis*is*p*", false);
        testit("ab", ".*p", false);
    }

    #[test]
    fn not_working() {
    //    testit("mississippi", "mis*is*ip*.", true);
        testit("ab", ".*p", false);
    }

    #[test]
    fn debug_tests() {
        testit("ab", ".*", true);
    }

}
