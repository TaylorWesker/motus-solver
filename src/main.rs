use itertools::Itertools;
use std::env;
use std::fs;
use std::io;

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
            correctness: pattern,
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

fn into_correctness(pattern: &str) -> Vec<Correctness> {
    pattern
        .chars()
        .map(|l| match l {
            'C' => Correctness::Correct,
            'M' => Correctness::Misplaced,
            'A' => Correctness::Absent,
            _ => unreachable!(),
        })
        .collect()
}

fn generate_data<'a>(words_list: Vec<&'a str>, word_size: usize) {
    let words: Vec<&str> = words_list
        .into_iter()
        .filter(|w| w.chars().count() == word_size)
        .collect();

    let total_words = words.len() as f64;

    println!("word, entropy");
    for g in words.iter() {
        let mut entropy = 0.0;
        for p in (0..word_size)
            .map(|_| {
                [
                    Correctness::Correct,
                    Correctness::Misplaced,
                    Correctness::Absent,
                ]
            })
            .multi_cartesian_product()
        {
            let guess = GuessResult::init(g, p);
            let matching_count = words.iter().filter(|w| guess.r#match(w)).count() as f64;
            if matching_count == 0.0 {
                continue;
            }
            let prob = matching_count / total_words;
            entropy += -(prob * prob.log2());
        }
        println!("{}, {}", g, entropy);
    }
}

fn generate_data2<'a>(words_list: Vec<&'a str>, word_size: usize, first_letter: char) {
    let words: Vec<&str> = words_list
        .into_iter()
        .filter(|w| w.chars().count() == word_size && w.chars().nth(0) == Some(first_letter))
        .collect();

    let total_words = words.len() as f64;

    println!("word, entropy");
    for g in words.iter() {
        let mut entropy = 0.0;
        for p in (0..word_size - 1)
            .map(|_| {
                [
                    Correctness::Correct,
                    Correctness::Misplaced,
                    Correctness::Absent,
                ]
            })
            .multi_cartesian_product()
        {
            let p = [&[Correctness::Correct], &p[..]].concat();
            let guess = GuessResult::init(g, p);
            let matching_count = words.iter().filter(|w| guess.r#match(w)).count() as f64;
            if matching_count == 0.0 {
                continue;
            }
            let prob = matching_count / total_words;
            entropy += -(prob * prob.log2());
        }
        println!("{}, {}", g, entropy);
    }
}

fn play<'a>(
    words_list: Vec<&'a str>,
    word_size: usize,
    first_letter: char,
    first_guess: GuessResult,
) {
    let words: Vec<&str> = words_list
        .into_iter()
        .filter(|w| w.chars().count() == word_size && w.chars().nth(0) == Some(first_letter))
        .collect();

    let mut words_remaining: Vec<&&str> = words.iter().filter(|w| first_guess.r#match(w)).collect();

    let mut total_words = words_remaining.len() as f64;

    while total_words as usize != 1 {
        assert!(total_words as usize != 0);
        let mut bests = Vec::new();
        let mut best_entropy: f64 = 0.0;
        for g in words.iter() {
            let mut entropy = 0.0;
            for p in (0..word_size - 1)
                .map(|_| {
                    [
                        Correctness::Correct,
                        Correctness::Misplaced,
                        Correctness::Absent,
                    ]
                })
                .multi_cartesian_product()
            {
                let p = [&[Correctness::Correct], &p[..]].concat();
                let guess = GuessResult::init(g, p);
                let matching_count =
                    words_remaining.iter().filter(|w| guess.r#match(w)).count() as f64;
                if matching_count == 0.0 {
                    continue;
                }
                let prob = matching_count / total_words;
                entropy += -(prob * prob.log2());
            }
            if entropy > best_entropy {
                best_entropy = entropy;
                let next_guess = g.to_string();
                bests.push((next_guess, entropy));
            }
        }
        println!("{:#?}", bests);
        println!("{}", total_words);
        println!("{:?}", words_remaining);

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let nexts_guessm = buffer.clone();
        let nexts_guess = nexts_guessm.trim();

        buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let pattern = buffer.trim();

        let new_guess = GuessResult::init(&nexts_guess, into_correctness(&pattern));

        words_remaining = words_remaining
            .into_iter()
            .filter(|w| new_guess.r#match(w))
            .collect();

        total_words = words_remaining.len() as f64;
    }
}

fn display_match<'a>(
    words_list: Vec<&'a str>,
    word_size: usize,
    first_letter: char,
    guess: GuessResult,
) -> Vec<&'a str> {
    let words = words_list.into_iter().filter(|w| {
        w.chars().count() == word_size && w.chars().nth(0) == Some(first_letter) && guess.r#match(w)
    });

    words.collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let char_len = args[0].parse::<usize>().unwrap();
    let first_letter = args[1].parse::<char>().unwrap();
    let first_guess = args[2].parse::<String>().unwrap();
    let first_mask = args[3].parse::<String>().unwrap();

    let filename = "data/dict.txt";
    let file_content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .to_lowercase();
    let words_list = file_content.split("\n").collect();

    play(
        words_list,
        char_len,
        first_letter,
        GuessResult::init(&first_guess, into_correctness(&first_mask)),
    );
}
