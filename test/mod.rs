macro_rules! parser_tests {
    ($([$test_name:ident] { $input:literal -> $output:literal });*;) => {
        $(
            #[test]
            fn $test_name() {
                const INPUT: &'static str = $input;
                const OUTPUT: &'static str = $output;
                let mut reader = std::io::BufReader::new(INPUT.as_bytes());
                let result = parser::parse(&mut reader);
                assert_eq!(result.trim(), OUTPUT);
            }
        )*
    };
}

pub mod code;
pub mod emphasis;
pub mod headings;
pub mod html_entities;
pub mod image;
