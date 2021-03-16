use base64;
use std::{fs, io};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    path: String,
}

#[allow(dead_code)]
fn hamming_distance_between_bytes(b1: u8, b2: u8) -> usize {
    let mut x = b1 ^ b2;
    let mut distance: usize = 0;

    while x > 0 {
        distance += (x & 1) as usize;
        x >>= 1;
    }

    distance
}

#[allow(dead_code)]
fn hamming_distance_between_strings(s1: &[u8], s2: &[u8]) -> Result<usize, io::Error> {
    if s1.len() != s2.len() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "invalid string length",
        ));
    }

    let mut distance: usize = 0;
    for (b1, b2) in s1.iter().zip(s2) {
        distance += hamming_distance_between_bytes(*b1, *b2);
    }

    Ok(distance)
}

fn main() -> Result<(), io::Error> {
    let args = Cli::from_args();
    let contents: String = fs::read_to_string(&args.path)
        .expect("Could not read file.")
        .split_whitespace()
        .collect();

    let bytes = base64::decode(contents).unwrap();
    println!("Decoded: {:?}", bytes);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_website_example() {
        assert_eq!(
            hamming_distance_between_strings(b"this is a test", b"wokka wokka!!!").unwrap(),
            37
        );
    }
}
