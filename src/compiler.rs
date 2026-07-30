use crate::ast::*;
use crate::bytecode::Opcode;
use crate::instruction::*;
use crate::resolver::{Function, Variable};

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

                self.instructions.push(Instruction::Pop {
                    variable: var,
                    dst_type: ValueType::Var,
                    src_type: value_type_from_expr(value),
                });
            }

            Statement::Expression(expr) => {
                self.compile_expression(expr);
            }

            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                // Evaluate the condition.
                self.compile_expression(condition);

                let condition_type = value_type_from_expr(condition);
                self.emit_conv_if_needed(condition_type, ValueType::Bool);

                // BF <else/end>
                let branch_false_index = self.instructions.len();
                self.instructions
                    .push(Instruction::Branch(0, BranchType::False));

                // Then block.
                for stmt in then_branch.iter() {
                    self.compile_statement(stmt);
                }

                if let Some(else_branch) = else_branch {
                    // B <end>
                    let branch_end_index = self.instructions.len();
                    self.instructions
                        .push(Instruction::Branch(0, BranchType::Unconditional));

                    // Else starts immediately after the unconditional branch.
                    let else_start_index = self.instructions.len();

                    // Patch BF -> else.
                    if let Some(Instruction::Branch(offset, BranchType::False)) =
                        self.instructions.get_mut(branch_false_index)
                    {
                        *offset = (else_start_index - branch_false_index + 1) as u16;
                    }

                    // Else block.
                    for stmt in else_branch.iter() {
                        self.compile_statement(stmt);
                    }

                    // End of if/else.
                    let end_index = self.instructions.len();

                    // Patch B -> end.
                    if let Some(Instruction::Branch(offset, BranchType::Unconditional)) =
                        self.instructions.get_mut(branch_end_index)
                    {
                        *offset = (end_index - branch_end_index + 1) as u16;
                    }
                } else {
                    // No else: BF jumps directly to the end.
                    let end_index = self.instructions.len();

                    if let Some(Instruction::Branch(offset, BranchType::False)) =
                        self.instructions.get_mut(branch_false_index)
                    {
                        *offset = (end_index - branch_false_index + 1) as u16;
                    }
                }
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
                    self.emit_conv_if_needed(value_type_from_expr(arg), ValueType::Var);
                }

                let func = Function {
                    name: name.clone(),
                    var_ref: 0,
                };

                self.instructions.push(Instruction::Call {
                    function: func,
                    args_len: args.len(),
                });
            }
        }
    }
}
