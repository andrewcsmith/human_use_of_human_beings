extern crate voting_experts;
extern crate radix_trie;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use voting_experts::trie::{frequency_trie_from_string, conditional_probability, boundary_entropy};
use voting_experts::{Expert, TrieStats, Key};
use radix_trie::{TrieKey, Trie, TrieCommon};

pub struct SilenceExpert<'a, K: 'a + TrieKey, V: 'a> {
    trie: &'a Trie<K, V>,
    stats: &'a TrieStats,
}

impl<'a> Expert for SilenceExpert<'a, Key<'a>, u32> {
    fn vote(&self, string: &str) -> usize {
        let mut winner: (usize, f64) = (0, 0.);
        for i in 1..string.len() {
            if string.is_char_boundary(i) {
                let (a, b): (&str, &str) = string.split_at(i);
                let aent = self.stats.normalized(self.trie, Key(a))
                    .1.unwrap_or(0.);
                let bent = self.stats.normalized(self.trie, Key(&b[1..]))
                    .1.unwrap_or(0.);
                
                let entropy = aent + bent;
                if entropy > winner.1 {
                    winner = (i, entropy)
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

fn bring_silence<'a>(string: &'a str, votes: &[usize], window_size: usize, threshold: usize) -> Vec<&'a str> {
    let mut boundaries: Vec<usize> = Vec::new();
    for (idx, window) in votes.windows(window_size).enumerate() {
        let (max_idx, max_val) = window.iter().enumerate().max_by_key(|w| w.1).unwrap();
        let abs_index = max_idx + idx;
        if *max_val >= threshold && boundaries.last().and_then(|b| Some(*b != abs_index)).unwrap_or(true) {
            boundaries.push(abs_index);
        }
    }

    let mut out: Vec<&str> = Vec::new();

    let mut byte_index = 0usize;
    let mut char_iter = string.chars();
    let mut last_boundary = 0;

    for boundary in boundaries {
        let mut new_byte_index = byte_index;
        for _ in 0..(boundary - last_boundary) {
            new_byte_index += char_iter.next().unwrap().len_utf8();
        }
        out.push(&string[byte_index..(new_byte_index-1)]);
        last_boundary = boundary;
        byte_index = new_byte_index;
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
    println!("{}", &s);

    for (i, paragraph) in s.split('\n').enumerate() {
        let offset = 5;
        let window_size = (8 - i) + offset;
        let votes = cast_votes(&paragraph, window_size);
        let silenced = bring_silence(&paragraph, &votes, window_size - offset, (8 - i)).iter().fold(String::new(), |mut memo, s| {
            memo.push_str(s);
            memo.push(' ');
            memo
        });
        println!("{}\n\n{}\n\n", i, &silenced);
    }

    Ok(())
}
