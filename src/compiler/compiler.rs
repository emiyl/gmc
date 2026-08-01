use super::ast::*;
use super::bytecode::Opcode;
use super::instruction::*;
use super::resolver::{Function, Variable};

#[derive(Debug)]
struct LoopScope {
    continue_target: Option<usize>,
    pending_continue_jumps: Vec<usize>,
}

fn value_type_from_expr(expr: &Expr) -> ValueType {
    match expr {
        Expr::Integer(_) => ValueType::Int32,
        Expr::String(_) => ValueType::String,
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
        Expr::Call { .. } => ValueType::Var,
        Expr::Ternary {
            then_expr,
            else_expr,
            ..
        } => {
            let then_type = value_type_from_expr(then_expr);
            let else_type = value_type_from_expr(else_expr);
            if then_type == else_type {
                then_type
            } else {
                ValueType::Var
            }
        }
    }
}

pub struct Compiler {
    pub instructions: Vec<Instruction>,
    break_scopes: Vec<Vec<usize>>,
    loop_scopes: Vec<LoopScope>,
    temp_counter: u32,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            break_scopes: Vec::new(),
            loop_scopes: Vec::new(),
            temp_counter: 0,
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

    fn emit_branch(&mut self, branch_type: BranchType) -> usize {
        let index = self.instructions.len();
        self.instructions.push(Instruction::Branch(0, branch_type));
        index
    }

    fn patch_branch_to(&mut self, branch_index: usize, target_index: usize) {
        if let Some(Instruction::Branch(offset, _)) = self.instructions.get_mut(branch_index) {
            *offset = target_index as i32 - branch_index as i32;
        }
    }

    fn emit_jump_to(&mut self, target_index: usize) {
        let branch_index = self.emit_branch(BranchType::Unconditional);
        self.patch_branch_to(branch_index, target_index);
    }

    fn push_break_scope(&mut self) {
        self.break_scopes.push(Vec::new());
    }

    fn pop_break_scope_and_patch(&mut self, target_index: usize) {
        let break_jumps = self
            .break_scopes
            .pop()
            .expect("break scope stack underflow");

        for branch_index in break_jumps {
            self.patch_branch_to(branch_index, target_index);
        }
    }

    fn push_loop_scope(&mut self, continue_target: Option<usize>) {
        self.loop_scopes.push(LoopScope {
            continue_target,
            pending_continue_jumps: Vec::new(),
        });
        self.push_break_scope();
    }

    fn set_current_loop_continue_target(&mut self, target_index: usize) {
        if let Some(scope) = self.loop_scopes.last_mut() {
            scope.continue_target = Some(target_index);
            let pending = std::mem::take(&mut scope.pending_continue_jumps);
            for branch_index in pending {
                self.patch_branch_to(branch_index, target_index);
            }
        }
    }

    fn pop_loop_scope_and_patch_breaks(&mut self, break_target_index: usize) {
        let scope = self.loop_scopes.pop().expect("loop scope stack underflow");
        assert!(
            scope.pending_continue_jumps.is_empty(),
            "continue target left unresolved when exiting loop"
        );
        self.pop_break_scope_and_patch(break_target_index);
    }

    fn emit_break(&mut self) {
        let branch_index = self.emit_branch(BranchType::Unconditional);
        if let Some(scope) = self.break_scopes.last_mut() {
            scope.push(branch_index);
        } else {
            panic!("'break' used outside of loop/switch");
        }
    }

    fn emit_continue(&mut self) {
        if self.loop_scopes.is_empty() {
            panic!("'continue' used outside of loop");
        }

        if let Some(target_index) = self.loop_scopes.last().and_then(|scope| scope.continue_target)
        {
            self.emit_jump_to(target_index);
            return;
        }

        let branch_index = self.emit_branch(BranchType::Unconditional);
        self.loop_scopes
            .last_mut()
            .expect("loop scope stack underflow")
            .pending_continue_jumps
            .push(branch_index);
    }

    fn next_temp_name(&mut self, prefix: &str) -> String {
        let id = self.temp_counter;
        self.temp_counter += 1;
        format!("__gmlc_{}_{}", prefix, id)
    }

    fn emit_popz(&mut self) {
        self.instructions.push(Instruction::PopZ);
    }

    fn compile_expression_statement_with_discard(&mut self, expr: &Expr) {
        self.compile_expression(expr);
        self.emit_popz();
    }

