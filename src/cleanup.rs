
#[derive(Clone, Copy, Debug)]
enum State {
    String,
    Slash,
    Comment,
    MultilineComment,
    Asterisk,
    Whitespace,
    Newline,
}

pub fn remove_comments(mut program: String) -> String {
    let mut state: State = State::String;
    let mut current_idx: usize = 0;
    let mut current: char;
    while current_idx < program.chars().count() {
        current = program.as_str().as_bytes()[current_idx] as char;
        match state {
            // Alphanumeric char
            State::String | State::Extra => {
                state = match current {
                    ' ' | '\t' => State::Whitespace,
                    '\n' => State::Newline,
                    '/' => State::Slash,
                    _ => State::String,
                };
                current_idx += 1;
            }
            // Singular slash, might be a comment or a division
            State::Slash => {
                state = match current {
                    '/' => {
                        current_idx -= 1;
                        program.remove(current_idx);
                        program.remove(current_idx);
                        State::Comment
                    },
                    '*' => {
                        current_idx -= 1;
                        program.remove(current_idx);
                        program.remove(current_idx);
                        State::MultilineComment
                    },
                    ' ' | '\t' => State::Whitespace,
                    '\n' => State::Newline,
                    _ => State::String,
                };
                // current_idx += 1;
            }
            State::Comment => {
                state = match current {
                    '\n' => {
                        program.remove(current_idx);
                        State::Newline
                    },
                    _ => {
                        program.remove(current_idx);
                        State::Comment
                    }
                };
            }
            State::MultilineComment => {
                state = match current {
                    '*' => {
                        program.remove(current_idx);
                        State::Asterisk
                    },
                    _ => {
                        program.remove(current_idx);
                        State::MultilineComment
                    }
                };
            }
            State::Asterisk => {
                state = match current {
                    '/' => {
                        current_idx -= 1;
                        program.remove(current_idx);
                        program.remove(current_idx);
                        State::String
                    }
                    _ => {
                        program.remove(current_idx);
                        // current_idx += 1;
                        State::MultilineComment
                    }
                };
                // current_idx += 1;

            }
            State::Whitespace => {
                state = match current {
                    ' ' | '\t' => {
                        program.remove(current_idx);
                        State::Whitespace
                    }
                    '\n' => {
                        current_idx -= 1;
                        program.remove(current_idx);
                        State::Newline
                    }
                    _ => State::String
                };
            }
            State::Newline => {
                state = match current {
                    '\n' => {
                        program.remove(current_idx);
                        State::Newline
                    }
                    ' ' | '\t' => {
                        program.remove(current_idx);
                        State::Newline
                    }
                    _ => State::String
                };
            }
        }
    }
    program = remove_empty_lines(program);
    program
    // add_line_numbers(program)

}
fn remove_empty_lines(program: String) -> String {
    program.trim().lines().filter(|x| x != &"\n").fold(String::new(), |s, l| s + l + "\n")
}
pub fn add_line_numbers(program: String) -> String {
    let mut output: String = String::new();
    for (idx, sub) in program.lines().enumerate() {
        output.push_str(&format!("{:<3}{}\n", idx, sub))
    }
    output
}
