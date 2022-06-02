/* Greatest Common Divisor */

// Euclidean subtraction method.
pub fn gcd_suboptimal(mut a: u32, mut b: u32) -> u32 {
    if a == b {
        return a;
    }

    loop {
        if b > a {
            (a, b) = (b, a);
        };

        let result: u32 = a - b;

        if b == result {
            return result;
        } else {
            a = b;
            b = result;
        }
    }
}

// Euclidean division with remainder method.
pub fn gcd_optimal(mut a: u32, mut b: u32) -> u32 {
    let mut tmp: u32;

    while b > 0 {
        tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

#[cfg(test)]
mod testing {
    use super::{gcd_optimal, gcd_suboptimal};

    #[test]
    fn from_data() {
        GCD_TEST_DATA.iter().for_each(|item| {
            assert_eq!(gcd_optimal(item.0, item.1), item.2);
            assert_eq!(gcd_suboptimal(item.0, item.1), item.2);
        });
    }

    const GCD_TEST_DATA: &[(u32, u32, u32); 6] = &[
        (1, 1, 1),
        (1, 2, 1),
        (12, 18, 6),
        (11, 12, 1),
        (36, 26, 2),
        (1024, 2048, 1024),
    ];
}
