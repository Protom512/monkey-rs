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
    #[test]
    fn test_next_token() {
        let input = r"let five=5;
        let ten=10;
        
        let add=fn(x,y){
        x+y;
    };
    
    let result=add(five,ten);
    !-/*5;
    5<10>5;";
        let mut l = Lexer::new(input.to_string());
        let expecteds: Vec<Token>;
        expecteds = vec![
            Token {
                Type: LET.to_string(),
                Literal: "let".to_string(),
            },
            Token {
                Type: IDENT.to_string(),
                Literal: "five".to_string(),
            },
            Token {
                Type: ASSIGN.to_string(),
                Literal: "=".to_string(),
            },
            Token {
                Type: INT.to_string(),
                Literal: "5".to_string(),
            },
            Token {
                Type: SEMICOLON.to_string(),
                Literal: ";".to_string(),
            },
            Token {
                Type: LET.to_string(),
                Literal: "let".to_string(),
            },
            Token {
                Type: IDENT.to_string(),
                Literal: "ten".to_string(),
            },
            Token {
                Type: ASSIGN.to_string(),
                Literal: "=".to_string(),
            },
            Token {
                Type: INT.to_string(),
                Literal: "10".to_string(),
            },
            Token {
                Type: SEMICOLON.to_string(),
                Literal: ";".to_string(),
            },
            Token {
                Type: LET.to_string(),
                Literal: "let".to_string(),
            },
            Token {
                Type: IDENT.to_string(),
                Literal: "add".to_string(),
            },
            Token {
                Type: ASSIGN.to_string(),
                Literal: "=".to_string(),
            },
            Token {
                Type: FUNCTION.to_string(),
                Literal: "fn".to_string(),
            },
            Token {
                Type: LPAREN.to_string(),
                Literal: "(".to_string(),
            },
            Token {
                Type: IDENT.to_string(),
                Literal: "x".to_string(),
            },
            Token {
                Type: COMMA.to_string(),
                Literal: ",".to_string(),
            },
            Token {
                Type: IDENT.to_string(),
                Literal: "y".to_string(),
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
                Type: IDENT.to_string(),
                Literal: "x".to_string(),
            },
            Token {
                Type: PLUS.to_string(),
                Literal: "+".to_string(),
            },
            Token {
                Type: IDENT.to_string(),
                Literal: "y".to_string(),
            },
            Token {
                Type: SEMICOLON.to_string(),
                Literal: ";".to_string(),
            },
            Token {
                Type: RBRACE.to_string(),
                Literal: "}".to_string(),
            },
            Token {
                Type: SEMICOLON.to_string(),
                Literal: ";".to_string(),
            },
            // let result=add(five,ten);
            Token {
                Type: LET.to_string(),
                Literal: "let".to_string(),
            },
            Token {
                Type: IDENT.to_string(),
                Literal: "result".to_string(),
            },
            Token {
                Type: ASSIGN.to_string(),
                Literal: "=".to_string(),
            },
            Token {
                Type: IDENT.to_string(),
                Literal: "add".to_string(),
            },
            Token {
                Type: LPAREN.to_string(),
                Literal: "(".to_string(),
            },
            Token {
                Type: IDENT.to_string(),
                Literal: "five".to_string(),
            },
            Token {
                Type: COMMA.to_string(),
                Literal: ",".to_string(),
            },
            Token {
                Type: IDENT.to_string(),
                Literal: "ten".to_string(),
            },
            Token {
                Type: RPAREN.to_string(),
                Literal: ")".to_string(),
            },
            Token {
                Type: SEMICOLON.to_string(),
                Literal: ";".to_string(),
            },
            //   !-/*5;
            Token {
                Type: BANG.to_string(),
                Literal: "!".to_string(),
            },
            Token {
                Type: MINUS.to_string(),
                Literal: "-".to_string(),
            },
            Token {
                Type: SLASH.to_string(),
                Literal: "/".to_string(),
            },
            Token {
                Type: ASTERISK.to_string(),
                Literal: "*".to_string(),
            },
            Token {
                Type: INT.to_string(),
                Literal: "5".to_string(),
            },

            Token {
                Type: SEMICOLON.to_string(),
                Literal: ";".to_string(),
            },
        ];

        for expected in expecteds {
            let token = l.NextToken();
            dbg!(&token);
            assert_eq!(expected, token);
        }
    }
 
}
