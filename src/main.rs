use std::env;
use std::fs;

const MAX_CHAR_LEN: usize = 6;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Correctness {
    Correct,
    Misplaced,
    Absent,
}

struct GuessResult<'a> {
    word: &'a str,
    correctness: [Correctness; MAX_CHAR_LEN],
}

impl<'a> GuessResult<'a> {
    fn compute(target: &'_ str, guess: &'a str) -> GuessResult<'a> {
        let mut ret = GuessResult {
            word: guess,
            correctness: [Correctness::Absent; MAX_CHAR_LEN],
        };

        let mut used = [false; MAX_CHAR_LEN];

        for (i, c) in guess.chars().enumerate() {
            if target.chars().nth(i) == Some(c) {
                used[i] = true;
                ret.correctness[i] = Correctness::Correct;
                continue;
            }

            for (j, t) in target.chars().enumerate() {
                if used[j] {
                    continue;
                }

                if Some(t) == Some(c) {
                    used[j] = true;
                    ret.correctness[i] = Correctness::Misplaced;
                }
            }
        }

        ret
    }

    fn r#match(self: &Self, word: &str) -> bool {
        self.correctness
            .iter()
            .eq(GuessResult::compute(word, &self.word).correctness.iter())
    }
}

fn main() {
    let filename = "data/pli07.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .to_lowercase();

    let words: Vec<&str> = contents
        .split("\n")
        .filter(|w| w.chars().count() == MAX_CHAR_LEN)
        .collect();
    
    let total_words = words.len() as f64;


    for target in words.iter() {
        let mut entropy = 0.0;
        // println!("patern matching for '{}'", target);
        for w in words.iter() {
            let guess = GuessResult::compute(target, w);
            let matching_count = words.iter().filter(|w| guess.r#match(w)).count() as f64;
            // println!(
            //     "    {} {}",
            //     w,
            //     words.iter().filter(|w| guess.r#match(w)).count()
            // );
            let p = matching_count / total_words;
            entropy += -(p * p.log2());
        }
        // entropy /= total_words;
        println!("entropy for '{}': {}", target, entropy);
    }
}
