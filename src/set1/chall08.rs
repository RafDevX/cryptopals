use std::collections::HashMap;

use crate::util;

pub fn solve_chall08() {
    let input = util::read_hex_lines_stdin();
    let detected = detect_aes_ecb(&input).expect("None found");

    println!("Found ECB: {}", util::bytes_to_hex(detected));
}

fn detect_aes_ecb(candidates: &[Vec<u8>]) -> Option<&Vec<u8>> {
    // "the same 16 byte plaintext block will always produce the same 16 byte ciphertext"

    let mut top_candidate = None;
    let mut top_score = 0;

    for candidate in candidates {
        let mut dup_counters = HashMap::new();
        for val in candidate {
            dup_counters
                .entry(*val)
                .and_modify(|c| *c += 1)
                .or_insert(0);
        }
        let score = dup_counters.values().sum();

        if score > top_score {
            top_candidate = Some(candidate);
            top_score = score;
        }
    }

    top_candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let input = util::read_hex_lines_file("data/8.txt");
        let detected = detect_aes_ecb(&input).expect("None found");

        assert_eq!(
            "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a",
            util::bytes_to_hex(detected)
        );
    }
}
