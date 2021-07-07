use crate::scanner::*;
use num_traits::ToPrimitive;

pub fn compile(source: &String) {
    let mut scanner = Scanner::new(source);
    let mut line = 0;

    loop {
        let token = scanner.scan();
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        let text = std::str::from_utf8(&scanner.source[token.start..token.start+token.length]).unwrap();
        let token_byte = ToPrimitive::to_u8(&token.token_type).unwrap();
        println!("{:2} '{}'", token_byte, text);

        if matches!(token.token_type, TokenType::EOF) {
            break;
        }
    }
}
