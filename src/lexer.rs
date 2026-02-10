type LexData = (Lexeme, std::ops::Range<usize>);

#[derive(Debug)]
pub enum Lexeme {
    Cursive,
    Newline,

    Text
}

enum State {
    Normal
}

pub fn lex(input: &str) -> Vec<LexData> {
    let mut lexemes = vec![];
    let mut jump = 0usize;
    let mut ptr = 0usize;
    let mut state = State::Normal;

    while ptr + 1 < input.len() {
        let cursor = &input[ptr..ptr+1];
        let peek = if ptr + 1 < input.len() {
            &input[ptr+1..ptr+2] 
        } else {
            " "
        };
        match cursor {
            "/" if peek == "/" => {
                // all before was text; so
                lexemes.push((Lexeme::Text, jump..ptr));
                jump = ptr;
                lexemes.push((Lexeme::Cursive, jump..ptr+2));
                ptr += 2;
                jump = ptr;
                continue;
            },
            _ => {},
        }
        ptr += 1;
    }

    lexemes
}
