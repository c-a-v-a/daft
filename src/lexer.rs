#[derive(Debug)]
pub enum Token {
    Newline,
    AtxHeading(usize),
    SetextHeading(usize),
    Text(Vec<char>),
    Link(Vec<char>),
}

fn atx_heading_token(input: Vec<char>) -> Token {
    if input.iter().find(|c| **c != '#').is_none() && input.len() <= 6 {
        Token::AtxHeading(input.len())
    } else {
        Token::Text(input)
    }
}

fn setext_heading_token(input: Vec<char>, rest: &Vec<&str>) -> Token {
    if rest.len() > 1 {
        return Token::Text(input);
    }

    let x = input[0];
    let level = if x == '=' { 1 } else { 2 };

    if input.iter().find(|c| **c != x).is_none() {
        Token::SetextHeading(level)
    } else {
        Token::Text(input)
    }
}

fn link_token(input: Vec<char>) -> Token {
    if input[input.len() - 1] == '}' {
        let link_text = input[1..input.len() - 1].to_vec();

        Token::Link(link_text)
    } else {
        Token::Text(input)
    }
}

fn tokenize_word(word: Vec<char>, words: &Vec<&str>) -> Token {
    if word[0] == '\0' {
        return Token::Newline;
    }

    match word[0] {
        '#' => atx_heading_token(word),
        '=' | '-' => setext_heading_token(word, words),
        '{' => link_token(word),
        _ => Token::Text(word),
    }
}

pub fn tokenize_line(line: String) -> Vec<Token> {
    let words: Vec<&str> = line.split_whitespace().collect();

    words
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .map(|word| tokenize_word(word, &words))
        .collect()
}
