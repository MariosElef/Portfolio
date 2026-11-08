/// The paragraphs of `text`, separated by blank lines, trimmed and without empties.
pub fn paragraphs(text: &str) -> Vec<String> {
    text.split("\n\n")
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .map(str::to_owned)
        .collect()
}

/// A run of text, a run that stands out, or a tool mentioned in it.
#[derive(Debug, PartialEq, Eq)]
pub enum Segment {
    Text(String),
    /// Written `**like this**`.
    Strong(String),
    /// Written `[[icon:Label]]`, or `[[icon]]` to show the icon name itself. `before`
    /// and `after` are the characters touching it, such as `(` and `),`, kept with it
    /// so a line never breaks between a bracket and its mention.
    Tech {
        icon: String,
        label: String,
        before: String,
        after: String,
    },
}

/// Splits `text` into plain runs, `**strong**` runs and `[[icon:Label]]` mentions.
/// Anything that is not a complete mention, or a `**` without its pair, stays as text.
pub fn segments(text: &str) -> Vec<Segment> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("[[") {
        let Some(len) = rest[start + 2..].find("]]") else {
            break;
        };
        let inner = &rest[start + 2..start + 2 + len];
        if !rest[..start].is_empty() {
            out.push(Segment::Text(rest[..start].to_owned()));
        }
        let (icon, label) = inner.split_once(':').unwrap_or((inner, inner));
        out.push(Segment::Tech {
            icon: icon.trim().to_lowercase(),
            label: label.trim().to_owned(),
            before: String::new(),
            after: String::new(),
        });
        rest = &rest[start + 2 + len + 2..];
    }
    if !rest.is_empty() {
        out.push(Segment::Text(rest.to_owned()));
    }
    strong(glue(out))
}

/// Splits each plain run on its `**` pairs. A run with an unpaired `**` stays as it is.
fn strong(segments: Vec<Segment>) -> Vec<Segment> {
    segments
        .into_iter()
        .flat_map(|segment| match segment {
            Segment::Text(text) if text.matches("**").count() % 2 == 0 => text
                .split("**")
                .enumerate()
                .filter(|(_, part)| !part.is_empty())
                .map(|(i, part)| {
                    if i % 2 == 1 {
                        Segment::Strong(part.to_owned())
                    } else {
                        Segment::Text(part.to_owned())
                    }
                })
                .collect(),
            other => vec![other],
        })
        .collect()
}

/// Moves the characters touching each mention (no space between) from the text around
/// it into the mention.
fn glue(segments: Vec<Segment>) -> Vec<Segment> {
    let mut segments: Vec<Option<Segment>> = segments.into_iter().map(Some).collect();
    for i in 0..segments.len() {
        if !matches!(segments[i], Some(Segment::Tech { .. })) {
            continue;
        }
        let taken_before = match i.checked_sub(1).and_then(|j| segments[j].as_mut()) {
            Some(Segment::Text(text)) => {
                let keep = text.trim_end_matches(|c: char| !c.is_whitespace()).len();
                text.split_off(keep)
            }
            _ => String::new(),
        };
        let taken_after = match segments.get_mut(i + 1).and_then(Option::as_mut) {
            Some(Segment::Text(text)) => {
                let end = text.find(char::is_whitespace).unwrap_or(text.len());
                let rest = text.split_off(end);
                std::mem::replace(text, rest)
            }
            _ => String::new(),
        };
        if let Some(Segment::Tech { before, after, .. }) = segments[i].as_mut() {
            *before = taken_before;
            *after = taken_after;
        }
    }
    segments
        .into_iter()
        .flatten()
        .filter(|segment| !matches!(segment, Segment::Text(text) if text.is_empty()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Segment, paragraphs, segments};

    fn tech(icon: &str, label: &str, before: &str, after: &str) -> Segment {
        Segment::Tech {
            icon: icon.into(),
            label: label.into(),
            before: before.into(),
            after: after.into(),
        }
    }

    #[test]
    fn finds_mentions_between_text() {
        assert_eq!(
            segments("Built in [[python:Python]] and [[rust]]."),
            [
                Segment::Text("Built in ".into()),
                tech("python", "Python", "", ""),
                Segment::Text(" and ".into()),
                tech("rust", "rust", "", "."),
            ]
        );
    }

    #[test]
    fn keeps_touching_punctuation_with_the_mention() {
        assert_eq!(
            segments("in [[python:Python]] ([[fastapi:FastAPI]]) with x"),
            [
                Segment::Text("in ".into()),
                tech("python", "Python", "", ""),
                Segment::Text(" ".into()),
                tech("fastapi", "FastAPI", "(", ")"),
                Segment::Text(" with x".into()),
            ]
        );
    }

    #[test]
    fn finds_strong_runs() {
        assert_eq!(
            segments("Sign in **once**, then **go**."),
            [
                Segment::Text("Sign in ".into()),
                Segment::Strong("once".into()),
                Segment::Text(", then ".into()),
                Segment::Strong("go".into()),
                Segment::Text(".".into()),
            ]
        );
    }

    #[test]
    fn leaves_unpaired_stars_as_text() {
        assert_eq!(segments("a ** b"), [Segment::Text("a ** b".into())]);
    }

    #[test]
    fn leaves_unclosed_brackets_as_text() {
        assert_eq!(segments("a [[b"), [Segment::Text("a [[b".into())]);
    }

    #[test]
    fn splits_on_blank_lines_and_drops_empties() {
        assert_eq!(
            paragraphs(" One.\n\n\n\nTwo.\nStill two. \n\n"),
            ["One.", "Two.\nStill two."]
        );
    }
}
