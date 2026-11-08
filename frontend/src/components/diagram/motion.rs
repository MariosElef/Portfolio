//! A diagram's motion, worked out in Rust as the page renders and played by the browser as
//! CSS: no script. The build-up (a part rises in, its lines draw out, and the parts they
//! reach rise in as they connect) becomes a delay per part and line. Then, for an
//! architecture, requests flow through it: each leaves a part nothing leads to, heads on
//! towards the busier parts making each part's side calls on the way, and comes back
//! along the lines that go both ways, with telemetry streaming in on its own. A pipeline
//! instead runs one change at a time down its longest way through, now and then failing
//! at a step. Both are planned as one long cycle, at random for each render, and loop.
//!
//! Every rule is scoped to the diagram's id, and its keyframes named after it, so several
//! diagrams on a page, or one page swapped for another, never share an animation.

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Write,
    hash::{DefaultHasher, Hash, Hasher},
    time::{SystemTime, UNIX_EPOCH},
};

use super::{
    edges::travel_ms,
    kinds::style,
    layout::{Route, zone_of},
};
use crate::types::content::Diagram;

/// The build-up: how long a line takes to draw, how long a part waits before its lines
/// start, and how long a part takes to rise in and a drawn line to settle.
const DRAW_MS: u32 = 900;
const SETTLE_MS: u32 = 220;
const RISE_MS: u32 = 600;
const FADE_MS: u32 = 400;

/// Requests: how long the planned traffic runs before it loops, the longest walk, how
/// long a part takes to pass a request on, and how long between new requests and between
/// telemetry reports.
const FLOW_CYCLE_MS: u32 = 60_000;
const MAX_HOPS: usize = 6;
const HANDLE_MS: (u32, u32) = (80, 260);
const NEXT_MS: (u32, u32) = (400, 1300);
const REPORT_MS: (u32, u32) = (1500, 4000);
/// How likely telemetry goes on past where it lands, to an alert and on.
const ESCALATE: f64 = 0.3;

/// How likely a part makes a side call of each kind: storing state almost always, an
/// integrity check often, a tampering warning rarely, anything else now and then.
fn side_call(kind: &str) -> f64 {
    match kind {
        "state" => 0.85,
        "integrity" => 0.6,
        "tamper" => 0.05,
        _ => 0.25,
    }
}

/// A pipeline: how long a step works, how likely a run fails, how long the result stays,
/// and the least time its planned runs cover before they loop.
const STEP_MS: (u32, u32) = (150, 400);
const FAIL_CHANCE: f64 = 0.3;
const RESULT_MS: u32 = 1400;
const PIPELINE_CYCLE_MS: u32 = 40_000;

/// The CSS that animates the diagram whose root element has the id `root`.
pub fn css(root: &str, graph: &Diagram, routes: &[Route]) -> String {
    let mut rng = Rng::seeded(&graph.title);
    let build = build(graph, routes);
    let mut out = Css::new(root);

    out.keyframes("rise", "from { opacity: 0; translate: 0 16px; }");
    out.keyframes("fade", "from { opacity: 0; }");
    out.keyframes("vanish", "to { opacity: 0; }");
    // Hidden while it waits, so its round cap shows no dot at its start.
    out.keyframes(
        "draw",
        "0% { stroke-dashoffset: 1; opacity: 0; } 1% { opacity: 1; } \
         100% { stroke-dashoffset: 0; opacity: 1; }",
    );
    for (id, at) in &build.nodes {
        out.animate(
            &format!(".n-{}", ident(id)),
            format!(
                "{} {RISE_MS}ms cubic-bezier(0.2, 0.7, 0.2, 1) {at}ms both",
                out.name("rise")
            ),
        );
    }
    for (id, at) in &build.zones {
        let zone = ident(id);
        out.animate(
            &format!("span.z-{zone}"),
            format!(
                "{} {RISE_MS}ms cubic-bezier(0.2, 0.7, 0.2, 1) {at}ms both",
                out.name("rise")
            ),
        );
        // Hairlines only fade: moving SVG shapes is unreliable in Safari.
        out.animate(
            &format!("line.z-{zone}"),
            format!("{} {RISE_MS}ms ease {at}ms both", out.name("fade")),
        );
    }
    for (index, at) in build.edges.iter().enumerate() {
        let drawn = at + DRAW_MS;
        out.animate(
            &format!(".e{index} .edge-draw"),
            format!("{} {DRAW_MS}ms ease-in-out {at}ms both", out.name("draw")),
        );
        out.animate(
            &format!(".e{index} .edge-draw"),
            format!("{} {FADE_MS}ms ease {drawn}ms forwards", out.name("vanish")),
        );
        out.animate(
            &format!(".e{index} .edge-line"),
            format!("{} {FADE_MS}ms ease {drawn}ms both", out.name("fade")),
        );
    }

    if graph.nodes.iter().all(|node| node.step) {
        pipeline(&mut out, &mut rng, graph, routes, build.end);
    } else {
        flows(&mut out, &mut rng, routes, build.end);
    }
    out.finish()
}

