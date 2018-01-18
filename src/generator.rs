use AST;

use NonTerminalSymbol;
use ExpressionType;
use StatementType;
use UnaryOperator;

struct Generator {
    buf: String,
    current_expression: Option<ExpressionType>
}

impl Generator {
    fn new() -> Generator {
        Generator {
            buf: String::new(),
            current_expression: None
        }
    }

    fn generate(mut self, ast: &AST) -> String {
        if ast.symbol != NonTerminalSymbol::Program {
            panic!("Root node in AST is not a program node\r\n{:#?}", ast);
        }

        let mut fun_index = 0;
        loop {
            if ast.children.len() == fun_index { break }

            let current_fun_node = &ast.children[fun_index];

            let fun_name = match &current_fun_node.symbol {
                &NonTerminalSymbol::Function(ref value) => value,
                _ => panic!("Not a function")
            };

            self.buf.push_str(&format!(".globl {}\n{}:\n", fun_name, fun_name)[..]);
            self.postorder_traversal(&current_fun_node);

            fun_index += 1;
        }

        self.buf
    }

    fn visit_node(&mut self, ast: &AST) {

        match ast.symbol {
            NonTerminalSymbol::Expression(ref expression_type) => {
                match expression_type {
                    &ExpressionType::Constant(value) => {
                        self.buf.push_str(&format!("movq ${}, %rax\n", value));
                    },
                    &ExpressionType::UnaryOperation(ref unary_operator) => {
                        match unary_operator {
                            &UnaryOperator::Negation => {
                                self.buf.push_str("neg %rax\n");
                            },
                            &UnaryOperator::BitwiseComplement => {
                                self.buf.push_str("not %rax\n");
                            },
                            &UnaryOperator::LogicalNegation => {
                                self.buf.push_str("cmpq $0, %rax\nmovq $0, %rax\nsete %al\n");
                            },
                            _ => { panic!("Unsupported unary operator"); }
                        } 
                    }
                }
                self.current_expression = Some(expression_type.clone());
            },
            NonTerminalSymbol::Statement(ref statement_type) => {
                match statement_type {
                    &StatementType::Return => {
                        self.buf.push_str("ret\r\n");
                    },
                    _ => { panic!("Unsupported statement type: {:#?}", statement_type); }
                }
            },
            _ => { }
        };
    }

    fn postorder_traversal(&mut self, ast: &AST) {
        for child in &ast.children {
            self.postorder_traversal(child);
        }
        
        self.visit_node(ast);
    }
}

pub fn generate(ast: AST) -> String {
    let generator = Generator::new();

    generator.generate(&ast)
}