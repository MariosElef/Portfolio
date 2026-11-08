/// The month (1–12) of a `YYYY-MM-DD` date.
pub fn month(date: &str) -> Option<usize> {
    date.get(5..7)?
        .parse()
        .ok()
        .filter(|m| (1..=12).contains(m))
}

/// For each week (given by its first date), the month to label above it: the month
/// whenever it changes from the previous week's, `None` otherwise.
pub fn month_labels<'a>(week_starts: impl IntoIterator<Item = &'a str>) -> Vec<Option<usize>> {
    let mut previous = None;
    week_starts
        .into_iter()
        .map(|date| {
            let current = month(date);
            let label = (current != previous).then_some(current).flatten();
            previous = current;
            label
        })
        .collect()
}

/// `n` with commas between thousands: 4188 → "4,188".
pub fn thousands(n: u32) -> String {
    let digits = n.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// The day of the month and the month (1–12) of a `YYYY-MM-DD` date.
pub fn day_and_month(date: &str) -> Option<(u32, usize)> {
    Some((date.get(8..10)?.parse().ok()?, month(date)?))
}

/// What a year of daily counts adds up to.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Stats {
    /// Days with at least one contribution.
    pub active_days: u32,
    /// The most days in a row with at least one.
    pub longest_streak: u32,
    /// The busiest day's date and count.
    pub busiest: Option<(String, u32)>,
}

/// Stats for `days`, given oldest first as `(date, count)`.
pub fn stats<'a>(days: impl IntoIterator<Item = (&'a str, u32)>) -> Stats {
    let mut stats = Stats::default();
    let mut streak = 0;
    for (date, count) in days {
        if count == 0 {
            streak = 0;
            continue;
        }
        stats.active_days += 1;
        streak += 1;
        stats.longest_streak = stats.longest_streak.max(streak);
        if stats.busiest.as_ref().is_none_or(|(_, most)| count > *most) {
            stats.busiest = Some((date.to_owned(), count));
        }
    }
    stats
}

#[cfg(test)]
mod tests {
    use super::{Stats, day_and_month, month, month_labels, stats, thousands};

    #[test]
    fn reads_day_and_month() {
        assert_eq!(day_and_month("2026-10-04"), Some((4, 10)));
    }

    #[test]
    fn counts_active_days_streaks_and_the_busiest_day() {
        let days = [
            ("a", 1),
            ("b", 3),
            ("c", 0),
            ("d", 2),
            ("e", 9),
            ("f", 1),
            ("g", 0),
        ];
        assert_eq!(
            stats(days),
            Stats {
                active_days: 5,
                longest_streak: 3,
                busiest: Some(("e".into(), 9))
            }
        );
    }

    #[test]
    fn an_empty_year_has_no_busiest_day() {
        assert_eq!(stats([("a", 0)]), Stats::default());
    }

    #[test]
    fn reads_the_month() {
        assert_eq!(month("2026-10-18"), Some(10));
        assert_eq!(month("bad"), None);
    }

    #[test]
    fn labels_each_month_once() {
        let labels = month_labels(["2025-10-19", "2025-10-26", "2025-11-02", "2025-11-09"]);
        assert_eq!(labels, [Some(10), None, Some(11), None]);
    }

    #[test]
    fn groups_thousands() {
        assert_eq!(thousands(7), "7");
        assert_eq!(thousands(4188), "4,188");
        assert_eq!(thousands(1_234_567), "1,234,567");
    }
}