// ---- The build-up ---------------------------------------------------------------------

/// When each part, zone and line appears, in milliseconds from the start.
struct Build {
    nodes: Vec<(String, u32)>,
    zones: Vec<(String, u32)>,
    edges: Vec<u32>,
    /// When everything has appeared.
    end: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Event {
    Show(String),
    Advance,
    Arrive(usize),
}

/// The build-up, step by step: the parts nothing leads to on the top row rise in first;
/// a part's zone appears with it, and so do the parts nothing leads to in that zone; a
/// moment after a part appears, every line from it draws out, and the part it reaches
/// rises in as it connects.
fn build(graph: &Diagram, routes: &[Route]) -> Build {
    let zone: HashMap<&str, &str> = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), zone_of(graph, node)))
        .collect();
    let reached: HashSet<&str> = routes.iter().map(|route| route.to.as_str()).collect();
    let top = graph.nodes.iter().map(|node| node.y).min().unwrap_or(0);

    let mut events = BinaryHeap::new();
    let mut seq = 0u32;
    let mut push = |events: &mut BinaryHeap<_>, at: u32, event: Event| {
        seq += 1;
        events.push(Reverse((at, seq, event)));
    };
    for node in &graph.nodes {
        if node.y == top && !reached.contains(node.id.as_str()) {
            push(&mut events, 0, Event::Show(node.id.clone()));
        }
    }

    let mut shown: HashSet<String> = HashSet::new();
    let mut nodes = HashMap::new();
    let mut zones = HashMap::new();
    let mut edges: Vec<Option<u32>> = vec![None; routes.len()];
    while let Some(Reverse((at, _, event))) = events.pop() {
        match event {
            Event::Show(id) => {
                if !shown.insert(id.clone()) {
                    continue;
                }
                // A part, and its zone with it; or, for a line from a zone, the zone.
                let zone_id = match zone.get(id.as_str()) {
                    Some(zone_id) => {
                        nodes.insert(id.clone(), at);
                        zone_id.to_string()
                    }
                    None => id.clone(),
                };
                if !zone_id.is_empty() && !zones.contains_key(&zone_id) {
                    zones.insert(zone_id.clone(), at);
                    shown.insert(zone_id.clone());
                    for node in &graph.nodes {
                        if zone.get(node.id.as_str()) == Some(&zone_id.as_str())
                            && !reached.contains(node.id.as_str())
                        {
                            push(&mut events, at, Event::Show(node.id.clone()));
                        }
                    }
                }
                push(&mut events, at + SETTLE_MS, Event::Advance);
            }
            Event::Advance => {
                for (index, route) in routes.iter().enumerate() {
                    if edges[index].is_none() && shown.contains(&route.from) {
                        edges[index] = Some(at);
                        push(&mut events, at + DRAW_MS, Event::Arrive(index));
                    }
                }
            }
            Event::Arrive(index) => push(&mut events, at, Event::Show(routes[index].to.clone())),
        }
    }

    // Anything the walk never reached appears once the rest has.
    let walked = nodes
        .values()
        .chain(zones.values())
        .map(|at| at + RISE_MS)
        .chain(edges.iter().flatten().map(|at| at + DRAW_MS + FADE_MS))
        .max()
        .unwrap_or(0);
    for node in &graph.nodes {
        nodes.entry(node.id.clone()).or_insert(walked);
    }
    for zone in &graph.zones {
        zones.entry(zone.id.clone()).or_insert(walked);
    }
    let edges: Vec<u32> = edges.into_iter().map(|at| at.unwrap_or(walked)).collect();
    let end = nodes
        .values()
        .chain(zones.values())
        .map(|at| at + RISE_MS)
        .chain(edges.iter().map(|at| at + DRAW_MS + FADE_MS))
        .max()
        .unwrap_or(0);

    let mut nodes: Vec<_> = nodes.into_iter().collect();
    let mut zones: Vec<_> = zones.into_iter().collect();
    nodes.sort();
    zones.sort();
    Build {
        nodes,
        zones,
        edges,
        end,
    }
}

