use rust_i18n::t;
use topcoat::{
    Result,
    context::Cx,
    icon::iconify::iconify_icon,
    view::{View, component, view},
};

use crate::{
    components::{
        actions::icon_link::icon_link,
        contribution_graph::{GraphDay, GraphWeek, contribution_graph},
        layout::{
            section::section,
            stack::{StackGap, stack},
        },
        surfaces::{
            panel::panel,
            stat::{stat, stat_grid},
        },
    },
    composables::{github::contributions, site::site},
    types::content::Contributions,
    utils::calendar::{day_and_month, month_labels, stats, thousands},
};

/// One big-number tile beside the graph.
struct Tile {
    label: String,
    value: String,
}

/// The four numbers shown beside the graph.
fn tiles(calendar: &Contributions) -> Vec<Tile> {
    let summary = stats(
        calendar
            .weeks
            .iter()
            .flatten()
            .map(|day| (day.date.as_str(), day.count)),
    );
    let busiest = summary
        .busiest
        .as_ref()
        .and_then(|(date, count)| {
            let (day, month) = day_and_month(date)?;
            Some(t!(
                "activity.busiest_value",
                count = thousands(*count),
                day = day,
                month = t!(format!("calendar.month_{month}"))
            ))
        })
        .unwrap_or_default();

    vec![
        Tile {
            label: t!("activity.total").into_owned(),
            value: thousands(calendar.total),
        },
        Tile {
            label: t!("activity.active_days").into_owned(),
            value: thousands(summary.active_days),
        },
        Tile {
            label: t!("activity.streak").into_owned(),
            value: t!("activity.days", count = summary.longest_streak).into_owned(),
        },
        Tile {
            label: t!("activity.busiest").into_owned(),
            value: busiest.into_owned(),
        },
    ]
}

/// The graph's weeks, with month labels and hover text from the locale.
fn weeks(calendar: Contributions) -> Vec<GraphWeek> {
    let months = month_labels(
        calendar
            .weeks
            .iter()
            .map(|week| week.first().map_or("", |day| day.date.as_str())),
    );
    calendar
        .weeks
        .into_iter()
        .zip(months)
        .map(|(days, month)| GraphWeek {
            month: month
                .map(|m| t!(format!("calendar.month_{m}")).into_owned())
                .unwrap_or_default(),
            days: days
                .into_iter()
                .map(|day| GraphDay {
                    weekday: day.weekday,
                    level: day.level,
                    label: t!(
                        "activity.day",
                        count = thousands(day.count),
                        date = day.date
                    )
                    .into_owned(),
                })
                .collect(),
        })
        .collect()
}

/// The year on GitHub: four headline numbers and the contribution graph. Renders
/// nothing when GitHub is not configured or cannot be read.
#[component]
pub async fn activity_section(cx: &Cx) -> Result<impl View> {
    let github = site(cx).await.github_url;
    let calendar = contributions(cx).await?;
    let content = calendar.map(|calendar| (tiles(&calendar), weeks(calendar)));
    let weekdays: Vec<String> = ["", "mon", "", "wed", "", "fri", ""]
        .into_iter()
        .map(|key| match key {
            "" => String::new(),
            key => t!(format!("calendar.{key}")).into_owned(),
        })
        .collect();
    let (less, more) = (t!("activity.less"), t!("activity.more"));

    let title = t!("activity.title");

    Ok(view! {
        if let Some((tiles, weeks)) = content {
            section(
                title: &title,
                action: view! {
                    icon_link(href: github, icon_data: iconify_icon!("lucide:github"), external: true, (t!("activity.profile")))
                }.into(),
                stack(gap: StackGap::Tight,
                    stat_grid(
                        for tile in tiles {
                            stat(label: &tile.label, value: &tile.value)
                        }
                    )
                    panel(
                        roomy: true,
                        contribution_graph(weeks: weeks, weekdays: weekdays, less: &less, more: &more)
                    )
                )
            )
        }
    })
}
