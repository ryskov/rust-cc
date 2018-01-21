use Token;
use Keyword;
use AST;
use Symbol;
use StatementType;
use UnaryOperator;
use BinaryOperator;

struct Parser {
    tokens: Vec<Token>
} 

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens
        }
    }

    fn next_token(&mut self) -> Token {
        let mut token = self.tokens.remove(0);

        loop {
            if token != Token::Space && token != Token::NewLine {
                break
            }

            token = self.tokens.remove(0);
        }
        
        token
    }

    fn peek(&mut self) -> Token {
        let mut idx = 0;

        loop {
            let token = &self.tokens[idx];

            if token != &Token::Space && token != &Token::NewLine {
                return (*token).clone();
            }

            idx += 1;
        }
    }

    fn next_is_space(&mut self) -> bool {
        let token = &self.tokens[0];
        
        match token {
            &Token::NewLine => true,
            &Token::Space => true,
            _ => false
        }
    }

    fn parse_program(&mut self) -> AST {
        let function = self.parse_function();

        AST::new(Symbol::Program, vec![function])
    }

    fn parse_function(&mut self) -> AST {
        let token = self.next_token();

        if token != Token::Keyword(Keyword::Int) {
            panic!("Invalid function, expected 'int', got {:?}", token);
        }

        let token = self.next_token();

        let function_name = match token {
            Token::Identifier(value) => value,
            _ => panic!(format!("Expected identifier, got: {:?}", token))
        };

        let token = self.next_token();

        if token != Token::OpenParen {
            panic!("Expected '(', got {:?}", token);
        }

        let token = self.next_token();

        if token != Token::CloseParen {
            panic!("Expected ')', got {:?}", token);
        }

        let token = self.next_token();

        if token != Token::OpenBrace {
            panic!("Expected '{{', got {:?}", token);
        }

        let statement = self.parse_statement();

        let token = self.next_token();

        if token != Token::CloseBrace {
            panic!("Expected '}}', got {:?}", token);
        }

        AST::new(Symbol::Function(function_name), vec![statement])
    }

    fn parse_statement(&mut self) -> AST {
        let token = self.next_token();

        if token != Token::Keyword(Keyword::Return) {
            panic!("Expected 'return', got {:?}", token);
        }

        if self.next_is_space() == false {
            panic!("Expected whitespace");
        }

        let expression = self.parse_expression();

        let token = self.next_token();

        if token != Token::Semicolon {
            panic!("Expected ';', got {:?}", token);
        }

        AST::new(Symbol::Statement(StatementType::Return), vec![expression])
    } 
    
    fn parse_expression(&mut self) -> AST {
        let logical_and_expression = self.parse_logical_and_expression();

        let mut last_result: Option<AST> = None;

        let mut next = self.peek();

        while next == Token::LogicalOr {
            let token = self.next_token();
            let next_logical_and_expression = self.parse_logical_and_expression();

            let first_child = match last_result {
                Some(ref ast) => (*ast).clone(),
                None => logical_and_expression.clone()
            };

            last_result = Some(AST::new(Symbol::Expression, vec![first_child, next_logical_and_expression]));

            next = self.peek();
        }

        match last_result {
            Some(ast) => ast,
            None => logical_and_expression
        }
    }

    fn parse_logical_and_expression(&mut self) -> AST {
        let equality_expression = self.parse_equality_expression();

        let mut last_result: Option<AST> = None;

        let mut next = self.peek();

        while next == Token::LogicalAnd {
            let token = self.next_token();
            let next_equality_expression = self.parse_equality_expression();

            let first_child = match last_result {
                Some(ref ast) => (*ast).clone(),
                None => equality_expression.clone()
            };

            last_result = Some(AST::new(Symbol::BinaryOperator(BinaryOperator::LogicalAnd), vec![first_child, next_equality_expression]));

            next = self.peek();
        }

        match last_result {
            Some(ast) => ast,
            None => equality_expression
        }
    }

    fn parse_equality_expression(&mut self) -> AST {
        let relational_expression = self.parse_relational_expression();

        let mut last_result: Option<AST> = None;

        let mut next = self.peek();

        while next == Token::Equal || next == Token::NotEqual {
            let token = self.next_token();
            let next_relational_expression = self.parse_relational_expression();

            let first_child = match last_result {
                Some(ref ast) => (*ast).clone(),
                None => relational_expression.clone()
            };

            last_result = match token {
                Token::Equal => {
                    Some(AST::new(Symbol::BinaryOperator(BinaryOperator::Equal), vec![first_child, next_relational_expression]))
                },
                Token::NotEqual => {
                    Some(AST::new(Symbol::BinaryOperator(BinaryOperator::NotEqual), vec![first_child, next_relational_expression]))
                },
                _ => { panic!("Could not parse {:?} in expression", token); }
            };

            next = self.peek();
        }

        match last_result {
            Some(ast) => ast,
            None => relational_expression
        }
    }

    fn parse_relational_expression(&mut self) -> AST {
        let additive_expression = self.parse_additive_expression();

        let mut last_result: Option<AST> = None;

        let mut next = self.peek();

        while next == Token::LessThan || next == Token::LessThanOrEqual || next == Token::GreaterThan || next == Token::GreaterThanOrEqual {
            let token = self.next_token();
            let next_additive_expression = self.parse_additive_expression();

            let first_child = match last_result {
                Some(ref ast) => (*ast).clone(),
                None => additive_expression.clone()
            };

            last_result = match token {
                Token::LessThan => {
                    Some(AST::new(Symbol::BinaryOperator(BinaryOperator::LessThan), vec![first_child, next_additive_expression]))
                },
                Token::LessThanOrEqual => {
                    Some(AST::new(Symbol::BinaryOperator(BinaryOperator::LessThanOrEqual), vec![first_child, next_additive_expression]))
                },
                Token::GreaterThan => {
                    Some(AST::new(Symbol::BinaryOperator(BinaryOperator::GreaterThan), vec![first_child, next_additive_expression]))
                },
                Token::GreaterThanOrEqual => {
                    Some(AST::new(Symbol::BinaryOperator(BinaryOperator::GreaterThanOrEqual), vec![first_child, next_additive_expression]))
                },
                _ => { panic!("Could not parse {:?} in expression", token); }
            };

            next = self.peek();
        }

        match last_result {
            Some(ast) => ast,
            None => additive_expression
        }
    }

    fn parse_additive_expression(&mut self) -> AST {
        let term = self.parse_term();

        let mut last_result: Option<AST> = None;

        let mut next = self.peek();

        while next == Token::Addition || next == Token::Minus {
            let token = self.next_token();
            let next_term = self.parse_term();

            let first_child = match last_result {
                Some(ref ast) => (*ast).clone(),
                None => term.clone()
            };

            last_result = match token {
                Token::Addition => {
                    Some(AST::new(Symbol::BinaryOperator(BinaryOperator::Addition), vec![first_child, next_term]))
                },
                Token::Minus => {
                    Some(AST::new(Symbol::BinaryOperator(BinaryOperator::Subtraction), vec![first_child, next_term]))
                },
                _ => { panic!("Could not parse {:?} in expression", token); }
            };

            next = self.peek();
        }

        // match last_result {
        //     Some(ast) => AST::new(Symbol::Expression, vec![ast]),
        //     None => AST::new(Symbol::Expression, vec![term])
        // }

        match last_result {
            Some(ast) => ast,
            None => term
        }
    }

    fn parse_term(&mut self) -> AST {
        let factor = self.parse_factor();

        let mut last_result: Option<AST> = None;

        let mut next = self.peek();

        while next == Token::Multiplication || next == Token::Division {
            let token = self.next_token();
            let next_factor = self.parse_factor();

            let first_child = match last_result {
                Some(ref ast) => (*ast).clone(),
                None => factor.clone()
            };

            last_result = match token {
                Token::Division => {
                    Some(AST::new(Symbol::BinaryOperator(BinaryOperator::Division), vec![first_child, next_factor]))
                },
                Token::Multiplication => {
                    Some(AST::new(Symbol::BinaryOperator(BinaryOperator::Multiplication), vec![first_child, next_factor]))
                },
                _ => { panic!("Could not parse {:?} in term", token); }
            };

            next = self.peek();
        }

        match last_result {
            Some(ast) => ast,
            None => factor
        }

        // match last_result {
        //     Some(ast) => AST::new(Symbol::Term, vec![ast]),
        //     None => AST::new(Symbol::Term, vec![factor])
        // }
    }

    fn parse_factor(&mut self) -> AST {
        let token = self.next_token();

        match token {
            Token::OpenParen => {
                let expression = self.parse_expression();

                let token = self.next_token();

                if token != Token::CloseParen {
                    panic!("Expected ')', but got {:?}", token);
                }

                expression
                // AST::new(Symbol::Factor, vec![expression])
            },
            Token::Minus | Token::BitwiseComplementOperator | Token::LogicalNegationOperator => {
                let factor = self.parse_factor();

                let unary_operation = match &token {
                    &Token::Minus => UnaryOperator::Negation,
                    &Token::BitwiseComplementOperator => UnaryOperator::BitwiseComplement,
                    &Token::LogicalNegationOperator => UnaryOperator::LogicalNegation,
                    _ => panic!("Should never go here")
                };

                let unary_operation = AST::new(Symbol::UnaryOperator(unary_operation), vec![factor]);

                unary_operation
                // AST::new(Symbol::Factor, vec![unary_operation])
            },
            Token::IntegerLiteral(value) => {
                let integer = AST::new(Symbol::Constant(value), Vec::new());
                integer
              //  AST::new(Symbol::Factor, vec![integer])
            },
            _ => { panic!("Invalid factor {:?}", token); }
        }
    }

    pub fn parse(&mut self) -> AST {
       self.parse_program()
    } 
}  

pub fn parse(tokens: Vec<Token>) -> AST {
    let mut parser = Parser::new(tokens);

    parser.parse()
}