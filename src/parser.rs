use crate::lexer::Token;

#[derive(Debug)]
pub struct Heading {
    pub level: usize,
    pub text: String,
    pub link: String,
}

fn heading_text(tokens: &Vec<Token>) -> String {
    tokens
        .into_iter()
        .filter_map(|x| {
            if let Token::Text(t) = x {
                Some(t)
            } else {
                None
            }
        })
        .map(|x| x.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

fn text_to_link(text: &String) -> String {
    "#".to_string()
        + &text
            .split(' ')
            .collect::<Vec<&str>>()
            .join("-")
            .to_lowercase()
            .to_string()
}

pub fn atx_heading(tokens: &Vec<Token>, level: usize) -> Heading {
    let text: String = heading_text(tokens);

    let link: String = match &tokens[tokens.len() - 1] {
        Token::Link(link) => link.into_iter().collect::<String>(),
        _ => text_to_link(&text),
    };

    Heading {
        level: level,
        text: text,
        link: link,
    }
}

pub fn setext_heading(level: usize, text_tokens: &Vec<Token>) -> Heading {
    let text: String = heading_text(text_tokens);

    let link: String = match &text_tokens[text_tokens.len() - 1] {
        Token::Link(link) => link.into_iter().collect::<String>(),
        _ => text_to_link(&text),
    };

    Heading {
        level: level,
        text: text,
        link: link,
    }
}

pub fn parse(tokens: Vec<Vec<Token>>) -> Vec<Heading> {
    let mut prev: Vec<Token> = vec![];
    let mut headings: Vec<Heading> = vec![];

    for t in tokens {
        match t[0] {
            Token::AtxHeading(level) => headings.push(atx_heading(&t, level)),
            Token::SetextHeading(level) => headings.push(setext_heading(level, &prev)),
            _ => {}
        };

        prev = t;
    }

    headings
}
