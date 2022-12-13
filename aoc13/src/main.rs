use std::{cmp::Ordering, io};

use anyhow::Result;

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines = input.split('\n').collect::<Vec<_>>();

    let mut packets = Vec::new();
    let mut sum = 0;
    for (i, pair) in lines.chunks(3).enumerate() {
        packets.push(pair[0]);
        packets.push(pair[1]);

        let ord = cmp(pair[0].as_bytes(), pair[1].as_bytes());
        if ord == Ordering::Less {
            sum += i + 1;
        }
    }
    println!("Part 1: {}", sum);

    packets.push("[[2]]");
    packets.push("[[6]]");
    packets.sort_by(|a, b| cmp(a.as_bytes(), b.as_bytes()));
    let mut start = 0;
    let mut end = 0;
    for (i, packet) in packets.iter().enumerate() {
        if packet == &"[[2]]" {
            start = i + 1;
            continue;
        }
        if packet == &"[[6]]" {
            end = i + 1;
            break;
        }
    }
    println!("Part 2: {}", start * end);
    Ok(())
}

fn cmp(list1: &[u8], list2: &[u8]) -> Ordering {
    if list1.is_empty() && !list2.is_empty() {
        return Ordering::Less;
    }
    if !list1.is_empty() && list2.is_empty() {
        return Ordering::Greater;
    }
    if list1.is_empty() && list2.is_empty() {
        return Ordering::Equal;
    }

    if list1[0] == b'[' && list2[0] == b'[' {
        let list1 = split(list1);
        let list2 = split(list2);
        for (a, b) in list1.iter().zip(list2.iter()) {
            match cmp(a, b) {
                Ordering::Equal => continue,
                other => return other,
            }
        }
        list1.len().cmp(&list2.len())
    } else if list1[0] == b'[' {
        let val2 = format!("[{}]", std::str::from_utf8(list2).unwrap());
        cmp(list1, val2.as_bytes())
    } else if list2[0] == b'[' {
        let val1 = format!("[{}]", std::str::from_utf8(list1).unwrap());
        cmp(val1.as_bytes(), list2)
    } else {
        let val1 = String::from_utf8_lossy(list1).parse::<i32>().unwrap();
        let val2 = String::from_utf8_lossy(list2).parse::<i32>().unwrap();
        val1.cmp(&val2)
    }
}

fn split(s: &[u8]) -> Vec<&[u8]> {
    if s[0] == b'[' {
        let mut elems = Vec::new();
        let mut depth = 1;
        let mut start = 1;
        let mut i = 1;
        while depth != 1 || s[i] != b']' {
            if depth == 1 && s[i] == b',' {
                elems.push(&s[start..i]);
                start = i + 1;
            }

            if s[i] == b'[' {
                depth += 1;
            } else if s[i] == b']' {
                depth -= 1;
            }
            i += 1;
        }
        if start != i {
            elems.push(&s[start..i]);
        }
        elems
    } else {
        vec![s]
    }
}
