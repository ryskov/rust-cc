pub mod lexer;
pub mod parser;
pub mod generator;

#[derive(Debug)]
pub struct AST {
    symbol: NonTerminalSymbol,
    children: Vec<AST>
}

impl AST {
    fn new(symbol: NonTerminalSymbol, children: Vec<AST>) -> AST {
        AST {
            symbol,
            children
        }
    } 
} 

#[derive(Debug,PartialEq)]
pub enum StatementType {
    Return,
    VariableDeclaration,
    VariableAssignment
}

#[derive(Debug,PartialEq,Clone)]
pub enum UnaryOperator {
    Negation,
    BitwiseComplement,
    LogicalNegation
}

#[derive(Debug,PartialEq,Clone)]
pub enum ExpressionType {
    Constant(usize),
    UnaryOperation(UnaryOperator)
}

#[derive(Debug,PartialEq)]
pub enum NonTerminalSymbol {
    Program,
    Function(String),
    Statement(StatementType),
    Expression(ExpressionType)
}

#[derive(Debug,PartialEq)]
pub enum Keyword {
    Int,
    Return
}

#[derive(Debug,PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
    Keyword(Keyword),
    Identifier(String),
    IntegerLiteral(usize),
    NewLine,
    Space,
    NegationOperator,
    BitwiseComplementOperator,
    LogicalNegationOperator
}