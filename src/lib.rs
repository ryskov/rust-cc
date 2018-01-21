pub mod lexer;
pub mod parser;
pub mod generator;

#[derive(Debug,Clone)]
pub struct AST {
    symbol: Symbol,
    children: Vec<AST>
}

impl AST {
    fn new(symbol: Symbol, children: Vec<AST>) -> AST {
        AST {
            symbol,
            children
        }
    } 
} 

#[derive(Debug,PartialEq,Clone)]
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
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    LogicalAnd,
    LogicalOrfnex
}

#[derive(Debug,PartialEq,Clone)]
pub enum Symbol {
    Program,
    Function(String),
    Statement(StatementType),
    Expression,
    LogicalAndExpression,
    EqualityExpression,
    RelationalExpression,
    AdditiveExpression,
    Term,
    Factor,
    Constant(usize),
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator)
}

#[derive(Debug,PartialEq,Clone)]
pub enum Keyword {
    Int,
    Return
}

#[derive(Debug,PartialEq,Clone)]
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
    Minus,
    BitwiseComplementOperator,
    LogicalNegationOperator,
    Addition,
    Multiplication,
    Division,
    LogicalAnd,
    BitwiseAnd,
    LogicalOr,
    BitwiseOr,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual
}