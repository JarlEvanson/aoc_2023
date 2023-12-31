use std::num::Wrapping;

pub fn gcd_single(mut u: usize, mut v: usize) -> usize {
    if u == 0 {
        return v;
    }

    if v == 0 {
        return u;
    }

    let gcd_exponent_on_two = (u | v).trailing_zeros();

    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    while u != v {
        if u < v {
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    u << gcd_exponent_on_two
}

pub fn lcm_single(u: usize, v: usize) -> usize {
    let gcd = gcd_single(u, v);

    u * (v / gcd)
}

pub fn lcm(numbers: &mut [usize]) -> usize {
    let mut len = numbers.len();

    {
        let mut a_index = Wrapping(0usize);
        let mut b_index = Wrapping(1usize);

        let mut place_index = 0;

        while b_index.0 < len {
            numbers[place_index] = lcm_single(numbers[a_index.0], numbers[b_index.0]);

            if a_index + Wrapping(2) < a_index {
                break;
            }

            a_index += 2;
            b_index += 2;
            place_index += 1;
        }

        if len % 2 == 1 {
            numbers[len / 2] = numbers[len - 1];
        }

        len = len / 2 + len % 2;
    }

    while len > 1 {
        let mut a_index = Wrapping(0usize);
        let mut b_index = Wrapping(1usize);

        let mut place_index = 0;

        while b_index.0 < len {
            numbers[place_index] = lcm_single(numbers[a_index.0], numbers[b_index.0]);

            a_index += 2;
            b_index += 2;
            place_index += 1;
        }

        if len % 2 == 1 {
            numbers[len / 2] = numbers[len - 1];
        }

        len = len / 2 + len % 2;
    }

    numbers[0]
}
