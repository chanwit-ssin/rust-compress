use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `targat`");
        return;
    }

    let (input_path, output_path) = (args().nth(1).unwrap(), args().nth(2).unwrap());
    let mut input = BufReader::new(File::open(input_path).unwrap());
    let output = File::create(output_path).unwrap();

    let mut encoder = ZlibEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}
