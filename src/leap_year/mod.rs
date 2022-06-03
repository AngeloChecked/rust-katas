fn leap_year(year: u64) -> bool {
    match year {
        year if year % 100 == 0 => { if year % 400 != 0 { false } else { true } }
        year if year % 4 == 0 => { true }
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::leap_year;

    #[test]
    fn year_2001_is_a_typical_common_year() {
        let result = leap_year(2001);
        assert_eq!(result, false)
    }

    #[test]
    fn year_2002_is_typical_common_year() {
        let result = leap_year(2002);
        assert_eq!(result, false)
    }

    #[test]
    fn year_2003_is_typical_common_year() {
        let result = leap_year(2003);
        assert_eq!(result, false)
    }

    #[test]
    fn year_1996_is_typical_leap_year() {
        let result = leap_year(1996);
        assert_eq!(result, true)
    }

    #[test]
    fn year_1992_is_typical_leap_year() {
        let result = leap_year(1992);
        assert_eq!(result, true)
    }

    #[test]
    fn year_1988_is_typical_leap_year() {
        let result = leap_year(1988);
        assert_eq!(result, true)
    }

    #[test]
    fn year_1900_is_atypical_common_year() {
        let result = leap_year(1900);
        assert_eq!(result, false)
    }

    #[test]
    fn year_1800_is_atypical_common_year() {
        let result = leap_year(1800);
        assert_eq!(result, false)
    }

    #[test]
    fn year_1700_is_atypical_common_year() {
        let result = leap_year(1700);
        assert_eq!(result, false)
    }

    #[test]
    fn year_2000_is_atypical_leap_year() {
        let result = leap_year(2000);
        assert_eq!(result, true)
    }

    #[test]
    fn year_1600_is_atypical_leap_year() {
        let result = leap_year(1600);
        assert_eq!(result, true)
    }

    #[test]
    fn year_1200_is_atypical_leap_year() {
        let result = leap_year(1200);
        assert_eq!(result, true)
    }
}
