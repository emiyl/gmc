use crate::ast::*;
use crate::bytecode::Opcode;
use crate::instruction::*;
use crate::resolver::Variable;

fn value_type_from_expr(expr: &Expr) -> ValueType {
    match expr {
        Expr::Integer(_) => ValueType::Int32,
        Expr::Variable(_) => ValueType::Var,
        Expr::Binary { operator, .. } => match operator {
            BinaryOp::Mul
            | BinaryOp::Div
            | BinaryOp::Rem
            | BinaryOp::Mod
            | BinaryOp::Add
            | BinaryOp::Sub
            | BinaryOp::And
            | BinaryOp::Or
            | BinaryOp::Xor
            | BinaryOp::Shl
            | BinaryOp::Shr => ValueType::Int32,
            BinaryOp::Lt
            | BinaryOp::Lte
            | BinaryOp::Eq
            | BinaryOp::Neq
            | BinaryOp::Gte
            | BinaryOp::Gt => ValueType::Bool,
        },
        Expr::Unary { operator, operand } => match operator {
            UnaryOp::Neg => value_type_from_expr(operand),
            UnaryOp::Not => ValueType::Bool,
        },
        Expr::Call { .. } => ValueType::Var, // Assuming function calls result in a variable type
    }
}

pub struct Compiler {
    pub instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn compile_program(&mut self, program: &[Statement]) {
        for statement in program {
            self.compile_statement(statement);
        }
    }

    pub fn emit_conv_if_needed(&mut self, from: ValueType, to: ValueType) {
        if from != to {
            self.instructions.push(Instruction::Conv { from, to });
        }
    }

    fn compile_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Assignment { name, value } => {
                self.compile_expression(value);
                let var = Variable {
                    name: name.clone(),
                    var_ref: 0,
                };

                let dst_type = ValueType::Var;
                let src_type = value_type_from_expr(value);

                self.instructions.push(Instruction::Pop {
                    variable: var,
                    dst_type,
                    src_type,
                });
            }
            Statement::Expression(expr) => {
                self.compile_expression(expr);
            }
        }
    }

    fn compile_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Integer(value) => {
                self.instructions.push(Instruction::PushI(*value));
            }

            Expr::Variable(name) => {
                let var = Variable {
                    name: name.clone(),
                    var_ref: 0,
                };

                self.instructions.push(Instruction::Push(var));
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                self.compile_expression(left);
                self.compile_expression(right);

                let lhs_type = value_type_from_expr(left);
                let rhs_type = value_type_from_expr(right);

                self.instructions.push(Instruction::BinaryOp {
                    lhs_type,
                    binary_op: operator.clone(),
                    rhs_type,
                });
            }

            Expr::Unary { operator, operand } => {
                self.compile_expression(operand);

                match operator {
                    UnaryOp::Neg => {
                        self.instructions.push(Instruction::UnaryOp {
                            opcode: Opcode::Neg,
                            operand_type: value_type_from_expr(operand),
                        });
                    }

                    UnaryOp::Not => {
                        let operand_type = value_type_from_expr(operand);

                        self.emit_conv_if_needed(operand_type, ValueType::Bool);

                        self.instructions.push(Instruction::UnaryOp {
                            opcode: Opcode::Not,
                            operand_type: ValueType::Bool,
                        });
                    }
                }
            }

            Expr::Call { name, args } => {
                for arg in args {
                    self.compile_expression(arg);
                }

                let var = Variable {
                    name: name.clone(),
                    var_ref: 0,
                };

                self.instructions.push(Instruction::Call { function: var });
            }
        }
    }
}
