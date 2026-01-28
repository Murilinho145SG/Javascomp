use javascomp::{
    lexer::{lexer, TokenKind},
};

fn main() {
	// TESTING
    let code = "let x = 10;";
    let mut l = lexer::Lexer::new(code);
    loop {
        let token = l.next_token();
        if token.token == TokenKind::EndOfFile {
            break;
        }
        println!("{:?}", token);
    }
}
