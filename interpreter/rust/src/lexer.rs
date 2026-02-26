use crate::constants::*;
use crate::token::Token;
use phf::phf_map;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: u32,
    read_position: u32,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;

        self.eat_whitespace();

        match self.ch {
            b',' => {
                tok = new_token(COMMA.to_string(), self.ch);
            }
            b';' => {
                tok = new_token(SEMICOLON.to_string(), self.ch);
            }
            b'(' => {
                tok = new_token(LPAREN.to_string(), self.ch);
            }
            b')' => {
                tok = new_token(RPAREN.to_string(), self.ch);
            }
            b'{' => {
                tok = new_token(LBRACE.to_string(), self.ch);
            }
            b'}' => {
                tok = new_token(RBRACE.to_string(), self.ch);
            }
            b'=' => {
                tok = new_token(ASSIGN.to_string(), self.ch);
            }
            b'+' => {
                tok = new_token(PLUS.to_string(), self.ch);
            }
            b'-' => {
                tok = new_token(MINUS.to_string(), self.ch);
            }
            b'!' => {
                tok = new_token(BANG.to_string(), self.ch);
            }
            b'*' => {
                tok = new_token(ASTERISK.to_string(), self.ch);
            }
            b'/' => {
                tok = new_token(SLASH.to_string(), self.ch);
            }
            b'<' => {
                tok = new_token(LT.to_string(), self.ch);
            }
            b'>' => {
                tok = new_token(GT.to_string(), self.ch);
            }
            0 => {
                tok = Token {
                    token_type: EOF.to_string(),
                    literal: String::new(),
                }
            }
            _ => {
                // How to read two chars such as ==
                if is_letter(self.ch as char) {
                    let identifier = self.read_identifier();
                    tok = Token {
                        token_type: lookup_identifier(&identifier),
                        literal: identifier,
                    };
                } else if is_number(self.ch as char) {
                    let number = self.read_number();
                    tok = Token {
                        token_type: INT.to_string(),
                        literal: number,
                    };
                } else {
                    tok = Token {
                        token_type: ILLEGAL.to_string(),
                        literal: (self.ch as char).to_string(),
                    };
                }
                // If we don't return early, end up eating a char via read_char called at the end
                return tok;
            }
        }

        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        println!("char: {}", self.ch as char);

        if self.read_position >= self.input.len() as u32 {
            self.ch = 0
        } else {
            self.ch = self.input.as_bytes()[self.read_position as usize]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let mut identifer: String = String::new();
        while is_letter(self.ch as char) {
            identifer.push(self.ch as char);
            self.read_char();
        }
        identifer
    }

    fn read_number(&mut self) -> String {
        let mut number: String = String::new();
        while is_number(self.ch as char) {
            number.push(self.ch as char);
            self.read_char();
        }
        number
    }

    fn eat_whitespace(&mut self) {
        while self.ch as char == ' '
            || self.ch as char == '\t'
            || self.ch as char == '\n'
            || self.ch as char == '\r'
        {
            let _x = &self.read_char();
        }
    }
}

fn lookup_identifier(identifier: &String) -> String {
    KEYWORDS.get(identifier).unwrap_or(&IDENT).to_string()
}

fn new_token(token_type: String, literal_byte: u8) -> Token {
    Token {
        token_type,
        literal: (literal_byte as char).to_string(),
    }
}

static KEYWORDS: phf::Map<&'static str, &'static str> = phf_map! {
    "fn" => FUNCTION,
    "let" => LET,
    "true" => TRUE,
    "false" => FALSE,
    "if" => IF,
    "else" => ELSE,
    "return" => RETURN,
};

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z'
}

