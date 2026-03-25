fn correct_cpf_mask(cpf: &str) -> bool {
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
    if !correct_cpf_mask(cpf) {
        return false;
    }

    let digits: Vec<usize> = cpf
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap() as usize)
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

    const VALID_CPF_LIST: [&str; 50] = [
        "444.896.358-69",
        "914.845.392-73",
        "913.483.331-54",
        "492.599.571-50",
        "171.526.732-09",
        "978.865.395-24",
        "276.288.342-30",
        "352.714.754-33",
        "344.834.812-07",
        "828.529.628-88",
        "082.286.187-94",
        "401.990.158-99",
        "936.574.116-58",
        "525.775.417-75",
        "339.826.052-70",
        "531.251.469-17",
        "736.617.356-83",
        "917.402.059-52",
        "608.601.076-55",
        "574.222.585-78",
        "111.939.160-11",
        "963.987.469-88",
        "127.944.651-04",
        "926.236.677-99",
        "231.607.564-91",
        "569.361.176-00",
        "962.178.346-18",
        "753.378.198-87",
        "785.793.479-80",
        "717.209.760-23",
        "750.430.126-41",
        "871.123.095-90",
        "105.530.896-25",
        "301.178.790-53",
        "770.672.300-39",
        "140.215.108-00",
        "793.886.352-13",
        "692.008.527-50",
        "511.248.128-57",
        "858.600.401-48",
        "495.877.823-30",
        "498.130.427-70",
        "954.106.453-16",
        "448.592.374-63",
        "713.236.042-60",
        "563.228.693-29",
        "279.623.471-12",
        "031.935.306-03",
        "637.208.006-01",
        "130.503.531-39",
    ];

    const INVALID_CPF_LIST: [&str; 18] = [
        "111.111.111-11",
        "222.222.222-22",
        "333.333.333-33",
        "444.444.444-44",
        "555.555.555-55",
        "666.666.666-66",
        "777.777.777-77",
        "888.888.888-88",
        "999.999.999-99",
        "123.456.789-00",
        "123.456.789-01",
        "123.456.789-02",
        "123.456.789-03",
        "123.456.789-04",
        "123.456.789-05",
        "123.456.789-06",
        "123.456.789-07",
        "123.456.789-08",
    ];

    #[test]
    fn should_check_if_cpf_format_is_valid() {
        for cpf in VALID_CPF_LIST.iter() {
            assert!(correct_cpf_mask(cpf));
        }
    }

    #[test]
    fn should_check_if_cpf_format_is_invalid() {
        assert!(!correct_cpf_mask("12345678909"));
        assert!(!correct_cpf_mask("123.456.78909"));
        assert!(!correct_cpf_mask("123.456.789-0"));
        assert!(!correct_cpf_mask("123.456.789-0a"));
    }

    #[test]
    fn should_validate_a_valid_cpf() {
        for cpf in VALID_CPF_LIST.iter() {
            assert!(validate_cpf(cpf));
        }
    }

    #[test]
    fn should_invalidate_an_invalid_cpf() {
        for cpf in INVALID_CPF_LIST.iter() {
            assert!(!validate_cpf(cpf));
        }
    }
}