// ---- Requests through an architecture ----------------------------------------------

/// Every line's passes, out and back, planned for one cycle.
struct Traffic<'a> {
    routes: &'a [Route],
    /// (line, back) → when each pass starts and ends.
    passes: HashMap<(usize, bool), Vec<(u32, u32)>>,
    /// (line, back) → when it is free again: a line carries one light at a time each
    /// way, so a request finding it busy waits its turn.
    free: HashMap<(usize, bool), u32>,
}

impl Traffic<'_> {
    /// Sends the light along a line from `at`, or once it is free; returns when it lands.
    fn pass(&mut self, line: usize, back: bool, at: u32) -> u32 {
        let free = self.free.entry((line, back)).or_insert(0);
        let start = at.max(*free);
        let end = start + travel_ms(&self.routes[line]);
        *free = end;
        self.passes
            .entry((line, back))
            .or_default()
            .push((start, end));
        end
    }

    fn out(&self, id: &str) -> Vec<usize> {
        (0..self.routes.len())
            .filter(|&i| self.routes[i].from == id)
            .collect()
    }

    /// Out along a line, on along the lines of the same kind from where it lands, one
    /// after another, and, when it goes both ways, back. Returns when it is done.
    fn call(&mut self, rng: &mut Rng, line: usize, at: u32, depth: usize) -> u32 {
        let mut at = self.pass(line, false, at);
        if depth < MAX_HOPS {
            let kind = self.routes[line].kind.clone();
            for next in self.out(&self.routes[line].to.clone()) {
                if self.routes[next].kind == kind {
                    let handled = at + rng.between(HANDLE_MS);
                    at = self.call(rng, next, handled, depth + 1);
                }
            }
        }
        if self.routes[line].two_way {
            at = self.pass(line, true, at + rng.between(HANDLE_MS));
        }
        at
    }
}

