#[cfg(test)]
mod tests {
    use monkey_lexer::*;
    use monkey_token::*;

    #[test]
    fn lexer_works() {
        let input = r"=+(){},;";

        let mut l = Lexer::new(input.to_string());
        let expecteds: Vec<Token>;
        expecteds = vec![
            Token {
                Type: ASSIGN.to_string(),
                Literal: "=".to_string(),
            },
            Token {
                Type: PLUS.to_string(),
                Literal: "+".to_string(),
            },
            Token {
                Type: LPAREN.to_string(),
                Literal: "(".to_string(),
            },
            Token {
                Type: RPAREN.to_string(),
                Literal: ")".to_string(),
            },
            Token {
                Type: LBRACE.to_string(),
                Literal: "{".to_string(),
            },
            Token {
                Type: RBRACE.to_string(),
                Literal: "}".to_string(),
            },
            Token {
                Type: COMMA.to_string(),
                Literal: ",".to_string(),
            },
            Token {
                Type: SEMICOLON.to_string(),
                Literal: ";".to_string(),
            },
        ];
        for expected in expecteds {
            let token = l.NextToken();
            assert_eq!(expected, token);
        }
    }

    fn test_next_token() {
        let input = r"let five=5;
        let ten=10;
        
        let add=fn(x,y){
        x+y;
    };
    
    let result=add(five,ten);";
        let mut l = Lexer::new(input.to_string());
        let expecteds: Vec<Token>;
        expecteds = vec![Token {
            Type: LET.to_string(),
            Literal: "let".to_string(),
        }]
    }
}
