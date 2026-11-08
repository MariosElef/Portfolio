//! Colouring short Rust and Python snippets: enough of each language to look right in
//! the hero, not a full parser. Strings and comments stay on one line.

/// What a piece of code is, for its colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Plain,
    Keyword,
    Function,
    Type,
    Name,
    Text,
    Number,
    Comment,
    Operator,
}

const RUST: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "false",
    "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "use", "where",
    "while",
];
const PYTHON: &[&str] = &[
    "and", "as", "async", "await", "class", "def", "elif", "else", "except", "False", "finally",
    "for", "from", "if", "import", "in", "is", "lambda", "None", "not", "or", "pass", "raise",
    "return", "True", "try", "while", "with", "yield",
];
const TYPES: &[&str] = &[
    "str", "bool", "int", "float", "i32", "i64", "u8", "u32", "u64", "usize",
];

/// Each line of `code`, as runs of text and what they are.
pub fn highlight(code: &str, language: &str) -> Vec<Vec<(String, Kind)>> {
    let python = language == "python";
    let keywords = if python { PYTHON } else { RUST };
    code.lines()
        .map(|line| merge(tokens(line, python, keywords)))
        .collect()
}

fn tokens(line: &str, python: bool, keywords: &[&str]) -> Vec<(String, Kind)> {
    let chars: Vec<char> = line.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    let rest = |i: usize| chars[i..].iter().collect::<String>();
    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        let comment = if python {
            c == '#'
        } else {
            c == '/' && next == Some('/')
        };
        if comment {
            out.push((rest(i), Kind::Comment));
            break;
        }
        // A Rust attribute, as `#[derive(Debug)]`: quiet, like a comment.
        if !python && c == '#' && next == Some('[') {
            out.push((rest(i), Kind::Comment));
            break;
        }
        let string_start = c == '"' || (python && c == 'f' && next == Some('"'));
        if string_start {
            let start = i;
            i += if c == 'f' { 2 } else { 1 };
            while i < chars.len() && chars[i] != '"' {
                i += if chars[i] == '\\' { 2 } else { 1 };
            }
            i = (i + 1).min(chars.len());
            out.push((chars[start..i].iter().collect(), Kind::Text));
            continue;
        }
        if c.is_ascii_digit() {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '.') {
                i += 1;
            }
            out.push((chars[start..i].iter().collect(), Kind::Number));
            continue;
        }
        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let kind = if keywords.contains(&word.as_str()) {
                Kind::Keyword
            } else if chars.get(i) == Some(&'!') && !python {
                i += 1;
                out.push((format!("{word}!"), Kind::Function));
                continue;
            } else if chars.get(i) == Some(&'(') {
                Kind::Function
            } else if word.starts_with(char::is_uppercase) || TYPES.contains(&word.as_str()) {
                Kind::Type
            } else {
                Kind::Name
            };
            out.push((word, kind));
            continue;
        }
        let kind = if "=<>!&|+-*/%".contains(c) {
            Kind::Operator
        } else {
            Kind::Plain
        };
        out.push((c.to_string(), kind));
        i += 1;
    }
    out
}

/// Neighbouring runs of the same kind, joined.
fn merge(tokens: Vec<(String, Kind)>) -> Vec<(String, Kind)> {
    let mut out: Vec<(String, Kind)> = Vec::new();
    for (text, kind) in tokens {
        match out.last_mut() {
            Some((last, last_kind)) if *last_kind == kind => last.push_str(&text),
            _ => out.push((text, kind)),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{Kind, highlight};

    #[test]
    fn colours_rust() {
        let lines = highlight(r#"    println!("Hi {name}");"#, "rust");
        let kinds: Vec<_> = lines[0]
            .iter()
            .map(|(text, kind)| (text.as_str(), *kind))
            .collect();
        assert_eq!(
            kinds,
            [
                ("    ", Kind::Plain),
                ("println!", Kind::Function),
                ("(", Kind::Plain),
                (r#""Hi {name}""#, Kind::Text),
                (");", Kind::Plain),
            ]
        );
    }

    #[test]
    fn colours_python() {
        let lines = highlight(r#"def hi(you: str) -> str:"#, "python");
        let kinds: Vec<_> = lines[0].iter().map(|(_, kind)| *kind).collect();
        assert_eq!(
            kinds,
            [
                Kind::Keyword,
                Kind::Plain,
                Kind::Function,
                Kind::Plain,
                Kind::Name,
                Kind::Plain,
                Kind::Type,
                Kind::Plain,
                Kind::Operator,
                Kind::Plain,
                Kind::Type,
                Kind::Plain,
            ]
        );
    }
}