fn is_number(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCase {
        expected_token: &'static str,
        expected_literal: &'static str,
    }

    #[test]
    fn test_individual_chars() {
        let mut lexer = Lexer::new(",;(){}=+ ".to_string());

        let tests = [
            TestCase {
                expected_token: COMMA,
                expected_literal: ",",
            },
            TestCase {
                expected_token: SEMICOLON,
                expected_literal: ";",
            },
            TestCase {
                expected_token: LPAREN,
                expected_literal: "(",
            },
            TestCase {
                expected_token: RPAREN,
                expected_literal: ")",
            },
            TestCase {
                expected_token: LBRACE,
                expected_literal: "{",
            },
            TestCase {
                expected_token: RBRACE,
                expected_literal: "}",
            },
            TestCase {
                expected_token: ASSIGN,
                expected_literal: "=",
            },
            TestCase {
                expected_token: PLUS,
                expected_literal: "+",
            },
            TestCase {
                expected_token: EOF,
                expected_literal: "",
            },
        ];

        for test_case in tests.iter() {
            let token = lexer.next_token();

            assert_eq!(token.token_type, test_case.expected_token);
            assert_eq!(token.literal, test_case.expected_literal);
        }
    }

    #[test]
    fn test_code_block() {
        let mut lexer = Lexer::new(
            "let five = 5;\
            let ten = 10;\
            \
            let add = fn(x, y) {\
                x + y;\
            };
            \
            let result = add(five, ten);\
            !-/*5;\
            5 < 10 > 5;\
            \
            if (5 < 10) {\
                return true;\
            } else {\
                return false;\
            }"
            .to_string(),
        );

        let tests = [
            TestCase {
                expected_token: LET,
                expected_literal: "let",
            },
            TestCase {
                expected_token: IDENT,
                expected_literal: "five",
            },
            TestCase {
                expected_token: ASSIGN,
                expected_literal: "=",
            },
            TestCase {
                expected_token: INT,
                expected_literal: "5",
            },
            TestCase {
                expected_token: SEMICOLON,
                expected_literal: ";",
            },
            TestCase {
                expected_token: LET,
                expected_literal: "let",
            },
            TestCase {
                expected_token: IDENT,
                expected_literal: "ten",
            },
            TestCase {
                expected_token: ASSIGN,
                expected_literal: "=",
            },
            TestCase {
                expected_token: INT,
                expected_literal: "10",
            },
            TestCase {
                expected_token: SEMICOLON,
                expected_literal: ";",
            },
            TestCase {
                expected_token: LET,
                expected_literal: "let",
            },
            TestCase {
                expected_token: IDENT,
                expected_literal: "add",
            },
            TestCase {
                expected_token: ASSIGN,
                expected_literal: "=",
            },
            TestCase {
                expected_token: FUNCTION,
                expected_literal: "fn",
            },
            TestCase {
                expected_token: LPAREN,
                expected_literal: "(",
            },
            TestCase {
                expected_token: IDENT,
                expected_literal: "x",
            },
            TestCase {
                expected_token: COMMA,
                expected_literal: ",",
            },
            TestCase {
                expected_token: IDENT,
                expected_literal: "y",
            },
            TestCase {
                expected_token: RPAREN,
                expected_literal: ")",
            },
            TestCase {
                expected_token: LBRACE,
                expected_literal: "{",
            },
            TestCase {
                expected_token: IDENT,
                expected_literal: "x",
            },
            TestCase {
                expected_token: PLUS,
                expected_literal: "+",
            },
            TestCase {
                expected_token: IDENT,
                expected_literal: "y",
            },
            TestCase {
                expected_token: SEMICOLON,
                expected_literal: ";",
            },
            TestCase {
                expected_token: RBRACE,
                expected_literal: "}",
            },
            TestCase {
                expected_token: SEMICOLON,
                expected_literal: ";",
            },
            TestCase {
                expected_token: LET,
                expected_literal: "let",
            },
            TestCase {
                expected_token: IDENT,
                expected_literal: "result",
            },
            TestCase {
                expected_token: ASSIGN,
                expected_literal: "=",
            },
            TestCase {
                expected_token: IDENT,
                expected_literal: "add",
            },
            TestCase {
                expected_token: LPAREN,
                expected_literal: "(",
            },
            TestCase {
                expected_token: IDENT,
                expected_literal: "five",
            },
            TestCase {
                expected_token: COMMA,
                expected_literal: ",",
            },
            TestCase {
                expected_token: IDENT,
                expected_literal: "ten",
            },
            TestCase {
                expected_token: RPAREN,
                expected_literal: ")",
            },
            TestCase {
                expected_token: SEMICOLON,
                expected_literal: ";",
            },
            TestCase {
                expected_token: BANG,
                expected_literal: "!",
            },
            TestCase {
                expected_token: MINUS,
                expected_literal: "-",
            },
            TestCase {
                expected_token: SLASH,
                expected_literal: "/",
            },
            TestCase {
                expected_token: ASTERISK,
                expected_literal: "*",
            },
            TestCase {
                expected_token: INT,
                expected_literal: "5",
            },
            TestCase {
                expected_token: SEMICOLON,
                expected_literal: ";",
            },
            TestCase {
                expected_token: INT,
                expected_literal: "5",
            },
            TestCase {
                expected_token: LT,
                expected_literal: "<",
            },
            TestCase {
                expected_token: INT,
                expected_literal: "10",
            },
            TestCase {
                expected_token: GT,
                expected_literal: ">",
            },
            TestCase {
                expected_token: INT,
                expected_literal: "5",
            },
            TestCase {
                expected_token: SEMICOLON,
                expected_literal: ";",
            },
            TestCase {
                expected_token: IF,
                expected_literal: "if",
            },
            TestCase {
                expected_token: LPAREN,
                expected_literal: "(",
            },
            TestCase {
                expected_token: INT,
                expected_literal: "5",
            },
            TestCase {
                expected_token: LT,
                expected_literal: "<",
            },
            TestCase {
                expected_token: INT,
                expected_literal: "10",
            },
            TestCase {
                expected_token: RPAREN,
                expected_literal: ")",
            },
            TestCase {
                expected_token: LBRACE,
                expected_literal: "{",
            },
            TestCase {
                expected_token: RETURN,
                expected_literal: "return",
            },
            TestCase {
                expected_token: TRUE,
                expected_literal: "true",
            },
            TestCase {
                expected_token: SEMICOLON,
                expected_literal: ";",
            },
            TestCase {
                expected_token: RBRACE,
                expected_literal: "}",
            },
            TestCase {
                expected_token: ELSE,
                expected_literal: "else",
            },
            TestCase {
                expected_token: LBRACE,
                expected_literal: "{",
            },
            TestCase {
                expected_token: RETURN,
                expected_literal: "return",
            },
            TestCase {
                expected_token: FALSE,
                expected_literal: "false",
            },
            TestCase {
                expected_token: SEMICOLON,
                expected_literal: ";",
            },
            TestCase {
                expected_token: RBRACE,
                expected_literal: "}",
            },
            TestCase {
                expected_token: EOF,
                expected_literal: "",
            },
        ];

        for test_case in tests.iter() {
            let token = lexer.next_token();

            assert_eq!(token.token_type, test_case.expected_token);
            assert_eq!(token.literal, test_case.expected_literal);
        }
    }
}
