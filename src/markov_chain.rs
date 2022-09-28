
use rand::{prelude::{IteratorRandom, Distribution}, distributions::WeightedIndex};
use std::fmt;

#[derive(Debug)]
pub struct MarkovChain {
    matrix: Vec<Vec<f64>>,
    raw_content: String,
    unique_words: Vec<String>
}

impl fmt::Display for MarkovChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string: Vec<String> = vec![];
        let mut longest = 0;
        for word in &self.unique_words {
            if word.len() > longest {
                longest = word.len();
            }
        }
        string.push(" ".repeat(longest + 2));

        for word in &self.unique_words {
            string.push(format!("{word}{}", " ".repeat(2 + longest - word.len())));
        }
        string.push("\n".to_string());
        for (i, row) in self.matrix.iter().enumerate() {
            string.push(format!("{} {}", self.unique_words[i], " ".repeat(1 + longest - self.unique_words[i].len())));
            for item in row {
                string.push(format!("{item:.2} {}", " ".repeat(1 + longest - format!("{item:.2}").len())));
            }
            string.push("\n".to_string());
        }

        writeln!(f, "{}", &string.join(""))
    }
}

impl MarkovChain {

    /// Removes all superflous lines such as newlines and blank lines.
    fn parse(mut input: Vec<String>) -> Vec<String> {
        input.retain(|s| s.as_str() != "\n");
        let input = input.iter().map(|s| s.replace("\n", "")).collect();

        input
    }

    pub fn new<T: AsRef<str>>(content: T) -> MarkovChain {
        let vec = content.as_ref().to_string().split(' ').map(|s| s.to_string()).collect();
        let parsed_content = MarkovChain::parse(vec);

        // TODO: Redo this parse with iterator::retain() or something
        let mut unique_words: Vec<String> = Vec::new();
        for word in &parsed_content {
            if !unique_words.contains(&word.to_string()) {
                unique_words.push(word.to_string());
            }
        }

        let mut matrix: Vec<Vec<f64>> = Vec::with_capacity(unique_words.len());
        for _ in 0..unique_words.len() {
            matrix.push(vec![0.0; unique_words.len()]);
        }

        MarkovChain {
            matrix,
            raw_content: parsed_content.join(" "),
            unique_words,
        }
    }

    /// Gets the index of a specific word instide the chain corpus.
    pub fn word_index(&self, s: &String) -> Option<usize> {
        for (i, word) in self.unique_words.iter().enumerate() {
            if word == s {
                return Some(i);
            }
        }

        None
    }

    pub fn train(&mut self) {
        let raw_word_array: Vec<String> = self.raw_content.split(' ').map(|x| x.to_string()).collect();
        for raw_word_index in 0 .. raw_word_array.len() - 1 {
            let word = raw_word_array[raw_word_index].to_string();
            let src_word_index  = self.word_index(&word).unwrap();
            let dest_word_index = self.word_index(&raw_word_array[raw_word_index + 1]).unwrap();
            let frac: f64 = (1.0 / raw_word_array.iter().filter(|x| **x == word).count() as f64) as f64;

            self.matrix[src_word_index][dest_word_index] += frac;
        }
    }

    /// Generate a `n` word long string of text from the chain.
    pub fn generate(&self, n: usize) -> String {
        let mut rng = rand::thread_rng();

        // For now:
        // just pick a random word from the raw_content.
        let starter = self.raw_content.split(' ').choose(&mut rng).unwrap().to_string();

        let mut current_word = starter.clone();
        let mut sentence: Vec<String> = Vec::with_capacity(n);
        sentence.push(starter);
        for _ in 0..n - 1 {
            let cur_word_index = self.word_index(&current_word).unwrap();

            // Get a weighted distribution from the matrix for `current_word`
            let (words, weights): (Vec<&String>, Vec<f64>) = self.matrix[cur_word_index]
                .iter()
                .zip(self.unique_words.iter())
                .filter(|(f, _)| **f > 0.0)
                .map(|(x, s)| (s, x))
                .unzip();

            if weights.is_empty() {
                break;
            }

            let w_distri = WeightedIndex::new(weights).unwrap();

            let next_word = words[w_distri.sample(&mut rng)];

            sentence.push(next_word.to_string());
            current_word = next_word.to_string();
        }

        sentence.join(" ")
    }

    pub fn len(&self) -> usize {
        self.unique_words.len()
    }
}
