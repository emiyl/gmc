use std::collections::HashMap;

use super::Program;
use super::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub var_ref: u32,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub var_ref: u32,
}

pub struct Resolver {
    pub variables: HashMap<String, Variable>,
    pub functions: HashMap<String, Function>,
    var_next_index: u32,
    func_next_index: u32,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            var_next_index: 0,
            func_next_index: 0,
        }
    }

    fn get_variable(&mut self, name: &str) -> Variable {
        if let Some(variable) = self.variables.get(name) {
            return variable.clone();
        }

        let variable = Variable {
            name: name.to_string(),
            var_ref: 0xA0000000 | self.var_next_index,
        };

        self.var_next_index += 1;

        self.variables.insert(name.to_string(), variable.clone());

        variable
    }

    fn get_function(&mut self, name: &str) -> Function {
        if let Some(function) = self.functions.get(name) {
            return function.clone();
        }

        let function = Function {
            name: name.to_string(),
            var_ref: self.func_next_index,
        };

        self.func_next_index += 1;

        self.functions.insert(name.to_string(), function.clone());

        function
    }

    pub fn resolve(&mut self, instructions: Vec<Instruction>) -> Program {
        let instructions = instructions
            .into_iter()
            .map(|instruction| match instruction {
                Instruction::Push(var) => {
                    let variable = self.get_variable(&var.name);
                    let var_kind = var.var_ref & 0xF8000000;
                    let var_ref = var_kind | (variable.var_ref & 0x07FFFFFF);

                    let variable = Variable {
                        name: variable.name,
                        var_ref,
                    };

                    Instruction::Push(variable)
                }

                Instruction::Pop {
                    variable: var,
                    dst_type,
                    src_type,
                } => {
                    let variable = self.get_variable(&var.name);
                    let var_kind = var.var_ref & 0xF8000000;
                    let var_ref = var_kind | (variable.var_ref & 0x07FFFFFF);

                    let variable = Variable {
                        name: variable.name,
                        var_ref,
                    };

                    Instruction::Pop {
                        variable,
                        dst_type,
                        src_type,
                    }
                }

                Instruction::Call { function, args_len } => {
                    let function = self.get_function(&function.name);

                    Instruction::Call { function, args_len }
                }

                Instruction::PushFunc(function) => {
                    let function = self.get_function(&function.name);
                    Instruction::PushFunc(function)
                }

                other => other,
            })
            .collect::<Vec<Instruction>>();
        let variables = self.variables.values().cloned().collect::<Vec<Variable>>();
        let functions = instructions
            .iter()
            .filter_map(|instruction| match instruction {
                Instruction::Call { function, .. } => Some(Function {
                    name: function.name.clone(),
                    var_ref: function.var_ref,
                }),
                _ => None,
            })
            .collect::<Vec<Function>>();

        Program::new(instructions, variables, functions)
    }
}
