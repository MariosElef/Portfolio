//! What each kind of line means, for the diagrams that draw it. A diagram's legend is
//! the entries for the kinds its lines use.

use std::collections::BTreeMap;

use crate::models::{DiagramEdge, LegendEntry};

/// Each kind: its label, and what crosses a line of that kind.
pub const LEGEND: &[(&str, &str, &str)] = &[
    (
        "request",
        "A request",
        "A request and its answer: browser, tunnel, router, then the gateway, a reverse proxy that forwards it to one API and waits for the reply. Only sign-in pages go from the router straight to the identity provider.",
    ),
    (
        "webhook",
        "A webhook",
        "Stripe telling the platform what happened, on its own host and never through the gateway. The signature is checked and each event stored once, for payments to act on.",
    ),
    (
        "alert",
        "An alert",
        "Raised by Suricata, signed on site, checked again in the cloud, then stored for analysts to search.",
    ),
    (
        "stream",
        "The raw stream",
        "Raw network events, streamed through Kafka to another team's anomaly detector.",
    ),
    (
        "tamper",
        "Tampering",
        "A signature that does not match: the alert was changed on the way, and the security team is emailed at once.",
    ),
    (
        "integrity",
        "Integrity",
        "Every alert checked again, a hundred at a time in a Merkle tree, its root kept on the blockchain with a token from Keycloak.",
    ),
    (
        "intel",
        "Threat intelligence",
        "Addresses, domains and file hashes checked against MISP, which imports a public feed on a schedule.",
    ),
    (
        "identity",
        "Identity",
        "Tokens and proofs: sign-in, refresh, one exchanged token per API, sign-out, services calling each other, and a device joining the network once.",
    ),
    (
        "agent",
        "The agent",
        "Any API can ask the AI agent. It acts as the user and never touches the database.",
    ),
    (
        "delivery",
        "A change shipping",
        "A merge, built, checked and deployed inside the cluster, with no credentials anywhere else.",
    ),
    (
        "session",
        "Sessions",
        "The gateway keeps each session server side, beside it; the browser only holds a cookie naming it.",
    ),
    (
        "state",
        "State",
        "Where things are kept: each service's own role and schema in the database.",
    ),
    (
        "telemetry",
        "Telemetry",
        "Traces, logs and metrics from every layer, gathered in one place. Alert rules watch them and post to the team chat, so a failure is seen the moment it happens.",
    ),
];

/// The legend for a diagram drawing `edges`: an entry for each kind they use.
pub fn for_edges(edges: &[DiagramEdge]) -> BTreeMap<String, LegendEntry> {
    LEGEND
        .iter()
        .filter(|(kind, ..)| edges.iter().any(|edge| edge.kind == *kind))
        .map(|&(kind, label, detail)| {
            (
                kind.into(),
                LegendEntry {
                    label: label.into(),
                    detail: detail.into(),
                },
            )
        })
        .collect()
}
