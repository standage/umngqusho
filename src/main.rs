extern crate bio;
extern crate argparse;
use argparse::{ArgumentParser, Store};
use bio::io::fastq;
use std::fs::File;
use std::io;

fn main() {
    let mut infile = "-".to_string();
    let mut outfile = "-".to_string();
    let mut fhin;
    let mut fhout;
    let mut stdout;
    let mut stdin;
    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut outfile).add_option(&["-o", "--outfile"], Store, "output file");
        parser.refer(&mut infile).add_argument("seqs", Store, "sequences in Fastq format");
        parser.parse_args_or_exit();
    }

    let instream: &mut dyn std::io::Read = match infile.as_str() {
        "-" => {
            stdin = io::stdin();
            &mut stdin
        }
        filename => {
            fhin = File::open(&filename).unwrap();
            &mut fhin
        }
    };
    let outstream: &mut dyn std::io::Write = match outfile.as_str() {
        "-" => {
            stdout = io::stdout();
            &mut stdout
        }
        filename => {
            fhout = File::create(&filename).unwrap();
            &mut fhout
        }
    };
    let reader = fastq::Reader::new(instream);
    let mut count = 0;
    let mut len = 0;
    for result in reader.records() {
        let record = result.expect("Error during Fastq parsing");
        count += 1;
        len += record.seq().len();
    }
    let output = format!("Number of reads: {}; number of bases: {}\n", count, len);
    outstream.write_all(output.as_bytes()).expect("Error writing output");
}
