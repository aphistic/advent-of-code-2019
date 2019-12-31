pub fn possible_password(password: &str) -> bool {
    let mut has_double = false;

    let mut repeated = 0;
    let mut last_digit = None;

    for raw_digit in password.bytes() {
        let digit = raw_digit - 48;

        match last_digit {
            Some(last) => {
                if digit == last {
                    repeated += 1;
                } else {
                    if repeated == 1 {
                        has_double = true
                    }
                    repeated = 0;
                }

                if last > digit {
                    return false
                }

                last_digit = Some(digit);
            },
            None => last_digit = Some(digit),
        }
    }
    if repeated == 1 {
        // cover cases where the double is at the end of the password
        has_double = true
    }

    has_double
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(false, possible_password("111111"))
    }

    #[test]
    fn example2() {
        assert_eq!(false, possible_password("223450"))
    }

    #[test]
    fn example3() {
        assert_eq!(false, possible_password("123789"))
    }

    #[test]
    fn example4() {
        assert_eq!(true, possible_password("112233"))
    }

    #[test]
    fn example5() {
        assert_eq!(false, possible_password("123444"))
    }

    #[test]
    fn example6() {
        assert_eq!(true, possible_password("111122"))
    }

    #[test]
    fn possible_password_wrong() {
        assert_eq!(false, possible_password("669997"))
    }
}