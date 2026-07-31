pub fn format_json(json: &str) -> String {
    let json =
        serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(json).unwrap())
            .unwrap();
    let json = remove_json_colon_spaces(&json);
    let json = add_json_trailing_commas(&json);
    json
}

fn remove_json_colon_spaces(json: &str) -> String {
    let mut out = String::with_capacity(json.len());
    let mut chars = json.chars().peekable();
    let mut in_string = false;
    let mut escaped = false;

    while let Some(c) = chars.next() {
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            out.push(c);
        } else {
            if c == '"' {
                in_string = true;
                out.push(c);
            } else if c == ':' && chars.peek() == Some(&' ') {
                out.push(':');
                chars.next(); // Skip the space after the colon.
            } else {
                out.push(c);
            }
        }
    }

    out
}

pub fn add_json_trailing_commas(json: &str) -> String {
    let mut out = String::with_capacity(json.len() + json.len() / 16);

    let mut in_string = false;
    let mut escaped = false;

    for c in json.chars() {
        if in_string {
            out.push(c);

            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }

            continue;
        }

        match c {
            '"' => {
                in_string = true;
                out.push(c);
            }

            '}' | ']' => {
                // Insert a comma before any trailing whitespace.
                let end = out.trim_end_matches(char::is_whitespace).len();

                if end > 0 {
                    let prev = out.as_bytes()[end - 1];

                    if prev != b'{' && prev != b'[' && prev != b',' {
                        out.insert(end, ',');
                    }
                }

                out.push(c);
            }

            _ => out.push(c),
        }
    }

    out
}
