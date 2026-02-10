pub type LexData = (Lexeme, std::ops::Range<usize>);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lexeme {
    Cursive,
    Bold,
    Underline,
    Deemphasised,

    Newline,

    Text
}

enum State {
    Normal,
    
    Newline,
    Escape,
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
        match state {
            State::Normal => match cursor {
                // basic emphasis
                "/" if peek == "/" => {
                    // all before was text; so
                    lexemes.push((Lexeme::Text, jump..ptr));
                    jump = ptr;
                    lexemes.push((Lexeme::Cursive, jump..ptr+2));
                    ptr += 2;
                    jump = ptr;
                    continue;
                },

                "!" if peek == "!" => {
                    // all before was text; so
                    lexemes.push((Lexeme::Text, jump..ptr));
                    jump = ptr;
                    lexemes.push((Lexeme::Bold, jump..ptr+2));
                    ptr += 2;
                    jump = ptr;
                    continue;
                },

                "_" if peek == "_" => {
                    // all before was text; so
                    lexemes.push((Lexeme::Text, jump..ptr));
                    jump = ptr;
                    lexemes.push((Lexeme::Underline, jump..ptr+2));
                    ptr += 2;
                    jump = ptr;
                    continue;
                },

                "?" if peek == "?" => {
                    // all before was text; so
                    lexemes.push((Lexeme::Text, jump..ptr));
                    jump = ptr;
                    lexemes.push((Lexeme::Deemphasised, jump..ptr+2));
                    ptr += 2;
                    jump = ptr;
                    continue;
                },

                "\n" => {
                    lexemes.push((Lexeme::Text, jump..ptr));
                    jump = ptr;
                    lexemes.push((Lexeme::Newline, jump..ptr+1));
                    ptr += 1;
                    jump = ptr;
                    state = State::Newline;
                    continue;
                },
                _ => {},
            },
            State::Newline => match cursor {
                "\n" => {
                    lexemes.push((Lexeme::Newline, jump..ptr+1));
                    ptr += 1;
                    jump = ptr;
                    continue;
                }
                _ => state = State::Normal,
            }
            _ => todo!(),
    }
        ptr += 1;
    }
    lexemes.push((Lexeme::Text, jump..ptr));

    lexemes
}
