use std::cmp::Ordering;

pub fn ace_cmp(mut a: &str, mut b: &str) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }

    // Fast check if either are empty string
    match (a.is_empty(), b.is_empty()) {
        (true, false) => return Ordering::Less,
        (false, true) => return Ordering::Greater,
        (true, true) => unreachable!(), // Because `a == b` test above would have returned
        (false, false) => {}
    }

    while !a.is_empty() || !b.is_empty() {
        let a_digit_start = a
            .chars()
            .position(|c| c.is_ascii_digit())
            .unwrap_or(a.len());
        let b_digit_start = b
            .chars()
            .position(|c| c.is_ascii_digit())
            .unwrap_or(b.len());

        // Compare strings up to first digit
        let a_str = &a[..a_digit_start];
        let b_str = &b[..b_digit_start];
        match a_str.cmp(b_str) {
            Ordering::Equal => {}
            ord => return ord,
        }

        // Move slices to start of first digit
        a = &a[a_digit_start..];
        b = &b[b_digit_start..];

        let a_digit_end = a
            .chars()
            .position(|c| !c.is_ascii_digit())
            .unwrap_or(a.len());
        let b_digit_end = b
            .chars()
            .position(|c| !c.is_ascii_digit())
            .unwrap_or(b.len());

        // Find any leading zeroes
        let a_first_non_zero = a.chars().position(|c| c != '0').unwrap_or(a.len());
        let b_first_non_zero = b.chars().position(|c| c != '0').unwrap_or(b.len());

        // Get the string of digits to compare
        let a_str = &a[a_first_non_zero..a_digit_end];
        let b_str = &b[b_first_non_zero..b_digit_end];

        // Longer strings of digits, after any leading zeroes have been
        // removed, are bigger numbers
        match a_str.len().cmp(&b_str.len()) {
            Ordering::Equal => {}
            ord => return ord,
        }

        // Lexicographical order is the same as numeric order when comparing
        // two strings of digits of equal length
        match a_str.cmp(b_str) {
            Ordering::Equal => {}
            ord => return ord,
        }

        // Use length of any strings of zeroes to break ties
        match a_first_non_zero.cmp(&b_first_non_zero) {
            Ordering::Equal => {}
            ord => return ord,
        }

        // Move slices to end of digit sequence
        a = &a[a_digit_end..];
        b = &b[b_digit_end..];
    }

    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ace_cmp() {
        for (a, b, ord) in [
            ("", "", Ordering::Equal),
            ("", "x", Ordering::Less),
            ("x", "", Ordering::Greater),
            ("x", "x", Ordering::Equal),
            ("x", "y", Ordering::Less),
            ("x2", "x10", Ordering::Less),
            ("x10", "x2", Ordering::Greater),
            ("2", "x2", Ordering::Less),
            ("x00", "x02", Ordering::Less),
            ("x002", "x02", Ordering::Greater), // Longer string of zeroes is bigger
            ("x02", "x002", Ordering::Less),
            ("x02a", "x02b", Ordering::Less),
            ("x02a", "x02b", Ordering::Less),
            ("3.14", "3.015", Ordering::Less), // `ace_cmp()` will not generate correct float ordering
            ("001", "02", Ordering::Less),
            ("1002", "102", Ordering::Greater),
        ] {
            eprintln!("\nComparing '{a}' <=> '{b}' expecting {ord:?}");
            assert_eq!(ace_cmp(a, b), ord);
        }
    }
}
