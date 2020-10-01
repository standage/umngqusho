extern crate argparse;
extern crate bio;
extern crate rand;

use argparse::{ArgumentParser, Store};
use bio::io::fastq;
use rand::Rng;
// use rand_xorshift::XorShiftRng;
use std::fs::File;
use std::io;

fn main() {
    let mut infile = "-".to_string();
    let mut outfile = "-".to_string();
    let mut numreads = "500".to_string();
    let mut seed = "0".to_string();
    let mut fhin;
    let mut fhout;
    let mut stdout;
    let mut stdin;

    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut outfile)
            .add_option(&["-o", "--outfile"], Store, "output file")
            .metavar("FILE");
        parser.refer(&mut numreads)
            .add_option(&["-n", "--num-reads"], Store, "number of sequences to sample")
            .metavar("INT");
        parser.refer(&mut seed)
            .add_option(&["-s", "--seed"], Store, "seed for random number generator")
            .metavar("INT");
        parser.refer(&mut infile)
            .add_argument("seqs", Store, "sequences in Fastq format");
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
    let mut rng = rand::thread_rng();
    let size: usize = numreads.parse().unwrap();
    let mut reservoir: Vec<fastq::Record> = Vec::new();
    let reader = fastq::Reader::new(instream);
    let mut writer = fastq::Writer::new(outstream);
    let mut count = 0;
    for result in reader.records() {
        let record = result.expect("Error during Fastq parsing");
        count += 1;
        if reservoir.len() < size {
            reservoir.push(record);
        }
        else {
            let r = rng.gen_range(1, count);
            if r <= size {
                reservoir[r - 1] = record;
            }
        }
    }
    eprintln!("[umngqusho] Sampled {} reads from a total of {}", reservoir.len(), count);
    for record in reservoir.iter() {
        writer.write_record(record).expect("Error writing Fastq record");
    }
}
