#[allow(dead_code)]
pub fn my_atoi(s: String) -> i32 {
    //Ignore leading white space
    let mut chars = s.trim_start().chars().peekable();

    //Check for sign
    let negative = match chars.peek() {
        Some('+') => { chars.next(); false },
        Some('-') => { chars.next(); true },
        _ => false
    };

    //Accumulate as negative and convert back at the end if we need to
    let decimal_base:u32 = 10;
    let mut val:i32 = 0;
    for c in chars {
        val = match c.to_digit(decimal_base) {
            Some(digit) => val.saturating_mul(decimal_base as i32).saturating_sub(digit as i32),
            _ => break
        }
    }

    //Converting back
    if !negative { val = val.saturating_neg(); }

    return val;

}

#[allow(dead_code)]
pub fn my_atoi_bytes(s: String) -> i32 {
    let ss = s.as_bytes();
    let len = s.len();

    let mut sign = b'+';

    let mut val:i32 = 0;

    let mut i = 0;

    // Ignore leading white space
    while i < len && ss[i] == b' ' { i += 1; }

    // Check for a sign
    if i < len && (ss[i] == b'+' || ss[i] == b'-'){
        sign = ss[i];
        i += 1;
    }

    //Accumulate as if it were negative 
    while i < len && ss[i] >= b'0' && ss[i] <= b'9' {
        val = val.saturating_mul(10)
            .saturating_sub((ss[i] - b'0') as i32);
        i += 1;
    }

    //If positive, negate
    if sign == b'+' {
        val = val.saturating_neg();
    }

    return val;
}

#[allow(dead_code)]
pub fn my_atoi_noob(s: String) -> i32 {
    if s.len() == 0 {return 0};
    let mut leading_spaces_flag:bool = false;
    let mut sign_flag:bool = false;
    let mut number_started_flag:bool = false;

    let mut current = 0;
    let mut start = 0;
    let mut end = s.len() - 1;

    let mut sign:i32 = 1;

    let mut accum:i32 = 0;

    let mut any_digits = false;

    while current <= s.len() -1 {
        let character = match s.chars().nth(current) {
            Some(c) => c,
            None => '.'
        };


        // 1. Read in and ignore any leading white space
        if !leading_spaces_flag{
            if character == ' ' {
                current += 1;
                continue;
            }
            leading_spaces_flag = true;
        }


        // 2. Check if next char is a sign

        if !sign_flag {
            if character == '-' {
                sign = -1;
                current += 1;
            } else if character == '+' {
                current += 1;
            } // we don't move forward if it's neither
            sign_flag = true;
            continue;
        }

        // 3. Read characters until a non-digit, or end of string is reached
        if !number_started_flag{
            if character == '0' {
                current += 1;
                continue;
            }
            start = current;
            number_started_flag = true;
        }

        if character.is_digit(10) {
            current += 1;
            any_digits = true;
        } else {
            end = current - 1;
            break;
        }
    }

    if !any_digits { return 0; }

    let num_slice = &s[start..end+1].to_string().chars().rev().collect::<String>();
    for (i, x) in num_slice.chars().enumerate() {
        let base = i as u32;
        let (mult, overflowed) = (10 as i32).overflowing_pow(base);
        if overflowed { accum = if sign == 1 {i32::MAX} else {return i32::MIN;}; break}
        let significant_digit = x.to_digit(10).unwrap() as i32;
        let slot = match significant_digit.checked_mul(mult) {
            Some(val) => val,
            None => { accum = if sign == 1 {i32::MAX} else {return i32::MIN;}; break; }
        };
        accum = match accum.checked_add(slot) {
            Some(val) => val,
            None => if sign == 1 {i32::MAX} else { return i32::MIN }
        };
    }

    return accum * sign;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testit(s:&str, e:i32){
        assert_eq!(my_atoi(s.to_string()), e);
    }

    #[test]
    fn easy() {
        testit("123", 123);
        testit("  -123", -123);
        testit("-0123", -123);
    }

    #[test]
    fn negative_numbers() {
        testit("-123abc", -123);
    }

    #[test]
    fn positive_numbers() {
        testit("  123", 123);
        testit(" 000000000000000123", 123);
        testit(" 0123abc12", 123);
    }

    #[test]
    fn big_numbers(){
        testit("-91283472332", i32::MIN);
        testit("+91283472332", i32::MAX);
    }

    #[test]
    fn dumb_zeroes(){
        testit("", 0);
        testit("+", 0 );
        testit("-", 0 );
    }

}
