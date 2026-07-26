use crate::ast::*;
use crate::instruction::*;
use crate::resolver::Variable;

fn value_type_from_expr(expr: &Expr) -> ValueType {
    match expr {
        Expr::Integer(_) => ValueType::Int,
        Expr::Variable(_) => ValueType::Var,
        Expr::Binary { .. } => ValueType::Var, // Assuming binary expressions result in a variable type
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

                self.instructions.push(Instruction::PushVar(var));
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

                match operator {
                    BinaryOp::Add => {
                        self.instructions.push(Instruction::Add {
                            lhs_type: lhs_type,
                            rhs_type: rhs_type,
                        });
                    }
                }
            }
        }
    }
}
