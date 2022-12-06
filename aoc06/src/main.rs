use std::io::{self, Read};

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    println!("Part 1: {}", start_of_packet(&buf, 4));
    println!("Part 2: {}", start_of_packet(&buf, 14));

    Ok(())
}

fn start_of_packet(data: &[u8], marker_len: usize) -> usize {
    for (i, seq) in data.windows(marker_len).enumerate() {
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
