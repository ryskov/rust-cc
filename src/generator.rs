use AST;

use Symbol;
use StatementType;
use UnaryOperator;
use BinaryOperator;

struct Generator {
    buf: String,
    expression_stack: Vec<String>
}

impl Generator {
    fn new() -> Generator {
        Generator {
            buf: String::new(),
            expression_stack: Vec::new()
        }
    }

    fn generate(mut self, ast: &AST) -> String {
        if ast.symbol != Symbol::Program {
            panic!("Root node in AST is not a program node\r\n{:#?}", ast);
        }

        let mut fun_index = 0;
        loop {
            if ast.children.len() == fun_index { break }

            let current_fun_node = &ast.children[fun_index];

            let fun_name = match &current_fun_node.symbol {
                &Symbol::Function(ref value) => value,
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
            Symbol::Constant(value) => {
                self.expression_stack.push(format!("movq ${}, %rax\n", value));
            },
            Symbol::UnaryOperator(ref unary_operator) => {
                match unary_operator {
                    &UnaryOperator::Negation => {
                        let op1 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}neg %rax\n", op1));
                    },
                    &UnaryOperator::BitwiseComplement => {
                        let op1 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}not %rax\n", op1));
                    },
                    &UnaryOperator::LogicalNegation => {
                        let op1 = self.expression_stack.pop().unwrap();
                        
                        self.expression_stack.push(format!("{}cmpq $0, %rax\nmovq $0, %rax\nsete %al\n", op1));
                    }
                };
            },
            Symbol::BinaryOperator(ref binary_operator) => {
                match binary_operator {
                    &BinaryOperator::Addition => {
                        let op2 = self.expression_stack.pop().unwrap();
                        let op1 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}push %rax\n{}pop %rbx\naddq %rbx, %rax\n", op1, op2));
                    },
                    &BinaryOperator::Multiplication => {
                        let op2 = self.expression_stack.pop().unwrap();
                        let op1 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}push %rax\n{}pop %rbx\nimul %rbx, %rax\n", op1, op2));
                    },
                    &BinaryOperator::Subtraction => {
                        let op1 = self.expression_stack.pop().unwrap();
                        let op2 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}push %rax\n{}pop %rbx\nsubq %rbx, %rax\n", op1, op2));
                    },
                    &BinaryOperator::Division => {
                        let op1 = self.expression_stack.pop().unwrap();
                        let op2 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}push %rax\n{}pop %rbx\nmovq $0,%rdx\nidivq %rbx\n", op1, op2));
                    },
                    &BinaryOperator::Equal => {
                        let op2 = self.expression_stack.pop().unwrap();
                        let op1 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}push %rax\n{}pop %rdx\ncmpq %rax,%rdx\nmovq $0,%rax\nsete %al\n", op1, op2));
                    },
                    &BinaryOperator::NotEqual => {
                        let op2 = self.expression_stack.pop().unwrap();
                        let op1 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}push %rax\n{}pop %rdx\ncmpq %rax,%rdx\nmovq $0,%rax\nsetne %al\n", op1, op2));
                    },
                    &BinaryOperator::GreaterThanOrEqual => {
                        let op2 = self.expression_stack.pop().unwrap();
                        let op1 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}push %rax\n{}pop %rdx\ncmpq %rax,%rdx\nmovq $0,%rax\nsetge %al\n", op1, op2));
                    },
                    &BinaryOperator::GreaterThan => {
                        let op2 = self.expression_stack.pop().unwrap();
                        let op1 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}push %rax\n{}pop %rdx\ncmpq %rax,%rdx\nmovq $0,%rax\nsetg %al\n", op1, op2));
                    },
                    &BinaryOperator::LessThanOrEqual => {
                        let op2 = self.expression_stack.pop().unwrap();
                        let op1 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}push %rax\n{}pop %rdx\ncmpq %rax,%rdx\nmovq $0,%rax\nsetle %al\n", op1, op2));
                    },
                    &BinaryOperator::LessThan => {
                        let op2 = self.expression_stack.pop().unwrap();
                        let op1 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}push %rax\n{}pop %rdx\ncmpq %rax,%rdx\nmovq $0,%rax\nsetl %al\n", op1, op2));
                    },
                    &BinaryOperator::LogicalOr => {
                        let op2 = self.expression_stack.pop().unwrap();
                        let op1 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}push %rax\n{}pop %rdx\norq %rax,%rdx\nmovq $0,%rax\nsetne %al\n", op1, op2));
                    },
                    &BinaryOperator::LogicalAnd => {
                        let op2 = self.expression_stack.pop().unwrap();
                        let op1 = self.expression_stack.pop().unwrap();

                        self.expression_stack.push(format!("{}push %rax\n{}pop %rdx\ncmpq $0,%rdx\nsetne %cl\ncmpq $0,%rax\nsetne %al\n andb %cl, %al\n", op1, op2));
                    },
                    _ => panic!("Not supported")
                };
            },
            Symbol::Statement(ref statement_type) => {
                match statement_type {
                    &StatementType::Return => {
                        self.buf.push_str(&format!("{}ret\n", self.expression_stack[self.expression_stack.len() - 1]));
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