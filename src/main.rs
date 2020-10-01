extern crate argparse;
extern crate bio;
extern crate rand;

use argparse::{ArgumentParser, Store};
use bio::io::fastq;
use rand::{Rng, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
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
    let mut nonseedgen;
    let mut seedgen: XorShiftRng;

    {
        let mut parser = ArgumentParser::new();
        parser
            .refer(&mut outfile)
            .add_option(
                &["-o", "--outfile"],
                Store,
                "Write output to FILE; by default, output is written to the terminal (stdout)",
            )
            .metavar("FILE");
        parser
            .refer(&mut numreads)
            .add_option(
                &["-n", "--num-reads"],
                Store,
                "Randomly sample N sequences from the input; by default N=500",
            )
            .metavar("INT");
        parser
            .refer(&mut seed)
            .add_option(
                &["-s", "--seed"],
                Store,
                "Seed random number generator for reproducible behavior; by default, the RNG sets its own random state"
            )
            .metavar("INT");
        parser
            .refer(&mut infile)
            .add_argument("seqs", Store, "Sequences in Fastq format");
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
    let rng: &mut dyn RngCore = match seed.as_str() {
        "0" => {
            nonseedgen = rand::thread_rng();
            &mut nonseedgen
        }
        seedstr => {
            let seednum: u64 = seedstr.parse().unwrap();
            seedgen = SeedableRng::seed_from_u64(seednum);
            &mut seedgen
        }
    };
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
        } else {
            let r = rng.gen_range(1, count);
            if r <= size {
                reservoir[r - 1] = record;
            }
        }
    }
    eprintln!(
        "Sampled {} reads from a total of {}",
        reservoir.len(),
        count
    );
    for record in reservoir.iter() {
        writer
            .write_record(record)
            .expect("Error writing Fastq record");
    }
}
