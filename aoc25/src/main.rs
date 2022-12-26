use std::io;

use anyhow::Result;

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines = input.lines().collect::<Vec<_>>();
    let sum: isize = lines.iter().map(|line| snafu_to_dec(line)).sum();
    println!("Part 1: {}", dec_to_snafu(sum));

    Ok(())
}

fn snafu_to_dec(snafu: &str) -> isize {
    let mut dec = 0;
    for c in snafu.chars() {
        let digit = snafu_digit(c);
        dec = dec * 5 + digit;
    }
    dec
}

fn snafu_digit(c: char) -> isize {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Invalid digit"),
    }
}

fn dec_to_snafu(dec: isize) -> String {
    let mut snafu = String::new();
    if dec == 0 {
        return "0".to_string();
    }
    let mut dec = dec;
    while dec > 0 {
        let digit = (dec + 2) % 5 - 2;
        let c = dec_digit(digit);
        snafu.push(c);
        dec = (dec + 2) / 5;
    }
    snafu.chars().rev().collect()
}

fn dec_digit(digit: isize) -> char {
    match digit {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!("Invalid digit"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dec_to_snafu() {
        let dec = super::dec_to_snafu(0);
        assert_eq!(&dec, "0");

        let dec = super::dec_to_snafu(1);
        assert_eq!(&dec, "1");

        let dec = super::dec_to_snafu(2);
        assert_eq!(&dec, "2");

        let dec = super::dec_to_snafu(3);
        assert_eq!(&dec, "1=");

        let dec = super::dec_to_snafu(1747);
        assert_eq!(&dec, "1=-0-2");
    }
}
