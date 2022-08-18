use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::env;

struct SortedWords<'a> {
    sorted: HashMap<String, Vec<&'a str>>
}
impl<'a> SortedWords<'a> {
    fn new(prompt_length: usize, words: Vec<&'a str>) -> Self {
        let mut sorted = HashMap::<String, Vec<&str>>::new();
        std::iter::repeat('a'..='z')
            .take(prompt_length)
            .multi_cartesian_product()
            .for_each(|chars| drop(sorted.insert(chars.into_iter().collect(), Vec::new())));
        
        sorted.iter_mut()
            .for_each(|(prompt, vec)| words.iter()
                .for_each(|word| if word.contains(prompt) { vec.push(word) }));
        
        Self { sorted }
    }
    fn as_json(&self) -> String {
        let mut buf = '{'.to_string();
        self.sorted.iter()
            .for_each(|(prompt, vec)| {
                buf.push_str(format!("\"{}\":[", prompt).as_str());
                if !vec.is_empty() {
                    vec.iter()
                        .for_each(|word| buf.push_str(format!("\"{}\",", word).as_str()));
                    buf.pop();
                }
                buf.push_str("],")
            });
        buf.pop();
        buf.push('}');
        buf
    }
}

fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    let mut buf = String::new();
    File::open(&args[2])?.read_to_string(&mut buf)?;

    File::create("out.txt")?.write_all(SortedWords::new(args[1].parse::<usize>().unwrap(), buf.lines().collect()).as_json().as_bytes())?;

    Ok(())
}