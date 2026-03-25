fn cpf_format(cpf: &str) -> bool {
    let chars: Vec<char> = cpf.chars().collect();

    if chars.len() != 14 {
        return false;
    }

    for (i, c) in chars.iter().enumerate() {
        match i {
            3 | 7 => {
                if *c != '.' {
                    return false;
                }
            }

            11 => {
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

pub fn validate_cpf(cpf: &str) -> bool {
    if !cpf_format(cpf) {
        return false;
    }

    let digits: Vec<u8> = cpf
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let first_check_digit = &digits[9];
    let second_check_digit = &digits[10];

    if digits.iter().all(|&d| d == digits[0]) {
        return false;
    }

    let mut sum = 0;
    for (i, v) in (2..=10).rev().enumerate() {
        sum += digits[i] * v;
    }

    let mut calculated_first_check_digit = (sum * 10) % 11;

    if calculated_first_check_digit == 10 {
        calculated_first_check_digit = 0;
    }

    if calculated_first_check_digit != *first_check_digit {
        return false;
    }

    let mut sum = 0;

    for (i, v) in (2..=11).rev().enumerate() {
        sum += digits[i] * v;
    }

    let mut calculated_second_check_digit = (sum * 10) % 11;

    if calculated_second_check_digit == 10 {
        calculated_second_check_digit = 0;
    }

    if calculated_second_check_digit != *second_check_digit {
        return false;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_check_if_cpf_format_is_valid() {
        assert!(cpf_format("123.456.789-09"));
    }

    #[test]
    fn should_check_if_cpf_format_is_invalid() {
        assert!(!cpf_format("12345678909"));
        assert!(!cpf_format("123.456.78909"));
        assert!(!cpf_format("123.456.789-0"));
        assert!(!cpf_format("123.456.789-0a"));
    }

    #[test]
    fn should_validate_a_valid_cpf() {
        assert!(validate_cpf("529.982.247-25"));
    }

    #[test]
    fn should_invalidate_an_invalid_cpf() {
        assert!(!validate_cpf("123.456.789-00"));
        assert!(!validate_cpf("111.111.111-11"));
    }
}
