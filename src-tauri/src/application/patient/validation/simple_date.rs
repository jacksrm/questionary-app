pub fn validate_simple_date(date: &str) -> bool {
    let chars: Vec<char> = date.chars().collect();

    if chars.len() != 10 {
        return false;
    }

    for (i, c) in chars.iter().enumerate() {
        match i {
            4 | 7 => {
                if *c != '-' && *c != '/' {
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

    if chars[4] != chars[7] {
        return false;
    }

    let year: usize = date[0..4].parse().unwrap();
    let month: usize = date[5..7].parse().unwrap();
    let day: usize = date[8..10].parse().unwrap();

    if year < 1900 || year > 2200 {
        return false;
    }

    if month == 0 || month > 12 {
        return false;
    }

    if day == 0 || day > 31 {
        return false;
    }

    if month == 2 {
        if is_leap_year(year) {
            return day <= 29;
        } else {
            return day <= 28;
        }
    }

    if [4, 6, 9, 11].contains(&month) {
        return day <= 30;
    }

    true
}

fn is_leap_year(year: usize) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_check_if_date_format_is_valid() {
        assert!(validate_simple_date("2024-01-01"));
        assert!(validate_simple_date("2024/01/01"));
    }

    #[test]
    fn should_check_if_date_format_is_invalid() {
        assert!(!validate_simple_date("2024-1-1"));
        assert!(!validate_simple_date("2024-01/01"));
        assert!(!validate_simple_date("2024/01-01"));
        assert!(!validate_simple_date("2024/1/1"));
        assert!(!validate_simple_date("2024-01-0a"));
        assert!(!validate_simple_date("2024-13-01"));
        assert!(!validate_simple_date("2024-00-01"));
        assert!(!validate_simple_date("2024-01-32"));
        assert!(!validate_simple_date("2024-01-00"));
        assert!(!validate_simple_date("2024-02-30"));
        assert!(!validate_simple_date("2023-02-29"));
        assert!(!validate_simple_date("2024-04-31"));
        assert!(!validate_simple_date("2024-06-31"));
        assert!(!validate_simple_date("2024-09-31"));
        assert!(!validate_simple_date("2024-11-31"));
        assert!(!validate_simple_date("1899-01-01"));
        assert!(!validate_simple_date("2210-01-01"));
    }
}
