extern crate argparse;
extern crate bio;
extern crate rand;

use argparse::{ArgumentParser, Store, StoreOption};
use bio::io::fastq;
use rand::{Rng, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::fs::File;
use std::io;
use std::path::PathBuf;

struct Arguments {
    infile: Option<PathBuf>,
    outfile: Option<PathBuf>,
    numreads: usize,
    seed: Option<u64>,
}

impl Arguments {
    fn parse_or_exit() -> Self {
        let mut outfile = None;
        let mut infile = None;
        let mut numreads = 500;
        let mut seed = None;

        {
            let mut parser = ArgumentParser::new();
            parser
                .refer(&mut outfile)
                .add_option(
                    &["-o", "--outfile"],
                    StoreOption,
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
                    StoreOption,
                    "Seed random number generator for reproducible behavior; by default, the RNG sets its own random state"
                )
                .metavar("INT");
            parser
                .refer(&mut infile)
                .add_argument("seqs", StoreOption, "Sequence in Fastq format");
            parser.parse_args_or_exit();
        }

        Self {
            outfile,
            infile,
            numreads,
            seed,
        }
    }

    fn instream(&self) -> Box<dyn io::Read> {
        match &self.infile {
            Some(filename) => Box::new(File::open(&filename).unwrap()) as Box<dyn io::Read>,
            None => Box::new(io::stdin()),
        }
    }

    fn outstream(&self) -> Box<dyn io::Write> {
        match &self.outfile {
            Some(filename) => Box::new(File::create(&filename).unwrap()) as Box<dyn io::Write>,
            None => Box::new(io::stdout()),
        }
    }

    fn rng(&self) -> Box<dyn RngCore> {
        match self.seed {
            Some(seed) => Box::new(XorShiftRng::seed_from_u64(seed)) as Box<dyn RngCore>,
            None => Box::new(rand::thread_rng()),
        }
    }
}

fn sample_records(
    instream: &mut dyn io::Read,
    rng: &mut dyn RngCore,
    numreads: usize,
) -> Vec<fastq::Record> {
    let mut reservoir: Vec<fastq::Record> = Vec::new();
    let reader = fastq::Reader::new(instream);
    let mut count = 0;
    for result in reader.records() {
        let record = result.expect("Error during Fastq parsing");
        count += 1;
        if reservoir.len() < numreads {
            reservoir.push(record);
        } else {
            let r: usize = rng.gen_range(1, count);
            if r <= numreads {
                reservoir[r - 1] = record;
            }
        }
    }

    reservoir
}

fn write_records(outstream: &mut dyn io::Write, reads: &[fastq::Record]) {
    let mut writer = fastq::Writer::new(outstream);
    for record in reads {
        writer
            .write_record(record)
            .expect("Error writing Fastq record");
    }
}

fn main() {
    let args = Arguments::parse_or_exit();
    let mut instream = args.instream();
    let mut outstream = args.outstream();
    let mut rng = args.rng();
    let reads = sample_records(&mut instream, &mut rng, args.numreads);
    write_records(&mut outstream, &reads);
}
