use std::collections::HashMap;
use plotters::prelude::*;
use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        println!("Usage: <file-path>");
        return;
    }

    let file_path = &args[1];
    // println!("{}", file_path);

    let content = fs::read_to_string(file_path).expect("Unable to read from file");

    // Preprocessing to remove punctuations and converting to lowercase
    let re = Regex::new(r"[,.?!']").unwrap();
    let clean_text = re.replace_all(&content, "").to_lowercase();

    let mut words_count: HashMap<String, u32> = HashMap::new();

    let words = clean_text.split_whitespace();
    for word in words {
        let count = words_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    for (word, count) in words_count.iter() {
        println!("{}\t--->\t{}", word, count);
    }

    // Convert HashMap to sorted vectors
    let mut word_vec: Vec<_> = words_count.iter().collect();
    word_vec.sort_by(|a, b| b.1.cmp(a.1)); // Sort by frequency

    let words: Vec<String> = word_vec.iter().map(|&(word, _)| word.clone()).collect();
    let counts: Vec<u32> = word_vec.iter().map(|&(_, &count)| count).collect();

    // Create the drawing area
    let root = BitMapBackend::new("output.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Create a chart
    let max_count = *counts.iter().max().unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Word Frequency", ("Arial", 50).into_font())
        .margin(10)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .build_cartesian_2d(0..words.len(), 0u32..max_count)
        .unwrap();

    chart.configure_mesh()
        .x_labels(words.len())
        .x_label_formatter(&|index| {
            if *index < words.len() {
                words[*index].clone()
            } else {
                String::new()
            }
        })
        .draw()
        .unwrap();

    // Draw the bars
    chart.draw_series(
        (0..words.len()).map(|i| {
            let word = &words[i];
            let count = counts[i];
            let x0 = i;
            let x1 = i + 1;
            let y0 = 0;
            let y1 = count;

            Rectangle::new([(x0, y0), (x1, y1)], RED.filled())
        })
    ).unwrap();

    root.present().unwrap();

    println!("Graph has been displayed.");
}