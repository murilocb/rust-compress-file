extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, copy};
use std::time::Instant;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <source> <target>", args[0]);
        return Ok(());
    }

    let source_path = &args[1];
    let target_path = &args[2];

    let input_file = File::open(source_path)
    .unwrap_or_else(|_| panic!("Failed to open source file: {}", source_path));
    let output_file = File::create(target_path)
    .unwrap_or_else(|_| panic!("Failed to create target file: {}", target_path));

    let mut input = BufReader::new(input_file);
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder)
    .unwrap_or_else(|e| panic!("Failed to compress file: {}", e));

    let output_file = encoder.finish()
    .unwrap_or_else(|e| panic!("Failed to finish compression: {}", e));

    println!(
        "Source len: {} bytes",
        input.get_ref().metadata().unwrap().len()
    );
    println!(
        "Target len: {} bytes",
        output_file.metadata().unwrap().len()
    );
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
