use std::env;
use std::fs;
use itertools::Itertools;

const MAX_CHAR_LEN: usize = 6;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Correctness {
    Correct,
    Misplaced,
    Absent,
}

struct GuessResult<'a> {
    word: &'a str,
    correctness: Vec<Correctness>,
}

impl<'a> GuessResult<'a> {

    fn init(w: &'a str, pattern: Vec<Correctness>) -> Self {
        Self {
            word: w,
            correctness: pattern
        }
    }

    fn compute(target: &'_ str, guess: &'a str) -> Self {
        let mut ret = GuessResult {
            word: guess,
            correctness: vec![Correctness::Absent; target.len()],
        };

        let mut used = vec![false; target.len()];

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

fn generate_data(word_size: usize) {
    let filename = "data/pli07.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .to_lowercase();

    let words: Vec<&str> = contents
        .split("\n")
        .filter(|w| w.chars().count() == word_size)
        .collect();
    
    let total_words = words.len() as f64;

    println!("word, entropy");
    for g in words.iter() {
        let mut entropy = 0.0;
        // println!("patern matching for '{}'", g);
        for p in (0..word_size).map(|_| [Correctness::Correct, Correctness::Misplaced, Correctness::Absent]).multi_cartesian_product() {
            let guess = GuessResult::init(g, p);
            let matching_count = words.iter().filter(|w| guess.r#match(w)).count() as f64;
            if matching_count == 0.0 {
                continue;
            }
            let prob = matching_count / total_words;
            entropy += -(prob * prob.log2())
            // println!(
            //     "    {} {:?} {}",
            //     g,
            //     p,
            //     matching_count
            // );
        }
        // println!("word entropy for '{}' : {}", g, entropy);
        println!("{}, {}", g, entropy);

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    println!("{:?}", args);
    let char_len = args[0].parse::<usize>().unwrap();
    generate_data(char_len);
}
