use super::{get_score_identifier, Itertools};
use bitvec::{prelude::*, view::BitViewSized};
use rand::{self, rngs::ThreadRng, RngCore};

fn gen_random_dna(rng: &mut ThreadRng, length: usize) -> String {
    (0..(length / (32 / 2)))
        .into_iter()
        .map(|_| rng.next_u32())
        .flat_map(|num| num.into_bitarray::<Lsb0>().into_iter())
        .chunks(2)
        .into_iter()
        .map(|bitchunk| {
            let mut bits = bitchunk.take(2);
            let a = bits.next();
            let b = bits.next();
            match (a, b) {
                (Some(true), Some(true)) => 'C',
                (Some(true), Some(false)) => 'A',
                (Some(false), Some(true)) => 'G',
                (Some(false), Some(false)) => 'T',
                _ => 'N',
            }
        })
        .collect()
}

fn gen_other_seq(rng: &mut ThreadRng, seq: &str) -> String {
    let seq_candidate = gen_random_dna(rng, 200);
    if seq_candidate != seq {
        seq_candidate
    } else {
        gen_other_seq(rng, seq)
    }
}

#[test]
fn same_sequence_is_the_same() {
    let mut rng = rand::thread_rng();
    let seq1: String = gen_random_dna(&mut rng, 200);
    assert_eq!(
        get_score_identifier(seq1.as_str()),
        get_score_identifier(seq1.as_str())
    );
}

#[test]
fn diff_sequences_are_the_diff() {
    let mut rng = rand::thread_rng();
    let seq1: String = gen_random_dna(&mut rng, 200);
    let seq2: String = gen_other_seq(&mut rng, seq1.as_str());
    assert_ne!(
        get_score_identifier(seq1.as_str()),
        get_score_identifier(seq2.as_str())
    );
}
