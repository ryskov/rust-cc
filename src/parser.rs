use Token;
use Keyword;
use AST;
use NonTerminalSymbol;
use StatementType;
use ExpressionType;
use UnaryOperator;

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

    fn parse_program(&mut self) -> AST {
        let function = self.parse_function();

        AST::new(NonTerminalSymbol::Program, vec![function])
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

        AST::new(NonTerminalSymbol::Function(function_name), vec![statement])
    }

    fn next_is_space(&mut self) -> bool {
        let token = &self.tokens[0];
        
        match token {
            &Token::NewLine => true,
            &Token::Space => true,
            _ => false
        }
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

        AST::new(NonTerminalSymbol::Statement(StatementType::Return), vec![expression])
    } 

    fn parse_expression(&mut self) -> AST {
        let token = self.next_token();

        match token {
            Token::IntegerLiteral(value) => {
                AST::new(NonTerminalSymbol::Expression(ExpressionType::Constant(value)), Vec::new())
            },
            Token::NegationOperator => {
                let expression = self.parse_expression();
                AST::new(NonTerminalSymbol::Expression(ExpressionType::UnaryOperation(UnaryOperator::Negation)), vec![expression])
            },
            Token::BitwiseComplementOperator => {
                let expression = self.parse_expression();
                AST::new(NonTerminalSymbol::Expression(ExpressionType::UnaryOperation(UnaryOperator::BitwiseComplement)), vec![expression])
            },
            Token::LogicalNegationOperator => {
                let expression = self.parse_expression();
                AST::new(NonTerminalSymbol::Expression(ExpressionType::UnaryOperation(UnaryOperator::LogicalNegation)), vec![expression])
            },
            _ => panic!("Invalid expression: {:?}", token)
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