// find all subsequences of seq in s and return a vector of tuples containing the start position
// and the found subsequences as string slices
// ignore overlaps: if a subsequence is found, the search must continue from the next character
// missing lifetimes: the result string slices depend only from one input parameter, which one?
use crate::amino_acid_sequence::*;

// suggestion: write a function find_sub(&str, &str) -> Option<(usize, &str)> that finds the first subsequence in a string, you can use it in all the following functions
pub fn find_sub<'a, 'b>(full_string: &'a str, substring: &'b str) -> Option<(usize, &'a str)> {
    let encoded: AmminoacidSequence = substring.into();

    for i in 0..full_string.len() {
        match encoded.matches(&full_string[i..]) {
            Some(match_found) => return Some((i, match_found)),
            _ => {}
        }
    }

    None
}

fn subsequences1<'a>(s: &'a str, seq: &'a str) -> Vec<(usize, &'a str)> {
    let mut res: Vec<(usize, &str)> = Vec::new();
    let mut s_head = s;

    while let Some((pos, string)) = find_sub(s_head, seq) {
        s_head = &s[pos + string.len()..];
        res.push((pos, string));
    }

    res
}

pub fn demo1() {
    let a = "AACGGTAACCA".to_string();
    let seq = "A1-1,C2-4";

    for (off, sub) in subsequences1(&a, seq) {
        println!("Found subsequence at position {}: {}", off, sub);
    }
}

// Now we want to find different subsequences at the same time, seq is a vector of string slices with many subsequence to search
// For each subsequence find all the matches and to the results (there may be overlaps, ignore them), but in this way you can reuse the previous solution
// The result will contain: the start position in s, the found subsequence as string slice and the mached subsequence in seq
// Now the string slices in the rsult depend from two input parameters, which ones?
fn subsequences2<'a, 'b>(s: &'a str, seq: &'a [&'a str]) -> Vec<(usize, &'a str, &'a str)> {
    let sequence_plus_matches: Vec<(&str, Vec<(usize, &str)>)> = seq
        .iter()
        .map(|sequence| (*sequence, subsequences1(s, sequence)))
        .collect();

    let matches_with_sequence: Vec<Vec<(usize, &str, &str)>> = sequence_plus_matches
        .iter()
        .map(|(sequence, vec)| {
            let tmp: Vec<(usize, &str, &str)> = vec
                .iter()
                .map(|(pos, matched)| (*pos, *matched, *sequence))
                .collect();
            tmp
        })
        .collect();

    let flattened: Vec<(usize, &str, &str)> = matches_with_sequence.into_iter().flatten().collect();

    return flattened;
}

pub fn demo2() {
    let a = "AACGGTAACC".to_string();
    let seqs = ["A1-1,C2-4", "G1-1,T2-4"];

    for (off, matched, sub) in subsequences2(&a, &seqs) {
        println!("Found subsequence {} at position {}: {}", matched, off, sub);
    }
}

// Now we want to do some DNA editing! Therefore we receive a mutable string and we'd like to return a vector of mutable string slices
// Follow this steps:
// 1. adjust the lifetimes without any implementation yet: does it compile?
// 2. try to implement the function: does it compile?
// 3. if it doesn't compile, try to understand why from the compiler errors and draw all the necessary lifetimes
// 4. Spoiler: basically it's not possibile to return more then one mutable reference to the same data
// 5. Try this workaround: return a vector of indexes (first solution) and let the caller extract the mutable references
// 7. (later in the course you will learn about smart pointers, which can be used to solve this kind of problems in a more elegant way)
fn subsequences3(s: &mut str, seq: &str) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    let mut s_head = &*s;

    while let Some((pos, string)) = find_sub(s_head, seq) {
        s_head = &s[pos + string.len()..];
        res.push((pos, string.len()));
    }

    res
}

pub fn demo3() {
    let mut a = "AACGGTAACC".to_string();
    let seq = "A1-1,C2-4";

    for (off, len) in subsequences3(&mut a, seq) {
        println!(
            "Found subsequence at position {} with len {}: {}",
            off,
            len,
            &mut a[off..off + len]
        );
    }
}

// DNA strings may be very long and we can get a lot of matches.
// Therefore we want to process a subsequence as soon as we find it, without storing it in a vector
// A solution is to pass a closure to the function, which will be called for each match
// do you need to put lifetime annotations in the closure? why?
fn subsequence4(mut s: &str, seq: &str, f: fn(usize, &str) -> ()) {
    while let Some((pos, string)) = find_sub(s, seq) {
        s = &s[pos + string.len()..];
        f(pos, string);
    }
}

