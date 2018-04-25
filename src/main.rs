extern crate voting_experts;
extern crate radix_trie;
extern crate serde;
extern crate serde_json;
extern crate bincode;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use voting_experts::trie::{frequency_trie_from_string, conditional_probability, boundary_entropy};
use voting_experts::{Expert, TrieStats, Key};
use radix_trie::{TrieKey, Trie, TrieCommon};
use serde::{Serialize, Deserialize};

static TRIE_FILENAME: &str = "trie.code";
static STATS_FILENAME: &str = "stats.json";

pub struct SilenceExpert<'a, K: 'a + TrieKey, V: 'a> {
    trie: &'a Trie<K, V>,
    stats: &'a TrieStats,
}

impl<'a> Expert for SilenceExpert<'a, Key<'a>, u32> {
    fn vote(&self, string: &str) -> usize {
        let mut winner: (usize, f64) = (0, 0.);

        // Returns the str len in bytes
        for i in 1..string.len() {
            if string.is_char_boundary(i) {
                let (a, b): (&str, &str) = string.split_at(i);
                let (_, aent) = self.stats.normalized(self.trie, Key(a));
                let (_, bent) = self.stats.normalized(self.trie, Key(&b[1..]));

                // Must register entropy for both
                if let (Some(aent), Some(bent)) = (aent, bent) {
                    let entropy = aent + bent;
                    if entropy > winner.1 {
                        winner = (i, entropy)
                    } 
                }
            }
        }

        winner.0
    }
}

fn cast_votes(string: &str, window_size: usize) -> Vec<usize> {
    let mut out: Vec<usize> = vec![0; string.chars().count()];

    let trie = frequency_trie_from_string(string, window_size+1);
    let stats = TrieStats::from_trie(&trie);
    let silence_expert = SilenceExpert {
        trie: &trie, 
        stats: &stats,
    };

    for offset in 0..(string.len()-window_size) {
        let offset_bytes = string.chars()
            .take(offset).fold(0usize, |acc, c| acc + c.len_utf8());
        let byte_len = string[offset_bytes..].chars()
            .take(window_size).fold(0usize, |acc, c| acc + c.len_utf8());
        let key = &string[offset_bytes..(offset_bytes + byte_len)];
        let vote = silence_expert.vote(&key);
        out[vote + offset] += 1;
    }
    out
}

/// Casts votes and returns a vote for each char
fn cast_votes_with_trie(string: &str, window_size: usize, trie: &Trie<Key, u32>, stats: &TrieStats) -> Vec<usize> {
    let str_chars_len = string.chars().count();
    let mut out: Vec<usize> = vec![0; str_chars_len];

    let silence_expert = SilenceExpert {
        trie, 
        stats,
    };

    for offset in 0..(string.len()-window_size) {
        let offset_bytes = string.chars()
            .take(offset).fold(0usize, |acc, c| acc + c.len_utf8());
        let byte_len = string[offset_bytes..].chars()
            .take(window_size).fold(0usize, |acc, c| acc + c.len_utf8());
        let key = &string[offset_bytes..(offset_bytes + byte_len)];
        let vote = silence_expert.vote(&key);
        out[vote + offset] += 1;
    }

    out
}

fn bring_silence<'a>(string: &'a str, votes: &[usize], window_size: usize, threshold: usize) -> Vec<&'a str> {
    let mut boundaries: Vec<usize> = Vec::new();

    let (_, mut boundaries): (_, Vec<usize>) = votes
        .windows(window_size)
        .enumerate()
        .fold((0, Vec::new()), |(_, mut memo), (idx, window)| {
            let (max_idx, max_val) = window.iter().enumerate().max_by_key(|w| w.1).unwrap();
            let abs_index = max_idx + idx;

            if max_val >= &threshold && 
                boundaries.last().and_then(|b| Some(*b != abs_index)).unwrap_or(true) 
            {
                memo.push(abs_index);
            }

            (0, memo)
        });
    
    boundaries.push(string.len());
    
    let mut out: Vec<&str> = Vec::new();

    let mut char_iter = string.chars();
    let mut last_boundary = 0;

    for boundary in boundaries {
        out.push(&string[last_boundary..boundary]);
        last_boundary = boundary;
    }

    out
}

fn main() {
    go().unwrap()
}

fn go() -> Result<(), Box<Error>> {
    let mut s = String::new();
    let mut f = File::open(Path::new("./source.txt"))?;
    f.read_to_string(&mut s)?;
    // println!("{}", &s);

    let mut bison = String::new();
    let mut bison_file = File::open(Path::new("./bison.txt"))?;
    match bison_file.read_to_string(&mut bison) {
        Ok(n) => { println!("read {} bytes of bison", n); }
        Err(e) => { println!("errror: {}", e); }
    }

    bison = bison.to_lowercase();
    bison = bison.replace("\n", " ");
    bison = bison.replace("\r", " ");
    bison = bison.replace("\"", "");

    let trie_buf = File::open(Path::new(TRIE_FILENAME))
        .map(|mut f| {
            let mut buf = Vec::new();
            f.read_to_end(&mut buf).unwrap();
            buf
        });

    let trie = match &trie_buf {
        Ok(buf) => {
            // serde_json::from_str(&buf[..])?
            bincode::deserialize(&buf[..])?
        }

        Err(_) => {
            let trie = frequency_trie_from_string(&bison, 15);
            // let buf = serde_json::to_string_pretty(&trie)?;
            let buf: Vec<u8> = bincode::serialize(&trie)?;
            File::create(Path::new(TRIE_FILENAME))?
                .write_all(&buf[..])?;
            trie
        }
    };

    println!("read trie");

    let stats_buf = File::open(Path::new(STATS_FILENAME))
        .map(|mut f| {
            let mut buf = String::new();
            f.read_to_string(&mut buf).unwrap();
            buf
        });
    
    let stats = match &stats_buf {
        Ok(buf) => {
            serde_json::from_str(&buf[..])?
        }

        Err(_) => {
            let stats = TrieStats::from_trie(&trie);
            let buf = serde_json::to_string_pretty(&stats)?;
            File::create(Path::new(STATS_FILENAME))?
                .write_all(&buf.as_bytes())?;
            stats
        }
    };

    println!("calc stats");

    for (i, paragraph) in s.split('\n').enumerate() {
        let paragraph = paragraph.to_lowercase();
        let voting_window_size = 9;
        let silence_window_size = 3;
        let threshold = (8 - i);
        let votes = cast_votes_with_trie(&paragraph, voting_window_size, &trie, &stats);
        println!("");
        let silenced = bring_silence(&paragraph, &votes, silence_window_size, threshold).iter()
            .fold(String::new(), |mut memo, s| {
                for c in s.chars() {
                    memo.push(c);
                }
                memo.pop();
                memo
            });
        println!("{}\n\n{}\n\n", i, &silenced);
    }

    Ok(())
}
