use lazy_static::lazy_static;
use rand::distr::Distribution;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use wordcloud_rs::*;

static MIN_FONT_SIZE: f32 = 10.0;
static MAX_FONT_SIZE: f32 = 70.0;

lazy_static! {
    static ref RE_TOKEN: Regex = Regex::new(r"\w+").unwrap();
}

fn tokenize(text: String, is_rondom_font_size: bool) -> Vec<(Token, f32)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for token in RE_TOKEN.find_iter(&text) {
        *counts.entry(token.as_str().to_string()).or_default() += 1;
    }
    counts
        .into_iter()
        .map(|(k, v)| {
            (
                Token::Text(k),
                if is_rondom_font_size {
                    let mut rng = rand::rng();
                    v as f32
                        * rand::distr::Uniform::new(MIN_FONT_SIZE, MAX_FONT_SIZE)
                            .unwrap()
                            .sample(&mut rng) as f32
                } else {
                    v as f32
                },
            )
        })
        .collect()
}

fn get_tokens_from_file(path: &PathBuf, is_rondom_font_size: bool) -> Vec<(Token, f32)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for token in fs::read_to_string(path).unwrap().split_whitespace() {
        if token.is_empty() {
            continue;
        }
        let token = Token::Text(token.to_string());
        let count = 1;
        *counts.entry(token.to_string()).or_default() += count;
    }
    let tokens = counts
        .into_iter()
        .map(|(k, v)| {
            (
                Token::Text(k),
                if is_rondom_font_size {
                    let mut rng = rand::rng();
                    v as f32
                        * rand::distr::Uniform::new(MIN_FONT_SIZE, MAX_FONT_SIZE)
                            .unwrap()
                            .sample(&mut rng) as f32
                } else {
                    v as f32
                },
            )
        })
        .collect();
    return tokens;
}

fn get_image_files(path: &PathBuf) -> Vec<String> {
    let mut image_files = Vec::new();
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "jpg" || ext == "png" || ext == "jpeg" {
                        image_files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    image_files
}

pub fn generate(
    text_path: PathBuf,
    images_path: Option<PathBuf>,
    font_ttf: Option<PathBuf>,
    is_tokenize: bool,
    is_rondom_font_size: bool,
) {
    // Prepare the tokens
    let text = fs::read_to_string(text_path.clone()).unwrap();
    let mut tokens = if is_tokenize {
        tokenize(text, is_rondom_font_size)
    } else {
        get_tokens_from_file(&text_path, is_rondom_font_size)
    };
    tracing::info!(target: "wordcloud", "Tokenized {} words", tokens.len());

    // Prepare the images
    if let Some(images_path) = images_path {
        let image_files = get_image_files(&images_path);
        image_files.iter().for_each(|file| {
            let token = Token::from(file);
            tokens.push((token, 10.));
        });
    }
    tracing::info!(target: "wordcloud", "Added {} images", tokens.len());

    // Generate the word-cloud
    let mut builder = WordCloud::new();

    // set font

    let font = match font_ttf {
        Some(font) => font,
        None => PathBuf::from("wordcloud/assets/HackGenConsoleNF-Bold.ttf"),
    };
    builder = builder.font(font.to_str().unwrap());
    tracing::info!(target: "wordcloud", "Using font: {}", font.display());

    // Save it
    let wc = builder.generate(tokens);
    wc.save("wordcloud.png").unwrap();
    tracing::info!(target: "wordcloud", "Word cloud saved to wordcloud.png");
}
