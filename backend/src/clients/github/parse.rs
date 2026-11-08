//! Reading the contribution calendar out of GitHub's HTML.

use std::{collections::BTreeMap, sync::LazyLock};

use regex::Regex;

use crate::models::{ContributionDay, Contributions};

static TOTAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"id="js-contribution-activity-description"[^>]*>\s*([\d,]+)\s+contribution"#)
        .expect("valid regex")
});
static CELL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<td[^>]*ContributionCalendar-day[^>]*>"#).expect("valid regex"));
static TOOLTIP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"<tool-tip[^>]*for="(contribution-day-component-\d+-\d+)"[^>]*>([^<]*)</tool-tip>"#,
    )
    .expect("valid regex")
});

/// The value of attribute `name` in a single opening tag.
fn attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let start = tag.find(&format!(" {name}=\""))? + name.len() + 3;
    let len = tag[start..].find('"')?;
    Some(&tag[start..start + len])
}

/// Reads the calendar out of the contributions page.
pub fn parse(html: &str) -> anyhow::Result<Contributions> {
    let total = TOTAL
        .captures(html)
        .and_then(|c| c[1].replace(',', "").parse().ok())
        .ok_or_else(|| anyhow::anyhow!("no contribution total on the page"))?;

    // "No contributions on …" or "12 contributions on …", keyed by the cell it labels.
    let counts: BTreeMap<&str, u32> = TOOLTIP
        .captures_iter(html)
        .map(|c| {
            let text = c.get(2).map_or("", |m| m.as_str());
            let count = text
                .split_whitespace()
                .next()
                .and_then(|n| n.replace(',', "").parse().ok())
                .unwrap_or(0);
            (c.get(1).map_or("", |m| m.as_str()), count)
        })
        .collect();

    // The cell id is `contribution-day-component-<weekday>-<week>`.
    let mut weeks: BTreeMap<usize, Vec<ContributionDay>> = BTreeMap::new();
    for tag in CELL.find_iter(html).map(|m| m.as_str()) {
        let (Some(id), Some(date), Some(level)) = (
            attribute(tag, "id"),
            attribute(tag, "data-date"),
            attribute(tag, "data-level"),
        ) else {
            continue;
        };
        let mut position = id.rsplit('-');
        let (Some(week), Some(weekday)) = (
            position.next().and_then(|w| w.parse().ok()),
            position.next().and_then(|d| d.parse().ok()),
        ) else {
            continue;
        };
        weeks.entry(week).or_default().push(ContributionDay {
            date: date.to_owned(),
            weekday,
            count: counts.get(id).copied().unwrap_or(0),
            level: level.parse().unwrap_or(0),
        });
    }
    anyhow::ensure!(!weeks.is_empty(), "no contribution days on the page");

    Ok(Contributions {
        total,
        weeks: weeks
            .into_values()
            .map(|mut days| {
                days.sort_by_key(|day| day.weekday);
                days
            })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::parse;

    const PAGE: &str = r#"
        <h2 id="js-contribution-activity-description" class="f4">
          4,188
            contributions
            in the last year
        </h2>
        <td tabindex="0" data-date="2025-10-19" id="contribution-day-component-0-0" data-level="0" class="ContributionCalendar-day"></td>
        <td tabindex="0" data-date="2025-10-20" id="contribution-day-component-1-0" data-level="3" class="ContributionCalendar-day"></td>
        <td tabindex="0" data-date="2025-10-26" id="contribution-day-component-0-1" data-level="1" class="ContributionCalendar-day"></td>
        <tool-tip id="t1" for="contribution-day-component-0-0" class="sr-only">No contributions on October 19th.</tool-tip>
        <tool-tip id="t2" for="contribution-day-component-1-0" class="sr-only">1,024 contributions on October 20th.</tool-tip>
        <tool-tip id="t3" for="contribution-day-component-0-1" class="sr-only">2 contributions on October 26th.</tool-tip>
    "#;

    #[test]
    fn reads_total_weeks_and_days() {
        let calendar = parse(PAGE).unwrap();
        assert_eq!(calendar.total, 4188);
        assert_eq!(calendar.weeks.len(), 2);

        let monday = &calendar.weeks[0][1];
        assert_eq!((monday.date.as_str(), monday.weekday), ("2025-10-20", 1));
        assert_eq!((monday.count, monday.level), (1024, 3));
        assert_eq!(calendar.weeks[0][0].count, 0);
        assert_eq!(calendar.weeks[1][0].count, 2);
    }

    #[test]
    fn rejects_a_page_without_the_graph() {
        assert!(parse("<html>GitHub changed its markup</html>").is_err());
    }
}
