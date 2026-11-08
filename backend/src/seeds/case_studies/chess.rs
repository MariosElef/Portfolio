use crate::{
    models::CaseStudyDocument,
    seeds::blocks::{faq, heading, list, text},
};

/// The case study at `/projects/chess`.
pub fn study() -> CaseStudyDocument {
    CaseStudyDocument {
        project: "chess".into(),
        blocks: vec![
            text(
                "A complete game of chess for two players at one screen, with every \
                 rule in place, from castling and en passant to promotion, check, \
                 checkmate and stalemate. Beyond the game itself, it has **ten \
                 puzzles** to solve, **chess clocks** for timed games and **board \
                 themes** to choose from, and it plays **right in the browser**.",
            ),
            heading("What I built"),
            list(&[
                "**Every rule.** Moves are worked out for each piece, then narrowed \
                 to the legal ones: in check, only a move that captures the attacker \
                 or blocks it is allowed, and in double check only the king may move.",
                "**Puzzles.** Ten positions, each loaded from FEN notation, with the \
                 winning moves checked as they are played. Solving one unlocks the \
                 next, and progress is kept between visits.",
                "**Timed games.** Clocks of 2, 5, 10 or 30 minutes, or none at all.",
                "**Themes and sound.** Boards to choose from, remembered between \
                 visits, with sounds for captures, castling and checkmate, and music.",
            ]),
            heading("Questions about the design"),
            faq(&[
                (
                    "How does it know which moves are legal?",
                    "Each piece lists where it can go, then the game checks the king. \
                     In check, a move is allowed only if it captures the attacking \
                     piece or steps between it and the king; with two attackers, only \
                     the king can move. When a side has no legal move left, it is \
                     checkmate if its king is attacked and stalemate if not.",
                ),
                (
                    "Where do the puzzles come from?",
                    "Each is a position in FEN, the standard notation for a chess \
                     position, with its solution as a list of moves, kept in a JSON \
                     file so new ones can be added without touching the code.",
                ),
                (
                    "Where can it be played?",
                    "On itch.io, in the browser, with nothing to install.",
                ),
            ]),
        ],
    }
}
