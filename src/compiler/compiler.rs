use super::ast::*;
use super::bytecode::Opcode;
use super::instruction::*;
use super::resolver::{Function, Variable};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct StructConstructor {
    pub name: String,
    pub fields: Vec<(String, Expr)>,
}

#[derive(Debug)]
struct LoopScope {
    continue_target: Option<usize>,
    pending_continue_jumps: Vec<usize>,
}

fn value_type_from_expr(expr: &Expr) -> ValueType {
    match expr {
        Expr::Integer(_) => ValueType::Int32,
        Expr::String(_) => ValueType::String,
        Expr::Float(_) => ValueType::Double,
        Expr::Bool(_) => ValueType::Bool,
        Expr::Variable(_) => ValueType::Var,
        Expr::StructLiteral(_) => ValueType::Var,
        Expr::MemberAccess { .. } => ValueType::Var,
        Expr::Binary {
            operator,
            left,
            right,
        } => match operator {
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
            | BinaryOp::Shr => {
                let left_type = value_type_from_expr(left);
                let right_type = value_type_from_expr(right);
                match (left_type, right_type) {
                    (ValueType::Double, _) | (_, ValueType::Double) => ValueType::Double,
                    (ValueType::Float, _) | (_, ValueType::Float) => ValueType::Float,
                    _ => ValueType::Int32,
                }
            }
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
        Expr::Function { .. } => ValueType::Var,
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
    pub struct_constructors: Vec<StructConstructor>,
    struct_instance_vars: HashSet<String>,
    struct_literal_fields: HashMap<String, Vec<(String, Expr)>>,
    struct_name_prefix: String,
    declared_functions: HashMap<String, String>,
    function_parameters: HashMap<String, String>,
    local_vars: HashSet<String>,
    global_vars: HashSet<String>,
    static_vars: HashSet<String>,
    break_scopes: Vec<Vec<usize>>,
    loop_scopes: Vec<LoopScope>,
    temp_counter: u32,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            struct_constructors: Vec::new(),
            struct_instance_vars: HashSet::new(),
            struct_literal_fields: HashMap::new(),
            struct_name_prefix: String::new(),
            declared_functions: HashMap::new(),
            function_parameters: HashMap::new(),
            local_vars: HashSet::new(),
            global_vars: HashSet::new(),
            static_vars: HashSet::new(),
            break_scopes: Vec::new(),
            loop_scopes: Vec::new(),
            temp_counter: 0,
        }
    }

    pub fn with_struct_name_prefix(prefix: impl Into<String>) -> Self {
        let mut compiler = Self::new();
        compiler.struct_name_prefix = prefix.into();
        compiler
    }

    fn emit_call(&mut self, name: &str, args_len: usize) {
        self.instructions.push(Instruction::Call {
            function: Function {
                name: name.to_string(),
                var_ref: 0,
            },
            args_len,
        });
    }

    fn compile_struct_literal_expression(&mut self, fields: &[(String, Expr)]) {
        // Match GMS2 struct-literal prologue shape: skip static-init body at runtime.
        let skip_init_branch = self.emit_branch(BranchType::Unconditional);

        self.emit_call("@@SetStatic@@", 0);
        for (key, value) in fields {
            self.compile_expression(value);
            self.instructions.push(Instruction::Pop {
                variable: Variable {
                    name: format!("self.{}", key),
                    var_ref: 0xA000_0000,
                },
                dst_type: ValueType::Var,
                src_type: value_type_from_expr(value),
            });
        }
        self.instructions.push(Instruction::Exit);

        let after_init = self.instructions.len();
        self.patch_branch_to(skip_init_branch, after_init);

        let struct_name = if self.struct_name_prefix.is_empty() {
            format!("___struct___{}", self.struct_constructors.len())
        } else {
            format!(
                "___struct___{}_{}",
                self.struct_name_prefix,
                self.struct_constructors.len()
            )
        };
        self.struct_constructors.push(StructConstructor {
            name: struct_name.clone(),
            fields: fields.to_vec(),
        });

        self.instructions.push(Instruction::PushFunc(Function {
            name: struct_name,
            var_ref: 0,
        }));
        self.emit_conv_if_needed(ValueType::Int32, ValueType::Var);

        self.emit_call("@@NullObject@@", 0);
        self.emit_call("method", 2);

        self.instructions.push(Instruction::Dup(ValueType::Var));
        self.instructions.push(Instruction::Pop {
            variable: Variable {
                name: format!(
                    "global.{}",
                    self.struct_constructors
                        .last()
                        .expect("constructor added")
                        .name
                ),
                var_ref: 0xA000_0000,
            },
            dst_type: ValueType::Var,
            src_type: ValueType::Var,
        });

        self.emit_call("@@NewGMLObject@@", 1);

        // Ensure struct fields are materialized on this runtime even when constructor
        // method resolution differs from vanilla runner expectations.
        let temp_name = self.next_temp_name("struct");
        let temp_var = Variable {
            name: temp_name.clone(),
            var_ref: 0xA000_0000,
        };
        self.instructions.push(Instruction::Pop {
            variable: temp_var.clone(),
            dst_type: ValueType::Var,
            src_type: ValueType::Var,
        });

        for (key, value) in fields {
            self.compile_expression(value);
            self.emit_conv_if_needed(value_type_from_expr(value), ValueType::Var);

            self.instructions.push(Instruction::PushS(key.clone()));
            self.emit_conv_if_needed(ValueType::String, ValueType::Var);

            self.instructions.push(Instruction::Push(temp_var.clone()));
            self.emit_conv_if_needed(ValueType::Var, ValueType::Int32);
            self.instructions.push(Instruction::PushI32(100000));
            self.instructions.push(Instruction::BinaryOp {
                lhs_type: ValueType::Int32,
                binary_op: BinaryOp::Add,
                rhs_type: ValueType::Int32,
            });
            self.emit_conv_if_needed(ValueType::Int32, ValueType::Var);

            self.emit_call("variable_struct_set", 3);
            self.emit_popz();
        }

        self.instructions.push(Instruction::Push(temp_var));
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

        if let Some(target_index) = self
            .loop_scopes
            .last()
            .and_then(|scope| scope.continue_target)
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

    fn emit_stacktop_marker(&mut self) {
        self.instructions.push(Instruction::PushI(-1));
    }

    fn collect_array_access_chain(expr: &Expr, indices: &mut Vec<Expr>) -> Option<String> {
        match expr {
            Expr::Call { name, args } if name == "array_get" && args.len() == 2 => {
                if let Some(CallArg::Positional(index)) = args.get(1) {
                    indices.push(index.clone());
                }
                if let Some(CallArg::Positional(base)) = args.get(0) {
                    Self::collect_array_access_chain(base, indices)
                } else {
                    None
                }
            }
            Expr::Variable(name) => Some(name.clone()),
            _ => None,
        }
    }

    fn compile_array_read_call(&mut self, expr: &Expr) -> bool {
        let mut reversed_indices = Vec::new();
        let Some(base_name) = Self::collect_array_access_chain(expr, &mut reversed_indices) else {
            return false;
        };

        if reversed_indices.is_empty() {
            return false;
        }

        let mut indices = reversed_indices;
        indices.reverse();

        if indices.len() == 1 {
            self.emit_stacktop_marker();
            self.compile_expression(&indices[0]);
            self.instructions.push(Instruction::Push(Variable {
                name: base_name,
                var_ref: 0,
            }));
            return true;
        }

        // Emit the BC17+ multidimensional access chain used by the native compiler:
        // push owner marker, first index, base array, then PUSHAC* and final PUSHAF.
        self.instructions.push(Instruction::PushI(-6));
        self.compile_expression(&indices[0]);
        self.instructions.push(Instruction::Push(Variable {
            name: base_name,
            var_ref: 0,
        }));

        for index in indices.iter().take(indices.len() - 1).skip(1) {
            self.compile_expression(index);
            self.instructions.push(Instruction::Break(-4));
        }

        self.compile_expression(&indices[indices.len() - 1]);
        self.instructions.push(Instruction::Break(-2));

        true
    }

    fn emit_popz(&mut self) {
        self.instructions.push(Instruction::PopZ);
    }

    pub fn compile_function_body(&mut self, params: &[FunctionParameter], body: &[Statement]) {
        let previous_function_parameters = std::mem::take(&mut self.function_parameters);
        let previous_local_vars = std::mem::take(&mut self.local_vars);
        let previous_global_vars = std::mem::take(&mut self.global_vars);
        let previous_static_vars = std::mem::take(&mut self.static_vars);

        for (index, param) in params.iter().enumerate() {
            let arg_name = format!("builtin.argument{}", index);
            self.function_parameters
                .insert(param.name.clone(), arg_name);
        }

        for statement in body {
            self.compile_statement(statement);
        }

        self.function_parameters = previous_function_parameters;
        self.local_vars = previous_local_vars;
        self.global_vars = previous_global_vars;
        self.static_vars = previous_static_vars;
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

    /// Resolve a plain variable name to its scoped form based on declarations.
    /// Returns e.g. `"local.x"`, `"global.x"`, or `"static.x"`, falling back
    /// to the name as-is for ordinary instance variables.
    fn resolve_var_name<'a>(&self, name: &'a str) -> std::borrow::Cow<'a, str> {
        if name.contains('.') {
            // Already scoped (e.g. "global.score", "self.x")
            return std::borrow::Cow::Borrowed(name);
        }
        if let Some(mapped) = self.function_parameters.get(name) {
            return std::borrow::Cow::Owned(mapped.clone());
        }
        if self.local_vars.contains(name) {
            return std::borrow::Cow::Owned(format!("local.{}", name));
        }
        if self.global_vars.contains(name) {
            return std::borrow::Cow::Owned(format!("global.{}", name));
        }
        if self.static_vars.contains(name) {
            return std::borrow::Cow::Owned(format!("static.{}", name));
        }
        std::borrow::Cow::Borrowed(name)
    }

    fn resolve_known_struct_expr(&self, expr: &Expr) -> Option<Expr> {
        match expr {
            Expr::StructLiteral(fields) => Some(Expr::StructLiteral(fields.clone())),
            Expr::Variable(name) => self
                .struct_literal_fields
                .get(name)
                .map(|fields| Expr::StructLiteral(fields.clone())),
            Expr::MemberAccess { target, field } => {
                let resolved_target = self.resolve_known_struct_expr(target)?;
                if let Expr::StructLiteral(fields) = resolved_target {
                    fields
                        .iter()
                        .find(|(key, _)| key == field)
                        .map(|(_, value)| value.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn compile_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::FunctionDeclaration { name, .. } => {
                let script_name = format!("gml_Script_{}@{}", name, self.struct_name_prefix);
                self.declared_functions
                    .insert(name.clone(), script_name.clone());

                self.instructions.push(Instruction::PushFunc(Function {
                    name: script_name.clone(),
                    var_ref: 0,
                }));
                self.emit_conv_if_needed(ValueType::Int32, ValueType::Var);

                self.instructions.push(Instruction::PushI(-1));
                self.emit_conv_if_needed(ValueType::Int16, ValueType::Var);

                self.emit_call("method", 2);
                self.instructions.push(Instruction::Dup(ValueType::Var));

                let var = Variable {
                    name: format!("self.{}", name),
                    var_ref: 0xA000_0000,
                };
                self.instructions.push(Instruction::Pop {
                    variable: var,
                    dst_type: ValueType::Var,
                    src_type: ValueType::Var,
                });
            }

            Statement::Assignment { name, value } => {
                let resolved = self.resolve_var_name(name).into_owned();

                if self.try_compile_self_update_assignment(&resolved, value) {
                    return;
                }

                if let Expr::StructLiteral(fields) = value {
                    self.struct_instance_vars.insert(resolved.clone());
                    self.struct_literal_fields
                        .insert(resolved.clone(), fields.clone());
                } else {
                    self.struct_instance_vars.remove(&resolved);
                    self.struct_literal_fields.remove(&resolved);
                }

                self.compile_expression(value);

                let var = Variable {
                    name: resolved,
                    var_ref: 0xA000_0000,
                };

                self.instructions.push(Instruction::Pop {
                    variable: var,
                    dst_type: ValueType::Var,
                    src_type: value_type_from_expr(value),
                });
            }

            Statement::VarDeclaration { declarations } => {
                for (var_name, value) in declarations {
                    self.local_vars.insert(var_name.clone());
                    if let Some(init_expr) = value {
                        self.compile_expression(init_expr);
                        let var = Variable {
                            name: format!("local.{}", var_name),
                            var_ref: 0xA000_0000,
                        };
                        self.instructions.push(Instruction::Pop {
                            variable: var,
                            dst_type: ValueType::Var,
                            src_type: value_type_from_expr(init_expr),
                        });
                    }
                }
            }

            Statement::GlobalVarDeclaration { name } => {
                self.global_vars.insert(name.clone());
            }

            Statement::StaticDeclaration { name, value } => {
                self.static_vars.insert(name.clone());

                // Emit the static-init guard pattern:
                //   Break(-6)          // isstaticok: push bool (already initialized?)
                //   BranchTrue → skip  // skip init if already done
                //   Break(-7)          // setstatic: mark as initialized
                //   <init value>
                //   Pop.static name
                //   [skip:]
                self.instructions.push(Instruction::Break(-6)); // isstaticok
                let branch_skip = self.emit_branch(BranchType::True);

                self.instructions.push(Instruction::Break(-7)); // setstatic

                let init_expr = value.as_ref().cloned().unwrap_or(Expr::Integer(0));
                self.compile_expression(&init_expr);

                let var = Variable {
                    name: format!("static.{}", name),
                    var_ref: 0xA000_0000,
                };
                self.instructions.push(Instruction::Pop {
                    variable: var,
                    dst_type: ValueType::Var,
                    src_type: value_type_from_expr(&init_expr),
                });

                let skip_target = self.instructions.len();
                self.patch_branch_to(branch_skip, skip_target);
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
                    self.instructions
                        .push(Instruction::Ret(value_type_from_expr(expr)));
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
            var_ref: 0xA000_0000,
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

            Expr::Float(value) => {
                self.instructions.push(Instruction::PushD(*value));
            }

            Expr::Bool(value) => {
                self.instructions.push(Instruction::PushBool(*value));
            }

            Expr::Variable(name) => {
                let resolved_name = self.resolve_var_name(name).into_owned();
                let var = Variable {
                    name: resolved_name,
                    var_ref: 0xA000_0000,
                };

                self.instructions.push(Instruction::Push(var));
            }

            Expr::StructLiteral(fields) => {
                self.compile_struct_literal_expression(fields);
            }

            Expr::MemberAccess { target, field } => {
                // `global.field`, `self.field`, `other.field` → scoped variable
                if let Expr::Variable(base) = target.as_ref() {
                    if matches!(base.as_str(), "global" | "self" | "other") {
                        let scoped = Expr::Variable(format!("{}.{}", base, field));
                        self.compile_expression(&scoped);
                        return;
                    }
                }

                if let Some(Expr::StructLiteral(fields)) = self.resolve_known_struct_expr(target) {
                    if let Some((_, value)) = fields.iter().find(|(key, _)| key == field) {
                        let folded_value = value.clone();
                        self.compile_expression(&folded_value);
                        return;
                    }
                }

                self.instructions.push(Instruction::PushS(field.clone()));
                self.emit_conv_if_needed(ValueType::String, ValueType::Var);
                self.compile_expression(target);

                // Butterscotch currently exposes struct handles as small ints; convert them
                // into the runtime target-id domain expected by variable_struct_get.
                if !matches!(target.as_ref(), Expr::Integer(_)) {
                    self.emit_conv_if_needed(value_type_from_expr(target), ValueType::Int32);
                    self.instructions.push(Instruction::PushI32(100000));
                    self.instructions.push(Instruction::BinaryOp {
                        lhs_type: ValueType::Int32,
                        binary_op: BinaryOp::Add,
                        rhs_type: ValueType::Int32,
                    });
                    self.emit_conv_if_needed(ValueType::Int32, ValueType::Var);
                } else {
                    self.emit_conv_if_needed(value_type_from_expr(target), ValueType::Var);
                }
                self.emit_call("variable_struct_get", 2);
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

            Expr::Function { name, .. } => {
                let function_name = name.clone().unwrap_or_else(|| self.next_temp_name("fn"));
                self.instructions.push(Instruction::PushFunc(Function {
                    name: function_name,
                    var_ref: 0,
                }));
                self.emit_conv_if_needed(ValueType::Int32, ValueType::Var);
            }

            Expr::Call { name, args } => {
                if name == "array_get" && self.compile_array_read_call(expr) {
                    return;
                }

                if name == "variable_struct_get" && args.len() == 2 {
                    self.compile_expression(args[1].expr());
                    self.emit_conv_if_needed(value_type_from_expr(args[1].expr()), ValueType::Var);

                    self.compile_expression(args[0].expr());
                    if !matches!(args[0].expr(), Expr::Integer(_)) {
                        self.emit_conv_if_needed(
                            value_type_from_expr(args[0].expr()),
                            ValueType::Int32,
                        );
                        self.instructions.push(Instruction::PushI32(100000));
                        self.instructions.push(Instruction::BinaryOp {
                            lhs_type: ValueType::Int32,
                            binary_op: BinaryOp::Add,
                            rhs_type: ValueType::Int32,
                        });
                        self.emit_conv_if_needed(ValueType::Int32, ValueType::Var);
                    } else {
                        self.emit_conv_if_needed(
                            value_type_from_expr(args[0].expr()),
                            ValueType::Var,
                        );
                    }

                    self.emit_call("variable_struct_get", 2);
                    return;
                }

                if name == "variable_struct_set" && args.len() == 3 {
                    self.compile_expression(args[2].expr());
                    self.emit_conv_if_needed(value_type_from_expr(args[2].expr()), ValueType::Var);

                    self.compile_expression(args[1].expr());
                    self.emit_conv_if_needed(value_type_from_expr(args[1].expr()), ValueType::Var);

                    self.compile_expression(args[0].expr());
                    if !matches!(args[0].expr(), Expr::Integer(_)) {
                        self.emit_conv_if_needed(
                            value_type_from_expr(args[0].expr()),
                            ValueType::Int32,
                        );
                        self.instructions.push(Instruction::PushI32(100000));
                        self.instructions.push(Instruction::BinaryOp {
                            lhs_type: ValueType::Int32,
                            binary_op: BinaryOp::Add,
                            rhs_type: ValueType::Int32,
                        });
                        self.emit_conv_if_needed(ValueType::Int32, ValueType::Var);
                    } else {
                        self.emit_conv_if_needed(
                            value_type_from_expr(args[0].expr()),
                            ValueType::Var,
                        );
                    }

                    self.emit_call("variable_struct_set", 3);
                    return;
                }

                for arg in args.iter().rev() {
                    self.compile_expression(arg.expr());
                    self.emit_conv_if_needed(value_type_from_expr(arg.expr()), ValueType::Var);
                }

                let func_name = self
                    .declared_functions
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| name.clone());
                let func = Function {
                    name: func_name,
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
