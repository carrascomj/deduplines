Based on https://github.com/daihang16/nubeamdedup/,
will dedup a file with lines of DNA sequences.

# Usage

`deduplines --help`:

```
Usage: deduplines <input_dir> <output_dir> [--length <length>]

Remove duplicate lines from .lines files contatining DNA. The algorithm computes a hash
for each sequence in the files that it is only guaranteed for sequences of the same length.

Positional Arguments:
  input_dir         path to input directory containing .lines files to process.
  output_dir        path to output directory.

Options:
  --length          an optional lenght of the slice that will be used to compute
                    the hash of each sequence. The length of the first sequence
                    by default.
  --help            display usage information
```

# Installation

The first step is to [install Rust](https://www.rust-lang.org/tools/install):

```bash
# Unix-like OS
curl https://sh.rustup.rs -sSf | sh
```

After cloning this repository, it can be installed through [cargo](https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html):

```bash
git clone https://github.com/carrascomj/deduplines.git
cd deduplines
cargo install --path .
```
