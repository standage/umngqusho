# umngqusho: reservoir sampler for Fastq data written in Rust

```
$ umngqusho -h
Usage:
  umngqusho [OPTIONS] [SEQS]


Positional arguments:
  seqs                  Sequences in Fastq format

Optional arguments:
  -h,--help             Show this help message and exit
  -o,--outfile FILE     Write output to FILE; by default, output is written to
                        the terminal (stdout)
  -n,--num-reads INT    Randomly sample N sequences from the input; by default
                        N=500
  -s,--seed INT         Seed random number generator for reproducible behavior;
                        by default, the RNG sets its own random state
$
$ umngqusho --outfile=sampled1.fastq --num-reads=25 demo.fastq
Sampled 25 reads from a total of 496
$ umngqusho --outfile=sampled2.fastq --num-reads=25 demo.fastq
Sampled 25 reads from a total of 496
$ umngqusho --outfile=sampled3.fastq --num-reads=25 --seed=17332048 demo.fastq 
Sampled 25 reads from a total of 496
$ umngqusho --outfile=sampled4.fastq --num-reads=25 --seed=17332048 demo.fastq 
Sampled 25 reads from a total of 496
$ shasum sampled?.fastq
091b4b8153b97f94a4357342bbfbf6d1d58ce108  sampled1.fastq
e04c54c19f1a54e511ee3c1dd8e3d73b8cb05c46  sampled2.fastq
6e741b8b525a14aafee8884fe616fae171524ed8  sampled3.fastq
6e741b8b525a14aafee8884fe616fae171524ed8  sampled4.fastq
```
