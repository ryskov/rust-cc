use Token;
use Keyword;

#[derive(Debug)]
struct Lexer {
    buf: String,
    char_indices: Vec<(usize, char)>,
    len: usize,
    pos: usize
}

impl Lexer {
    pub fn new(buf: String) -> Lexer {
        let char_indices: Vec<(usize, char)> = buf.char_indices().collect();
        let len = char_indices.len();

        Lexer {
            buf,
            char_indices,
            len,
            pos: 0
        }
    }

    fn scan_until<F>(&mut self, mut current_byte_offset: usize, at_end_function: F) -> usize where F: Fn(char) -> bool {
        self.pos += 1;

        loop {
            if self.pos > self.len {
                if self.pos > 0 {
                    current_byte_offset += self.char_indices[self.pos - 1].1.len_utf8();
                }
                break
            }

            let indice = self.char_indices[self.pos];
            current_byte_offset = indice.0;
            if at_end_function(indice.1) {
                break
            } 

            self.pos += 1;
        }

        current_byte_offset
    }

    pub fn lex(mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            if self.pos >= self.len {
                break
            }

            let (current_byte_offset, current_char) = self.char_indices[self.pos];

            // println!("current_byte_offset: {}\r\ncurrent_char: {}", current_byte_offset, current_char);

            let token: Token = match current_char {
                ' ' => {
                    self.pos += 1;
                    Token::Space
                },
                '\n' | '\r' => {
                    self.pos += 1;
                    Token::NewLine
                },
                '(' => {
                    self.pos += 1;
                    Token::OpenParen
                },
                ')' => {
                    self.pos += 1;
                    Token::CloseParen
                },
                '{' => {
                    self.pos += 1;
                    Token::OpenBrace
                },
                '}' => {
                    self.pos += 1;
                    Token::CloseBrace
                },
                ';' => {
                    self.pos +=1;
                    Token::Semicolon
                },
                '-' => {
                    self.pos += 1;
                    Token::Minus
                },
                '~' => {
                    self.pos += 1;
                    Token::BitwiseComplementOperator
                },
                '!' => {
                    self.pos += 1;
                    Token::LogicalNegationOperator
                },
                '+' => {
                    self.pos += 1;
                    Token::Addition
                },
                '*' => {
                    self.pos += 1;
                    Token::Multiplication
                },
                '/' => {
                    self.pos += 1;
                    Token::Division
                },
                c if c.is_alphabetic() => {
                    let end_byte_offset = self.scan_until(current_byte_offset, |c| {
                        match c {
                            c if c.is_alphabetic() => false,
                            _ => true
                        }
                    });

                    match &self.buf[current_byte_offset..end_byte_offset] {
                        "int" => Token::Keyword(Keyword::Int),
                        "return" => Token::Keyword(Keyword::Return),
                        _ => Token::Identifier(self.buf[current_byte_offset..end_byte_offset].to_string())
                    }
                },
                c if c.is_numeric() => {
                    let end_byte_offset = self.scan_until(current_byte_offset, |c| {
                        match c {
                            c if c.is_numeric() => false,
                            _ => true
                        }
                    });

                    Token::IntegerLiteral(self.buf[current_byte_offset..end_byte_offset].parse::<usize>().unwrap())
                }
                _ => Token::NewLine
            };

            tokens.push(token);
        }

        tokens
    }
}

pub fn lex(file_contents: String) -> Vec<Token> { 
    Lexer::new(file_contents).lex() 
}