fn flows(out: &mut Css, rng: &mut Rng, routes: &[Route], start: u32) {
    let mut traffic = Traffic {
        routes,
        passes: HashMap::new(),
        free: HashMap::new(),
    };
    let telemetry = |line: usize| routes[line].kind == "telemetry";

    // How many parts each part leads on to, telemetry aside: the busier, the likelier a
    // request heads there.
    let reach = |id: &str| -> f64 {
        let mut seen = HashSet::from([id.to_owned()]);
        let mut stack = vec![id.to_owned()];
        while let Some(at) = stack.pop() {
            for (i, route) in routes.iter().enumerate() {
                if route.from == at && !telemetry(i) && seen.insert(route.to.clone()) {
                    stack.push(route.to.clone());
                }
            }
        }
        ((seen.len()) as f64).sqrt()
    };
    let reached: HashSet<&str> = routes.iter().map(|route| route.to.as_str()).collect();
    let mut sources: Vec<&str> = Vec::new();
    for (i, route) in routes.iter().enumerate() {
        if !telemetry(i) && !sources.contains(&route.from.as_str()) {
            sources.push(&route.from);
        }
    }
    let entries: Vec<&str> = sources
        .iter()
        .copied()
        .filter(|id| !reached.contains(id))
        .collect();
    let origins = if entries.is_empty() { sources } else { entries };

    // Requests, one setting off every second or so, overlapping as they would.
    let mut at = rng.between(NEXT_MS);
    while at < FLOW_CYCLE_MS && !origins.is_empty() {
        let from = origins[rng.pick_by(&origins, |id| reach(id))];
        let mut t = at;
        let mut here = from.to_owned();
        let mut kind: Option<String> = None;
        let mut seen = HashSet::from([here.clone()]);
        let mut walked = Vec::new();
        while walked.len() < MAX_HOPS {
            let lines: Vec<usize> = traffic
                .out(&here)
                .into_iter()
                .filter(|&i| !telemetry(i) && !seen.contains(&routes[i].to))
                .collect();
            // It carries on along lines of its own kind; the rest are side calls.
            let (ahead, side): (Vec<usize>, Vec<usize>) = lines
                .into_iter()
                .partition(|&i| kind.as_deref().is_none_or(|kind| routes[i].kind == kind));
            let calls: Vec<usize> = side
                .into_iter()
                .filter(|&i| rng.unit() < side_call(&routes[i].kind))
                .collect();
            let mut done = t;
            for &call in &calls {
                seen.insert(routes[call].to.clone());
                done = done.max(traffic.call(rng, call, t, 0));
            }
            t = done;
            if ahead.is_empty() {
                break;
            }
            let line = ahead[rng.pick_by(&ahead, |&i| reach(&routes[i].to))];
            kind.get_or_insert_with(|| routes[line].kind.clone());
            walked.push(line);
            t = traffic.pass(line, false, t) + rng.between(HANDLE_MS);
            here = routes[line].to.clone();
            seen.insert(here.clone());
        }
        // The answer retraces the lines that go both ways, as far as they do.
        for &line in walked.iter().rev() {
            if !routes[line].two_way {
                break;
            }
            t = traffic.pass(line, true, t) + rng.between(HANDLE_MS);
        }
        at += rng.between(NEXT_MS);
    }

    // Telemetry: a report from wherever it comes, now and then going on to an alert.
    let collectors: HashSet<&str> = (0..routes.len())
        .filter(|&i| telemetry(i))
        .map(|i| routes[i].to.as_str())
        .collect();
    let reporters: Vec<usize> = (0..routes.len())
        .filter(|&i| telemetry(i) && !collectors.contains(routes[i].from.as_str()))
        .collect();
    let mut at = rng.between(REPORT_MS);
    while at < FLOW_CYCLE_MS && !reporters.is_empty() {
        let mut line = reporters[rng.pick(reporters.len())];
        let mut t = traffic.pass(line, false, at);
        while rng.unit() < ESCALATE {
            let next: Vec<usize> = traffic
                .out(&routes[line].to)
                .into_iter()
                .filter(|&i| telemetry(i))
                .collect();
            if next.is_empty() {
                break;
            }
            line = next[rng.pick(next.len())];
            t = traffic.pass(line, false, t + rng.between(HANDLE_MS));
        }
        at += rng.between(REPORT_MS);
    }

    // Each line's light, out and back: hidden at its start or end between passes, and
    // sliding along it during each.
    let mut keys: Vec<_> = traffic.passes.keys().copied().collect();
    keys.sort_unstable();
    for (line, back) in keys {
        let passes: Vec<(u32, u32)> = traffic.passes[&(line, back)]
            .iter()
            .copied()
            .filter(|&(_, end)| end <= FLOW_CYCLE_MS)
            .collect();
        if passes.is_empty() {
            continue;
        }
        let (before, after) = if back {
            ("-1", "var(--streak)")
        } else {
            ("var(--streak)", "-1")
        };
        let mut timeline = Timeline::new(FLOW_CYCLE_MS, format!("stroke-dashoffset: {before}"));
        for (start, end) in passes {
            timeline.slide(
                start,
                end,
                format!("stroke-dashoffset: {before}"),
                format!("stroke-dashoffset: {after}"),
            );
        }
        let direction = if back { "back" } else { "out" };
        let name = out.keyframes(
            &format!("f{line}{direction}"),
            &timeline.finish(format!("stroke-dashoffset: {before}")),
        );
        out.animate(
            &format!(".c{line} .current-{direction}"),
            format!("{name} {FLOW_CYCLE_MS}ms linear {start}ms infinite"),
        );
    }
}

