use crate::resolver::Variable;

pub fn encode_variable(variable: &Variable) -> u16 {
    let name = &variable.name;
    let reference = if name.starts_with("global.") {
        0xFFFB
    } else if name.starts_with("self.") {
        0xFFFF
    } else {
        0xFFFA
    };
    reference
}
