pub mod aliquot;
pub mod error;
pub mod types;

use crate::aliquot::*;
use crate::error::AliquotError;
use std::env;
use std::ops::Range;
use std::str::FromStr;
use std::thread;

fn help() {
    println!("Usage: aliquot [-m] NUMBER(s)");
    println!("-n MAX      Maximum number of numbers in a sequence (default: 1000000)");
    println!(
        "-m MAX      Maximum value for a number in a sequence (default: {})",
        u64::MAX
    );
    println!("-c SIZE     Cache size (default: 1000000)");
    println!("-l          Just print the lengths of the sequences");
    println!("-t THREADS  Set the number of threads to use");
    println!("-s          Just compute the aliquot sum instead of the aliquot sequence");
    println!("-v          Print debug messages");
    println!("-h          Print this help");
}

fn run() -> Result<(), AliquotError> {
    let args = env::args().collect::<Vec<String>>();
    let get_arg = |ind: usize| -> Result<&String, AliquotError> {
        if ind < args.len() {
            return Ok(&args[ind]);
        }
        let err_msg = format!("Missing argument at index {ind}");
        Err(AliquotError::InvalidArg(err_msg))
    };
    let mut debug = false;
    let mut max_len_seq = 1_000_000;
    let mut max_num = u64::MAX;
    let mut max_cache_size = 1_000_000;
    let mut lengths_only = false;
    let mut aliquot_sum_only = false;
    let mut n_threads = 1;
    let mut ranges: Vec<Range<u64>> = vec![];
    let mut ind = 1;
    while ind < args.len() {
        let arg = args[ind].as_str();
        match arg {
            "-n" => {
                ind += 1;
                let arg_string = get_arg(ind)?;
                max_len_seq = usize::from_str(arg_string)?;
            }
            "-m" => {
                ind += 1;
                let arg_string = get_arg(ind)?;
                max_num = u64::from_str(arg_string)?;
            }
            "-c" => {
                ind += 1;
                let arg_string = get_arg(ind)?;
                max_cache_size = usize::from_str(arg_string)?;
            }
            "-l" => {
                lengths_only = true;
            }
            "-t" => {
                ind += 1;
                let arg_string = get_arg(ind)?;
                n_threads = usize::from_str(arg_string)?;
            }
            "-s" => {
                aliquot_sum_only = true;
            }
            "-v" => {
                debug = true;
            }
            "-h" => {
                help();
                return Ok(());
            }
            _ => {
                // We assume these are the ranges of numbers to compute the aliquot sequences for
                for splt in arg.split(',') {
                    let range = match splt.find('-') {
                        Some(pos) => {
                            let (start_str, end_str) = splt.split_at(pos);
                            let start = u64::from_str(start_str)?;
                            let end = u64::from_str(&end_str[1..])? + 1;
                            if end < start {
                                let err_msg = format!("{start} - {end}");
                                return Err(AliquotError::InvalidRange(err_msg));
                            }
                            start..end
                        }
                        None => {
                            // This is just a single number
                            let num = u64::from_str(splt)?;
                            num..(num + 1)
                        }
                    };
                    ranges.push(range);
                }
            }
        }
        ind += 1;
    }
    // Distribute work to independent threads
    let mut workload = vec![vec![]; n_threads];
    if ranges.len() == 1 && n_threads > 1 {
        let n_per_thread = (ranges[0].end - ranges[0].start) / n_threads as u64;
        // Split the range
        for i in 0..n_threads {
            let ind = i as u64;
            let start = ranges[0].start + (ind * n_per_thread);
            let end = if i == (n_threads - 1) {
                ranges[0].end
            } else {
                ranges[0].start + ((ind + 1) * n_per_thread)
            };
            workload[i].push(start..end);
        }
    } else {
        // Distribute the ranges among the threads
        // The number of threads should not exceed the number of ranges
        for i in 0..ranges.len() {
            workload[i % n_threads].push(ranges[i].clone());
        }
    }
    if debug {
        println!("Debug: Number of threads: {n_threads}");
    }
    let mut handles = vec![];
    for w in workload {
        let handle = thread::spawn(move || -> Result<(), AliquotError> {
            let mut gener =
                Generator::<u64>::with_params(max_num, max_len_seq, max_cache_size / n_threads, debug);
            for range in w {
                if aliquot_sum_only {
                    for n in range {
                        let aliquot_sum = Generator::<u64>::aliquot_sum(n)?;
                        println!("{n} {aliquot_sum}");
                    }
                } else {
                    for n in range {
                        let aliquot_seq = gener.aliquot_seq(n);
                        if lengths_only {
                            println!("{} {}", n, aliquot_seq.len());
                        } else {
                            let type_str = aliquot_seq.type_str();
                            let seq_string = aliquot_seq.seq_string();
                            println!("{n}: {type_str} {seq_string}");
                        }
                    }
                }
            }
            if debug {
                println!(
                    "Debug: Cache stored {} sequences and {} numbers",
                    gener.cache().n_seq(),
                    gener.cache().count()
                );
            }
            Ok(())
        });
        handles.push(handle);
    }
    for h in handles.into_iter() {
        h.join().unwrap()?;
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{err}");
    }
}