// ---- A pipeline ------------------------------------------------------------------------

fn pipeline(out: &mut Css, rng: &mut Rng, graph: &Diagram, routes: &[Route], start: u32) {
    let next = |id: &str| -> Vec<usize> {
        (0..routes.len())
            .filter(|&i| routes[i].from == id)
            .collect()
    };
    fn depth(routes: &[Route], id: &str, seen: &mut HashSet<String>) -> usize {
        if !seen.insert(id.to_owned()) {
            return 0;
        }
        let deepest = routes
            .iter()
            .filter(|route| route.from == id)
            .map(|route| 1 + depth(routes, &route.to, seen))
            .max()
            .unwrap_or(0);
        seen.remove(id);
        deepest
    }
    // The longest way through, so a run passes every check.
    let reached: HashSet<&str> = routes.iter().map(|route| route.to.as_str()).collect();
    let Some(first) = graph
        .nodes
        .iter()
        .find(|node| !reached.contains(node.id.as_str()))
    else {
        return;
    };
    let mut way = Vec::new();
    let mut here = first.id.clone();
    while let Some(line) = next(&here)
        .into_iter()
        .max_by_key(|&i| depth(routes, &routes[i].to, &mut HashSet::new()))
    {
        way.push(line);
        here = routes[line].to.clone();
        if way.len() > routes.len() {
            break;
        }
    }
    if way.is_empty() {
        return;
    }

    // Runs, one after another, until they fill the cycle.
    let mut fills: HashMap<usize, Vec<(u32, u32, u32)>> = HashMap::new();
    let mut passed: HashMap<String, Vec<(u32, u32)>> = HashMap::new();
    let mut failed: HashMap<String, Vec<(u32, u32)>> = HashMap::new();
    let mut red: HashMap<usize, Vec<(u32, u32)>> = HashMap::new();
    let mut t = 0;
    while t < PIPELINE_CYCLE_MS {
        let begin = t;
        let fails = (rng.unit() < FAIL_CHANCE).then(|| rng.pick(way.len()));
        let mut this_run_fills = Vec::new();
        let mut this_run_passed = vec![(first.id.clone(), begin)];
        let mut this_run_failed = None;
        for (step, &line) in way.iter().enumerate() {
            t += rng.between(STEP_MS);
            let fill_start = t;
            t += (travel_ms(&routes[line]) * 12 / 10).max(450);
            this_run_fills.push((line, fill_start, t));
            let reached = routes[line].to.clone();
            if fails == Some(step) {
                this_run_failed = Some((reached, line, t));
                break;
            }
            this_run_passed.push((reached, t));
        }
        t += RESULT_MS;
        for (line, fill_start, filled) in this_run_fills {
            fills.entry(line).or_default().push((fill_start, filled, t));
        }
        for (id, at) in this_run_passed {
            passed.entry(id).or_default().push((at, t));
        }
        if let Some((id, line, at)) = this_run_failed {
            failed.entry(id).or_default().push((at, t));
            red.entry(line).or_default().push((at, t));
        }
    }
    let cycle = t;
    let run = |name: &str| format!("{name} {cycle}ms linear {start}ms infinite");

    // The light fills each line like a progress bar, and stays until the run ends.
    out.rule(
        ".edge-current",
        "stroke-dashoffset: 0; stroke-dasharray: 0 2;",
    );
    for (line, windows) in sorted(fills) {
        let mut timeline = Timeline::new(cycle, "stroke-dasharray: 0 2".into());
        for (begin, filled, end) in windows {
            timeline.slide(
                begin,
                filled,
                "stroke-dasharray: 0 2".into(),
                "stroke-dasharray: 1 2".into(),
            );
            timeline.hold(end, "stroke-dasharray: 0 2".into());
        }
        let name = out.keyframes(
            &format!("p{line}"),
            &timeline.finish("stroke-dasharray: 0 2".into()),
        );
        out.animate(&format!(".c{line} .current-out"), run(&name));
    }
    // A step's check shows once a run passes it; its cross once one fails there.
    for (id, windows) in sorted(passed) {
        let name = out.keyframes(
            &format!("pass-{}", ident(&id)),
            &shown_during(cycle, &windows, "opacity"),
        );
        out.animate(&format!(".n-{} .step-pass", ident(&id)), run(&name));
    }
    for (id, windows) in sorted(failed) {
        let id = ident(&id);
        let name = out.keyframes(
            &format!("fail-{id}"),
            &shown_during(cycle, &windows, "opacity"),
        );
        out.animate(&format!(".n-{id} .step-fail"), run(&name));
        let calm = "border-color: color-mix(in oklab, white 15%, transparent); box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.3)";
        let alarm = "border-color: var(--color-red-400); box-shadow: 0 0 24px color-mix(in oklab, var(--color-red-500) 45%, transparent)";
        let mut timeline = Timeline::new(cycle, calm.into());
        for (begin, end) in windows {
            timeline.hold(begin, alarm.into());
            timeline.hold(end, calm.into());
        }
        let name = out.keyframes(&format!("alarm-{id}"), &timeline.finish(calm.into()));
        out.animate(&format!(".n-{id}"), run(&name));
    }
    // The line into a failed step turns red with it.
    for (line, windows) in sorted(red) {
        let normal = format!("stroke: {}", style(&routes[line].kind).color());
        let mut timeline = Timeline::new(cycle, normal.clone());
        for (begin, end) in windows {
            timeline.hold(begin, "stroke: var(--color-red-400)".into());
            timeline.hold(end, normal.clone());
        }
        let name = out.keyframes(&format!("red{line}"), &timeline.finish(normal));
        out.animate(&format!(".e{line} .edge-line path"), run(&name));
        out.animate(&format!(".c{line} .glow"), run(&name));
    }
}

