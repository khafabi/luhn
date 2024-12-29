pub fn is_valid(code: &str) -> bool {
    let filtered: String = code.chars().filter(|c| !c.is_whitespace()).collect();

    if filtered.len() <= 1 {
        return false;
    }

    if !filtered.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    let digits: Vec<u32> = filtered
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .collect();


    let total = luhn_sum(&digits, false);


    total % 10 == 0
}

fn luhn_sum(digits: &[u32], should_double: bool) -> u32 {
    match digits {
        [] => 0,
        [first, rest @ ..] => {
            let val = if should_double {
                let double = first * 2;
                if double > 9 {
                    double - 9
                } else {
                    double
                }
            } else {
                *first
            };

            val + luhn_sum(rest, !should_double)
        }
    }
}
