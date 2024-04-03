use argh::FromArgs;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
#[cfg(test)]
mod tests;

pub const SQRT_3: f64 = 1.732_050_807_568_877_2_f64;
pub const SQRT_5: f64 = 2.236_067_977_499_79_f64;
pub const NUCL: [char; 4] = ['A', 'T', 'G', 'C'];

#[derive(Debug, FromArgs)]
/// Remove duplicate lines from .lines files contatining DNA.
///
/// The algorithm computes a hash for each sequence in the files that it is only
/// guaranteed for sequences of the same length.
pub struct Args {
    /// path to input directory containing .lines files to process.
    #[argh(positional)]
    input_dir: PathBuf,

    /// path to output directory.
    #[argh(positional)]
    pub output_dir: PathBuf,

    /// an optional lenght of the slice that will be used to compute the hash of each sequence. The length of the first sequence by default.
    #[argh(option)]
    length: Option<usize>,
}

fn main() {
    let mut args: Args = argh::from_env();
    // create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output_dir).unwrap();
    // iterate over files in input directory
    let mut truth_set = HashSet::new();
    for entry in std::fs::read_dir(args.input_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        // check extension is correct
        if path.extension().map(|ext| ext != "lines").unwrap_or(true) {
            continue;
        }
        // create new file in output dir
        let output_path = args.output_dir.join(path.file_name().unwrap());
        let num_removed = process_file(&path, &output_path, &mut truth_set, &mut args.length);
        eprintln!("Removed {num_removed} sequences from file {path:?}.")
    }
}

fn process_file(
    path: &Path,
    output_path: &Path,
    truth_set: &mut HashSet<OrderedFloat<f64>>,
    length: &mut Option<usize>,
) -> i32 {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::with_capacity(1000000000, file);
    let mut writer = io::BufWriter::new(File::create(output_path).unwrap());
    let mut num_removed = 0;
    for line in reader.lines() {
        let mut line = line.unwrap();
        if length.is_none() {
            *length = Some(line.len());
        }
        let score_identifier = OrderedFloat::from(get_score_identifier(&line[0..length.unwrap()]));
        if truth_set.insert(score_identifier) {
            // write to output buffer
            line.push('\n');
            writer.write_all(line.as_bytes()).expect("should be alrite");
        } else {
            num_removed += 1;
        }
    }
    writer.flush().unwrap();
    num_removed
}

fn get_score_identifier(line: &str) -> f64 {
    let prod = line.chars().cartesian_product(NUCL).fold(
        [[1.0, 0.0], [0.0, 1.0]],
        |mut prod, (c, base)| {
            if c == base {
                prod[0][1] += prod[0][0];
                prod[1][1] += prod[1][0];
            } else {
                prod[0][0] += prod[0][1];
                prod[1][0] += prod[1][1];
            }
            prod
        },
    );
    prod[0][0] + SQRT_3 * prod[0][1] + std::f64::consts::SQRT_2 * prod[1][0] + SQRT_5 * prod[1][1]
}
