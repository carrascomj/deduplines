use argh::FromArgs;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

pub const SQRT_3: f64 = 1.732_050_807_568_877_2_f64;
pub const SQRT_5: f64 = 2.236_067_977_499_79_f64;

#[derive(Debug, FromArgs)]
/// Remove duplicate lines from .lines files contatining DNA.
pub struct Args {
    /// path to input directory containing .lines files to process.
    #[argh(positional)]
    input_dir: PathBuf,

    /// path to output directory.
    #[argh(positional)]
    pub output_dir: PathBuf,
}

fn main() {
    let args: Args = argh::from_env();
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
        let num_removed = process_file(&path, &output_path, &mut truth_set);
        eprintln!("Removed {num_removed} sequences from file {path:?}.")
    }
}

fn process_file(
    path: &Path,
    output_path: &Path,
    truth_set: &mut HashSet<OrderedFloat<f64>>,
) -> i32 {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::with_capacity(1000000000, file);
    let mut writer = io::BufWriter::new(File::create(output_path).unwrap());
    let mut num_removed = 0;
    for line in reader.lines() {
        let mut line = line.unwrap();
        let score_identifier = OrderedFloat::from(get_score_identifier(&line));
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
    let prod = line.chars().cartesian_product(['A', 'C', 'G', 'T']).fold(
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
