use serde_json::Value;

pub fn format_gamemaker_json(value: &Value) -> String {
    let mut out = String::new();
    format_value(value, &mut out, 0, false);
    out.push('\n');
    out
}

fn format_value(value: &Value, out: &mut String, indent: usize, inline: bool) {
    match value {
        Value::Null => out.push_str("null"),

        Value::Bool(v) => out.push_str(if *v { "true" } else { "false" }),

        Value::Number(v) => out.push_str(&v.to_string()),

        Value::String(v) => {
            out.push('"');
            out.push_str(&v.replace('"', "\\\""));
            out.push('"');
        }

        Value::Array(arr) => {
            if arr.is_empty() {
                out.push_str("[]");
                return;
            }

            out.push('[');

            if inline {
                for (i, item) in arr.iter().enumerate() {
                    if i != 0 {
                        out.push(',');
                    }

                    format_value(item, out, indent + 1, true);
                    out.push(',');
                }

                out.push(']');
                return;
            }

            out.push('\n');

            for (_i, item) in arr.iter().enumerate() {
                write_indent(out, indent + 1);

                // Objects inside arrays are GameMaker style inline
                let compact = matches!(item, Value::Object(_));

                format_value(item, out, indent + 1, compact);

                out.push(',');

                out.push('\n');
            }

            write_indent(out, indent);
            out.push(']');
        }

        Value::Object(map) => {
            if inline {
                out.push('{');

                for (i, (key, value)) in map.iter().enumerate() {
                    if i != 0 {
                        out.push(',');
                    }

                    write_string(out, key);
                    out.push(':');

                    format_value(value, out, indent + 1, false);
                }

                if !map.is_empty() {
                    out.push(',');
                }

                out.push('}');
                return;
            }

            if map.is_empty() {
                out.push_str("{}");
                return;
            }

            out.push('{');
            out.push('\n');

            for (key, value) in map.iter() {
                write_indent(out, indent + 1);

                write_string(out, key);
                out.push(':');

                format_value(value, out, indent + 1, false);

                out.push(',');
                out.push('\n');
            }

            write_indent(out, indent);
            out.push('}');
        }
    }
}

fn write_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push_str("  ");
    }
}

fn write_string(out: &mut String, s: &str) {
    out.push('"');
    out.push_str(&s.replace('"', "\\\""));
    out.push('"');
}
