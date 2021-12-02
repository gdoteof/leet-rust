pub fn is_palindrome(x: i32) -> bool {
    if x < 0 { return false; }
    let s:String = x.to_string();
    return chop(&s);
}

fn chop(s: &str) -> bool {
    if s.len() == 1 {
        return true;
    } else if s.as_bytes()[0] == s.as_bytes()[s.len()-1] {
        if s.len() == 2 { return true; }
        else { 
            return chop(&s[1..s.len()-1])
        }
    } else {
        return false;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn should_be_false() {
        assert_eq!(is_palindrome(100), false);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(1000021), false);
    }

    #[test]
    fn negative_numbers_false() {
        assert_eq!(is_palindrome(-10), false);
        assert_eq!(is_palindrome(-121), false);
    }
}
