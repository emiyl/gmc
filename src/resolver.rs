use std::collections::HashMap;

use crate::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub var_ref: u32,
}

pub struct Resolver {
    variables: HashMap<String, Variable>,
    next_index: u32,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            next_index: 0,
        }
    }

    fn get_variable(&mut self, name: &str) -> Variable {
        if let Some(variable) = self.variables.get(name) {
            return variable.clone();
        }

        let variable = Variable {
            name: name.to_string(),
            var_ref: 0xA0000000 | self.next_index,
        };

        self.next_index += 1;

        self.variables.insert(name.to_string(), variable.clone());

        variable
    }

    pub fn resolve(&mut self, instructions: Vec<Instruction>) -> (Vec<Instruction>, Vec<Variable>) {
        let instructions = instructions
            .into_iter()
            .map(|instruction| match instruction {
                Instruction::Push(var) => {
                    let variable = self.get_variable(&var.name);

                    Instruction::Push(variable)
                }

                Instruction::Pop {
                    variable: var,
                    dst_type,
                    src_type,
                } => {
                    let variable = self.get_variable(&var.name);

                    Instruction::Pop {
                        variable,
                        dst_type,
                        src_type,
                    }
                }

                other => other,
            })
            .collect::<Vec<Instruction>>();
        let variables = self.variables.values().cloned().collect::<Vec<Variable>>();
        (instructions, variables)
    }
}
