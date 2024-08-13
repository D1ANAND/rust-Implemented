extern crate flate2;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;



fn main() {

    if args().len() !=3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap()); // takes imput by reading the source file and its content

    let output = File::create(args().nth(2).unwrap()).unwrap(); // creates new target file

    let mut encoder = GzEncoder::new(output, Compression::default()); // creates a new gzip encoder instance and writes to the target file with default compression lvl.
    
    let start = Instant::now(); // creating timestamp for measuring time taken for compression 

    copy(&mut input, &mut encoder).unwrap(); // performs the copy operation from source to target

    let output = encoder.finish().unwrap(); // finish compression returns output

    println!("Source len: {:?}",input.get_ref().metadata().unwrap().len());

    println!("Target len: {:?}", output.metadata().unwrap().len());
    
    println!("Elapsed: {:?}", start.elapsed());



}