pub fn demo4() {
    let a = "AACGGTAACC".to_string();
    let seq = "A1-1,C2-4";

    subsequence4(&a, seq, |pos, sub| {
        println!("Found subsequence at position {}: {}", pos, sub);
    });
}

// Now let's define a struct SimpleDNAIter (add the required lifetimes), memorizing a DNA sequence and the subsequence to search
// Then we add a next() method to the struct, which will return the next subsequence found in the DNA sequence after each call
// The result of next() is a tuple, but it's wrapped in an Option, because a call to next() may find no more subsequences in the DNA sequence
// In order to implement it, you may add any other attribute to the struct (remember: the struct is stateful and after each call to next() you must start from the last position found)
// The struct may be used as shown in the demo_SimpleDNAIter() function
// This approach is similar to the previous one, but it's more flexible and it can be used in more complex scenarios. For example you may interrupt it
// at any time and resume it later

struct SimpleDNAIter<'a> {
    s: &'a str,
    seq: &'a str,
    start: usize,
}

impl<'a> SimpleDNAIter<'a> {
    pub fn new(s: &'a str, seq: &'a str) -> Self {
        SimpleDNAIter {
            s: s,
            seq: seq,
            start: 0,
        }
    }

    pub fn next(&mut self) -> Option<(usize, &str)> {
        if self.start == self.s.len() {
            return None;
        }

        match find_sub(&self.s[self.start..], self.seq) {
            None => {
                self.start = self.s.len();
                None
            }
            Some((index, ptr)) => {
                self.start = self.start + index + ptr.len();
                Some((self.start - ptr.len(), ptr))
            }
        }
    }
}

// TODO: togliere il ciclo infinito rimasto

pub fn demo_simple_dna_iter() {
    let mut dna_iter = SimpleDNAIter::new("ACGTACGTACGTACGT", "A1-1,C1-1");

    while let Some((pos, subseq)) = dna_iter.next() {
        println!("Found subsequence at position {}: {}", pos, subseq);
        // we can break and stop if we have found what we were looking for
    }
}

// finally we want to implement a real iterator, so that it can be used in a for loop and it may be combined we all the most common iterator methods
// The struct DNAIter is already defined, you have to implement the Iterator trait for it and add lifetimes
struct DNAIter<'a> {
    s: &'a str,
    seq: &'a str,
    start: usize,
}

impl<'a> DNAIter<'a> {
    pub fn new(s: &'a str, seq: &'a str) -> DNAIter<'a> {
        DNAIter {
            s: s,
            seq: seq,
            start: 0,
        }
    }
}

impl<'a> Iterator for DNAIter<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.s.len() {
            return None;
        }

        match find_sub(&self.s[self.start..], self.seq) {
            None => {
                self.start = self.s.len();
                None
            }
            Some((index, ptr)) => {
                self.start = self.start + index + ptr.len();
                Some((self.start - ptr.len(), ptr))
            }
        }
    }
}

pub fn demo_dna_iter() {
    let dna_iter = DNAIter::new("ACGTACGTAAACCCGTACGT", "A1-3,C1-2");

    // now you can combine it with all the iterator modifiers!!!
    dna_iter
        .filter(|(_pos, sub)| sub.len() >= 5)
        .for_each(|(pos, sub)| {
            println!(
                "Found subsequence at least long 5 at position {}: {}",
                pos, sub
            )
        });
}

// now let's return an iterator without defining a struct, just using a closure
// the std lib of rust support you with the std::from_fn() function
// we supply a skeleton implementation, you have to fill the closure
fn subsequence5_iter<'a>(s: &'a str, seq: &'a str) -> impl Iterator<Item = (usize, &'a str)> {
    let mut start = 0;

    // and any other necessary variable to remember the state
    std::iter::from_fn(move || match find_sub(&s[start..], seq) {
        None => {
            start = s.len();
            None
        }
        Some((index, ptr)) => {
            start = start + index + ptr.len();
            Some((start - ptr.len(), ptr))
        }
    })
}

pub fn demo_dna_iter2() {
    subsequence5_iter("ACGTACGTAAACCGTACGT", "A1-3,C1-2")
        .filter(|(_pos, sub)| sub.len() >= 5)
        .for_each(|(pos, sub)| {
            println!(
                "Found subsequence at least long 5 at position {}: {}",
                pos, sub
            )
        });
}
