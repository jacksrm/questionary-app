pub fn validate_phone(phone: &str) -> bool {
    let chars: Vec<char> = phone.chars().collect();

    if chars.len() != 15 {
        return false;
    }

    for (i, c) in chars.iter().enumerate() {
        match i {
            0 => {
                if *c != '(' {
                    return false;
                }
            }

            3 => {
                if *c != ')' {
                    return false;
                }
            }

            4 => {
                if *c != ' ' {
                    return false;
                }
            }

            10 => {
                if *c != '-' {
                    return false;
                }
            }

            _ => {
                if !c.is_ascii_digit() {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_check_if_phone_format_is_valid() {
        assert!(validate_phone("(85) 98765-4321"));
    }

    #[test]
    fn should_check_if_phone_format_is_invalid() {
        assert!(!validate_phone("85 98765-4321"));
        assert!(!validate_phone("(85)98765-4321"));
        assert!(!validate_phone("(85) 987654321"));
        assert!(!validate_phone("(85) 98765-432a"));
    }
}
