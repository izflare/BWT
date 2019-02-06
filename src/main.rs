extern crate sais;
extern crate clap;

use clap::{App, Arg};
use std::char;
use std::io::{prelude::*, BufReader, BufWriter};
use std::fs::File;
use std::collections::HashMap;
use std::time::Instant;
use sais::suffixarray;

// bucket constrution: char -> (start pos, size)
fn bucket_calc(d: &mut HashMap<usize, (usize, usize)>, v: &Vec<usize>, o: &mut Vec<usize>) -> () {
    //{{{
    let mut p = v[0]; // min char
    let mut q = v[0]; // max char
    let mut z = 0; // appearing time
    // counting appearing time of each char
    for j in 0..v.len() {
        let ref c = v[j];
        let i = match d.get(c) {Some(x) => (*x).1 + 1, None => 1};
        o[j] = i-1;
        d.insert(*c, (0, i));
        if *c < p {p = *c};
        if *c > q {q = *c};
    }
    // set start position of each char in bucket
    for b in p..q+1 {
        let i = match d.get(&b) {Some(x) => (*x).1, None => 0};
        if i > 0 {d.insert(b, (z, i));};
        z += i;
    }
    //}}}
}

fn main() {
    // arg
    let app = App::new("bwt")
        //{{{
        .version("0.1.0")                       
        .author("flare")     
        .about("bwt text convertor")
        .arg(Arg::with_name("input")
            .help("filename of input sourse text")
            .short("i")
            .long("input")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("reverse")
            .help("reverse the input text (input file must be .bwt file)")
            .short("r")
            .long("reverse")
        );
        //}}}
    let matches = app.get_matches();

    // file reading
    let mut s = String::new();
    let mut f = BufReader::new(File::open(&matches.value_of("input").unwrap()).expect("file not found"));
    f.read_to_string(&mut s).unwrap();
    let rev: bool = matches.is_present("reverse");

    // string -> vec (for index access, each char is casted to usize)
    let mut v: Vec<usize> = s.chars().map(|c| c as usize).collect();
    // constructing bwt
    if !rev {
        v.push(0);
        println!("constructing saffix array...");
        let mut sa: Vec<Option<usize>> = vec![None; v.len()];
        let start = Instant::now();
        suffixarray::create(&v, &mut sa);
        println!("done");
        println!("constructing BWT...");
        let bwt: String  = sa.iter().map(|x| char::from_u32(if x.unwrap() == 0 {v[v.len()-1] as u32} else {v[x.unwrap() - 1] as u32}).unwrap()).collect();
        let end = start.elapsed();
        println!("done");

        println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);
        let mut f = BufWriter::new(File::create(matches.value_of("input").unwrap().to_owned()+".bwt").unwrap());
        f.write(bwt.as_bytes()).unwrap();
    }

    // restore from bwt
    else {
        // appearing order among same chars in v
        let mut o: Vec<usize> = vec![0; v.len()];
        let h = matches.value_of("input").unwrap();
        let start = Instant::now();
        if (h.split('.').rev()).next().unwrap() != "bwt" {panic!("not .bwt type file.");}
        else {
            println!("restoring original text from BWT...");
            let mut d = HashMap::new();
            bucket_calc(&mut d, &v, &mut o);
            let mut p: usize = 0;
            let mut w: Vec<u32> = vec![0; v.len()-1];
            // LF-mapping
            for i in (0..v.len()-1).rev() {
                w[i] = v[p] as u32;
                p = (d.get(&v[p]).unwrap()).0 + o[p];
            }
            println!("done");

            let end = start.elapsed();
            println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);
            let mut f = BufWriter::new(File::create(h.to_owned()+".rev").unwrap());
            f.write(w.iter().map(|x| char::from_u32(*x).unwrap()).collect::<String>().as_bytes()).unwrap();
        }
    }
}