fn sorted<K: Ord, V>(map: HashMap<K, V>) -> Vec<(K, V)> {
    let mut entries: Vec<_> = map.into_iter().collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}

/// `property` at 1 during each window, at 0 otherwise, switching at once.
fn shown_during(cycle: u32, windows: &[(u32, u32)], property: &str) -> String {
    let mut timeline = Timeline::new(cycle, format!("{property}: 0"));
    for &(begin, end) in windows {
        timeline.hold(begin, format!("{property}: 1"));
        timeline.hold(end, format!("{property}: 0"));
    }
    timeline.finish(format!("{property}: 0"))
}

// ---- Writing it out ------------------------------------------------------------------

/// Keyframes over a cycle: values that hold until the next change, or slide to it.
struct Timeline {
    cycle: u32,
    frames: Vec<(u32, String, &'static str)>,
}

impl Timeline {
    fn new(cycle: u32, first: String) -> Self {
        Self {
            cycle,
            frames: vec![(0, first, "step-end")],
        }
    }

    /// The time for a new frame: never before the last, so frames keep their order.
    fn at(&self, at: u32) -> u32 {
        let last = self.frames.last().map_or(0, |frame| frame.0);
        at.clamp(last, self.cycle)
    }

    /// Jumps to `value` at `at` and holds it.
    fn hold(&mut self, at: u32, value: String) {
        let at = self.at(at);
        self.frames.push((at, value, "step-end"));
    }

    /// Slides from `from` at `begin` to `to` at `end`, then holds `to`.
    fn slide(&mut self, begin: u32, end: u32, from: String, to: String) {
        let begin = self.at(begin);
        self.frames.push((begin, from, "ease-in-out"));
        let end = self.at(end.max(begin + 1));
        self.frames.push((end, to, "step-end"));
    }

