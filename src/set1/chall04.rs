use crate::util;

use super::chall03;

pub fn solve_chall04() {
    let input = util::read_hex_lines_stdin();
    let (key, msg) = find_single_byte_encryption(&input).expect("No single-byte encryption found");
    println!("Key: {key:#02x} ({})", char::from(key));
    println!("Message: {}", util::bytes_to_hex(&msg));
    println!(
        "ASCII: {}",
        msg.iter().cloned().map(char::from).collect::<String>()
    );
}

fn find_single_byte_encryption(seqs: &[Vec<u8>]) -> Option<(u8, Vec<u8>)> {
    let mut top_candidate = None;

    for seq in seqs {
        let result = chall03::find_single_byte_key(seq);
        if let Some((score, _, _)) = result {
            match top_candidate {
                Some((top_score, _, _)) if top_score > score => {} // ignore
                _ => top_candidate = result,
            }
        }
    }

    top_candidate.map(|(_, key, msg)| (key, msg))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let input = util::read_hex_lines_file("data/4.txt");
        let (key, msg) =
            find_single_byte_encryption(&input).expect("No single-byte encryption found");
        assert_eq!("35", util::bytes_to_hex(&[key]));
        assert_eq!("Now that the party is jumping\n".as_bytes(), msg);
    }
}