    fn compile_for_clause_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Expression(expr) => self.compile_expression_statement_with_discard(expr),
            _ => self.compile_statement(statement),
        }
    }

    fn compile_assignment_to_variable(&mut self, name: &str, value: Expr) {
        self.compile_statement(&Statement::Assignment {
            name: name.to_string(),
            value,
        });
    }

    fn compile_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Assignment { name, value } => {
                if self.try_compile_self_update_assignment(name, value) {
                    return;
                }

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
                self.compile_expression_statement_with_discard(expr);
            }

            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.compile_expression(condition);
                let condition_type = value_type_from_expr(condition);
                self.emit_conv_if_needed(condition_type, ValueType::Bool);

                let branch_false_index = self.emit_branch(BranchType::False);

                for stmt in then_branch.iter() {
                    self.compile_statement(stmt);
                }

                if let Some(else_branch) = else_branch {
                    let branch_end_index = self.emit_branch(BranchType::Unconditional);
                    let else_start_index = self.instructions.len();
                    self.patch_branch_to(branch_false_index, else_start_index);

                    for stmt in else_branch.iter() {
                        self.compile_statement(stmt);
                    }

                    let end_index = self.instructions.len();
                    self.patch_branch_to(branch_end_index, end_index);
                } else {
                    let end_index = self.instructions.len();
                    self.patch_branch_to(branch_false_index, end_index);
                }
            }

            Statement::While { condition, body } => {
                let condition_start = self.instructions.len();
                self.compile_expression(condition);
                self.emit_conv_if_needed(value_type_from_expr(condition), ValueType::Bool);

                let branch_false_index = self.emit_branch(BranchType::False);

                self.push_loop_scope(Some(condition_start));
                for stmt in body.iter() {
                    self.compile_statement(stmt);
                }

                self.emit_jump_to(condition_start);

                let loop_end = self.instructions.len();
                self.patch_branch_to(branch_false_index, loop_end);
                self.pop_loop_scope_and_patch_breaks(loop_end);
            }

            Statement::Repeat { count, body } => {
                let temp_name = self.next_temp_name("repeat");
                self.compile_assignment_to_variable(&temp_name, count.clone());

                let condition_start = self.instructions.len();
                self.compile_expression(&Expr::Variable(temp_name.clone()));
                self.compile_expression(&Expr::Integer(0));
                self.instructions.push(Instruction::BinaryOp {
                    lhs_type: ValueType::Var,
                    binary_op: BinaryOp::Gt,
                    rhs_type: ValueType::Int32,
                });

                let branch_false_index = self.emit_branch(BranchType::False);

                self.push_loop_scope(None);
                for stmt in body.iter() {
                    self.compile_statement(stmt);
                }

                let decrement_start = self.instructions.len();
                self.set_current_loop_continue_target(decrement_start);
                self.compile_assignment_to_variable(
                    &temp_name,
                    Expr::Binary {
                        left: Box::new(Expr::Variable(temp_name.clone())),
                        operator: BinaryOp::Sub,
                        right: Box::new(Expr::Integer(1)),
                    },
                );

                self.emit_jump_to(condition_start);

                let loop_end = self.instructions.len();
                self.patch_branch_to(branch_false_index, loop_end);
                self.pop_loop_scope_and_patch_breaks(loop_end);
            }

            Statement::For {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init_statement) = init {
                    self.compile_for_clause_statement(init_statement);
                }

                let condition_start = self.instructions.len();
                let branch_false_index = if let Some(condition) = condition {
                    self.compile_expression(condition);
                    self.emit_conv_if_needed(value_type_from_expr(condition), ValueType::Bool);
                    Some(self.emit_branch(BranchType::False))
                } else {
                    None
                };

                self.push_loop_scope(None);
                for stmt in body.iter() {
                    self.compile_statement(stmt);
                }

                let update_start = self.instructions.len();
                self.set_current_loop_continue_target(update_start);
                if let Some(update_statement) = update {
                    self.compile_for_clause_statement(update_statement);
                }

                self.emit_jump_to(condition_start);

                let loop_end = self.instructions.len();
                if let Some(branch_false_index) = branch_false_index {
                    self.patch_branch_to(branch_false_index, loop_end);
                }
                self.pop_loop_scope_and_patch_breaks(loop_end);
            }

            Statement::DoUntil { body, condition } => {
                let loop_start = self.instructions.len();
                self.push_loop_scope(None);

                for stmt in body.iter() {
                    self.compile_statement(stmt);
                }

                let condition_start = self.instructions.len();
                self.set_current_loop_continue_target(condition_start);

                self.compile_expression(condition);
                self.emit_conv_if_needed(value_type_from_expr(condition), ValueType::Bool);

                let loop_back_branch = self.emit_branch(BranchType::False);
                self.patch_branch_to(loop_back_branch, loop_start);

                let loop_end = self.instructions.len();
                self.pop_loop_scope_and_patch_breaks(loop_end);
            }

            Statement::Switch {
                value,
                cases,
                default,
            } => {
                let temp_name = self.next_temp_name("switch");
                self.compile_assignment_to_variable(&temp_name, value.clone());

                self.push_break_scope();

                let mut case_branch_indices = Vec::with_capacity(cases.len());
                for (case_value, _) in cases {
                    self.compile_expression(&Expr::Variable(temp_name.clone()));
                    self.compile_expression(case_value);
                    self.instructions.push(Instruction::BinaryOp {
                        lhs_type: ValueType::Var,
                        binary_op: BinaryOp::Eq,
                        rhs_type: value_type_from_expr(case_value),
                    });
                    case_branch_indices.push(self.emit_branch(BranchType::True));
                }

                let jump_to_default_or_end = self.emit_branch(BranchType::Unconditional);

                let mut case_body_starts = Vec::with_capacity(cases.len());
                for (_, body) in cases {
                    case_body_starts.push(self.instructions.len());
                    for statement in body {
                        self.compile_statement(statement);
                    }
                }

                let default_start = self.instructions.len();
                if let Some(default_body) = default {
                    for statement in default_body {
                        self.compile_statement(statement);
                    }
                }

                let switch_end = self.instructions.len();

                for (branch_index, case_start) in case_branch_indices
                    .into_iter()
                    .zip(case_body_starts.into_iter())
                {
                    self.patch_branch_to(branch_index, case_start);
                }

                self.patch_branch_to(jump_to_default_or_end, default_start);
                self.pop_break_scope_and_patch(switch_end);
            }

            Statement::Break => {
                self.emit_break();
            }

            Statement::Continue => {
                self.emit_continue();
            }

            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    self.compile_expression(expr);
                    self.instructions.push(Instruction::Ret(value_type_from_expr(expr)));
                } else {
                    self.instructions.push(Instruction::Exit);
                }
            }
        }
    }

    fn try_compile_self_update_assignment(&mut self, name: &str, value: &Expr) -> bool {
        let (left, operator, right) = match value {
            Expr::Binary {
                left,
                operator,
                right,
            } => (left, operator, right),
            _ => return false,
        };

        let left_name = match &**left {
            Expr::Variable(left_name) => left_name,
            _ => return false,
        };

        if left_name != name {
            return false;
        }

        let is_supported = matches!(operator, BinaryOp::Add | BinaryOp::Sub)
            && matches!(&**right, Expr::Integer(1));
        if !is_supported {
            return false;
        }

        let var = Variable {
            name: name.to_string(),
            var_ref: 0,
        };

        self.instructions.push(Instruction::Push(var.clone()));
        self.instructions.push(Instruction::PushE(1));
        self.instructions.push(Instruction::BinaryOp {
            lhs_type: ValueType::Var,
            binary_op: operator.clone(),
            rhs_type: ValueType::Int32,
        });
        self.instructions.push(Instruction::Pop {
            variable: var,
            dst_type: ValueType::Var,
            src_type: ValueType::Var,
        });

        true
    }

    fn compile_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Integer(value) => {
                self.instructions.push(Instruction::PushI(*value));
            }

            Expr::String(value) => {
                self.instructions.push(Instruction::PushS(value.clone()));
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

            Expr::Ternary {
                condition,
                then_expr,
                else_expr,
            } => {
                let result_type = value_type_from_expr(expr);

                self.compile_expression(condition);
                self.emit_conv_if_needed(value_type_from_expr(condition), ValueType::Bool);

                let branch_false_index = self.emit_branch(BranchType::False);

                self.compile_expression(then_expr);
                self.emit_conv_if_needed(value_type_from_expr(then_expr), result_type);
                let branch_end_index = self.emit_branch(BranchType::Unconditional);

                let else_start = self.instructions.len();
                self.patch_branch_to(branch_false_index, else_start);

                self.compile_expression(else_expr);
                self.emit_conv_if_needed(value_type_from_expr(else_expr), result_type);

                let end = self.instructions.len();
                self.patch_branch_to(branch_end_index, end);
            }
        }
    }
}
