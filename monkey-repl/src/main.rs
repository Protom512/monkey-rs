use std::io::{self, Write};

use monkey_lexer::Lexer;

const PROMPT: &str = ">> ";

fn main() -> io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("{}", PROMPT);
        stdout.flush()?; // プロンプトを表示
        input.clear();
        stdin.read_line(&mut input)?; // ユーザーからの入力を受け取る

        let mut lexer = Lexer::new(input.clone());
        loop {
            let tok = lexer.NextToken();
            if lexer.endofinput() {
                break;
            }
            // トークンを表示
            writeln!(stdout, "{:?}", tok)?;
        }
    }
}