    fn finish(mut self, last: String) -> String {
        self.frames.push((self.cycle, last, "step-end"));
        let mut css = String::new();
        let mut previous = None;
        for (at, value, timing) in self.frames {
            // Two frames at one moment would cascade into one: the later one goes just after.
            let mut percent = f64::from(at) * 100.0 / f64::from(self.cycle);
            if let Some(previous) = previous
                && percent <= previous
            {
                percent = previous + 0.0001;
            }
            previous = Some(percent);
            let _ = write!(
                css,
                "{:.4}% {{ {value}; animation-timing-function: {timing}; }} ",
                percent.min(100.0)
            );
        }
        css
    }
}

/// The rules and keyframes for one diagram, scoped to its root.
struct Css {
    root: String,
    keyframes: String,
    /// Selector → its animations, written together since one replaces another.
    animations: Vec<(String, Vec<String>)>,
    rules: String,
}

impl Css {
    fn new(root: &str) -> Self {
        Self {
            root: root.to_owned(),
            keyframes: String::new(),
            animations: Vec::new(),
            rules: String::new(),
        }
    }

    fn name(&self, name: &str) -> String {
        format!("{}-{name}", self.root)
    }

    /// Declares keyframes and returns their name.
    fn keyframes(&mut self, name: &str, body: &str) -> String {
        let name = self.name(name);
        let _ = write!(self.keyframes, "@keyframes {name} {{ {body}}} ");
        name
    }

    fn animate(&mut self, selector: &str, animation: String) {
        match self.animations.iter_mut().find(|(s, _)| s == selector) {
            Some((_, list)) => list.push(animation),
            None => self.animations.push((selector.to_owned(), vec![animation])),
        }
    }

    fn rule(&mut self, selector: &str, body: &str) {
        let _ = write!(self.rules, "#{} {selector} {{ {body} }} ", self.root);
    }

    /// Everything, for visitors who have not asked for less motion.
    fn finish(self) -> String {
        let mut css = String::from("@media (prefers-reduced-motion: no-preference) { ");
        css.push_str(&self.keyframes);
        css.push_str(&self.rules);
        for (selector, list) in self.animations {
            let _ = write!(
                css,
                "#{} {selector} {{ animation: {}; }} ",
                self.root,
                list.join(", ")
            );
        }
        css.push('}');
        css
    }
}

/// `text` as a CSS identifier: lowercase letters, digits and hyphens.
pub fn ident(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

/// A small random number generator, seeded afresh for each render, so the traffic
/// differs from one visit to the next. Not for anything that needs to be unpredictable.
struct Rng(u64);

impl Rng {
    fn seeded(salt: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        salt.hash(&mut hasher);
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| since.as_nanos())
            .hash(&mut hasher);
        Self(hasher.finish() | 1)
    }

    fn next(&mut self) -> u64 {
        // xorshift64
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// In [0, 1).
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// In [low, high).
    fn between(&mut self, (low, high): (u32, u32)) -> u32 {
        low + (self.unit() * f64::from(high - low)) as u32
    }

    /// An index into `len` items.
    fn pick(&mut self, len: usize) -> usize {
        ((self.unit() * len as f64) as usize).min(len.saturating_sub(1))
    }

    /// An index into `items`, picked by weight.
    fn pick_by<T>(&mut self, items: &[T], weight: impl Fn(&T) -> f64) -> usize {
        let weights: Vec<f64> = items.iter().map(weight).collect();
        let mut roll = self.unit() * weights.iter().sum::<f64>();
        for (index, w) in weights.iter().enumerate() {
            roll -= w;
            if roll < 0.0 {
                return index;
            }
        }
        items.len().saturating_sub(1)
    }
}

#[cfg(test)]
mod tests {
    use super::{Timeline, ident};

    #[test]
    fn makes_identifiers() {
        assert_eq!(
            ident("A request, from the outside in"),
            "a-request--from-the-outside-in"
        );
    }

    #[test]
    fn frames_never_share_a_moment() {
        let mut timeline = Timeline::new(1000, "opacity: 0".into());
        timeline.hold(0, "opacity: 1".into());
        timeline.hold(500, "opacity: 0".into());
        let css = timeline.finish("opacity: 0".into());
        assert!(css.starts_with("0.0000% { opacity: 0;"));
        assert!(css.contains("0.0001% { opacity: 1;"));
        assert!(css.contains("50.0000% { opacity: 0;"));
        assert!(css.contains("100.0000%"));
    }
}
