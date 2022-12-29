use std::io::{self, Read};

use anyhow::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", start_of_packet(input.as_bytes(), 4));
    println!("Part 2: {}", start_of_packet(input.as_bytes(), 14));
    Ok(())
}

fn start_of_packet(input: &[u8], marker_len: usize) -> usize {
    for (i, seq) in input.windows(marker_len).enumerate() {
        let set = seq
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>();
        if set.len() == marker_len {
            return i + marker_len;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        const INPUT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        const INPUT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        const INPUT4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        const INPUT5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(start_of_packet(INPUT1.as_bytes(), 14), 19);

        assert_eq!(start_of_packet(INPUT2.as_bytes(), 4), 5);
        assert_eq!(start_of_packet(INPUT2.as_bytes(), 14), 23);

        assert_eq!(start_of_packet(INPUT3.as_bytes(), 4), 6);
        assert_eq!(start_of_packet(INPUT3.as_bytes(), 14), 23);

        assert_eq!(start_of_packet(INPUT4.as_bytes(), 4), 10);
        assert_eq!(start_of_packet(INPUT4.as_bytes(), 14), 29);

        assert_eq!(start_of_packet(INPUT5.as_bytes(), 4), 11);
        assert_eq!(start_of_packet(INPUT5.as_bytes(), 14), 26);
    }
}
