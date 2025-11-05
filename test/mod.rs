macro_rules! parser_test {
    ($input:literal => $output:literal) => {
        const INPUT: &'static str = $input;
        const OUTPUT: &'static str = $output;
        let mut reader = std::io::BufReader::new(INPUT.as_bytes());
        let result = parser::parse(&mut reader);
        assert_eq!(result, OUTPUT);
    };
}

pub mod code;
pub mod emphasis;
pub mod headings;
