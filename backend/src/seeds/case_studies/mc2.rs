use crate::{
    models::CaseStudyDocument,
    seeds::blocks::{
        both_ways, diagram, edge, emphasis, faq, heading, node, outside, plain, round_trip, step,
        text, zone,
    },
};

/// The case study at `/projects/mc2`.
pub fn study() -> CaseStudyDocument {
    CaseStudyDocument {
        project: "mc2".into(),
        blocks: vec![
            text(
                "Most businesses run on a patchwork of separate tools, each with \
                 its own login, its own copy of the customer list and its own \
                 rules for who may do what. **MC2 brings them together**: \
                 quoting and invoicing, accounting, payments and subscriptions, \
                 each a dedicated application behind **a single account**. Teams \
                 sign in once, move between the businesses they serve, and every \
                 application already knows who they are and what they are \
                 permitted to do.\n\n\
                 Accounting is taken care of as well: **MC2's accountants keep \
                 the books**, leaving owners free to focus on the performance \
                 and day-to-day running of their business.",
            ),
            heading("The architecture"),
            diagram(
                "A request, from the outside in",
                vec![
                    zone("clients", "Clients", (0, 0, 1164, 120)),
                    zone("network", "Private network", (0, 170, 1164, 120)),
                    zone("edge", "Edge", (0, 340, 1164, 120)),
                    zone("presentation", "Presentation", (0, 590, 250, 120)),
                    zone("ai", "AI", (290, 590, 150, 120)),
                    zone("access", "Access control", (472, 590, 692, 120)),
                    zone("application", "Application", (0, 840, 1164, 120)),
                    zone("data", "Data", (0, 1090, 1164, 120)),
                    zone("observability", "Observability", (1194, 340, 200, 990)),
                ],
                vec![
                    outside(
                        "stripe",
                        "Stripe",
                        "credit-card",
                        "Payments, outside the platform. Tells it what happened, signed.",
                        (0, 36, 246, 84),
                    ),
                    node(
                        "browser",
                        "Browser",
                        "globe",
                        "A signed, HttpOnly session cookie. Never a token.",
                        (270, 36, 300, 84),
                    ),
                    node(
                        "device",
                        "Team device",
                        "laptop",
                        "Registered once, by an admin.",
                        (594, 36, 300, 84),
                    ),
                    node(
                        "tunnel",
                        "Encrypted tunnel",
                        "lock",
                        "Every hostname resolves to a private address.",
                        (270, 206, 300, 84),
                    ),
                    node(
                        "control",
                        "Control plane",
                        "door-open",
                        "The one public door: two ports, nothing else.",
                        (594, 206, 300, 84),
                    ),
                    node(
                        "webhooks",
                        "Webhooks",
                        "webhook",
                        "Its own host, never behind the gateway. Checks the signature, stores each event once.",
                        (160, 376, 260, 84),
                    ),
                    node(
                        "router",
                        "Router",
                        "route",
                        "One host per app, split by path. TLS, security headers, sign-in rate limits.",
                        (460, 376, 300, 84),
                    ),
                    node(
                        "frontends",
                        "App frontends",
                        "layout-template",
                        "Server-rendered, one per product, on a shared design layer.",
                        (0, 626, 250, 84),
                    ),
                    node(
                        "agent",
                        "AI agent",
                        "bot",
                        "Any API can ask it. Acts as the user; no database.",
                        (290, 626, 150, 84),
                    ),
                    emphasis(
                        "gateway",
                        "Gateway",
                        "shield-check",
                        "Session, entitlement, refresh, a token per API, a proof per request.",
                        (560, 626, 200, 84),
                    ),
                    plain(
                        "sessions",
                        "Sessions",
                        "key-round",
                        "Tokens, kept server side.",
                        (784, 626, 150, 84),
                    ),
                    node(
                        "identity",
                        "Identity provider",
                        "fingerprint",
                        "The only issuer. Two-factor, businesses, roles.",
                        (964, 626, 200, 84),
                    ),
                    node(
                        "accounts",
                        "Accounts",
                        "building-2",
                        "Businesses, invitations, entitlements, organizations.",
                        (160, 876, 181, 84),
                    ),
                    plain(
                        "mail",
                        "Mail",
                        "mail",
                        "Internal only, one caller.",
                        (0, 876, 130, 84),
                    ),
                    node(
                        "operations",
                        "Operations API",
                        "receipt",
                        "Invoices, quotes, expenses.",
                        (365, 876, 181, 84),
                    ),
                    node(
                        "accounting",
                        "Accounting API",
                        "book-open",
                        "A double-entry ledger.",
                        (570, 876, 181, 84),
                    ),
                    node(
                        "payments",
                        "Payments API",
                        "credit-card",
                        "Checkout, subscriptions, payment events.",
                        (775, 876, 181, 84),
                    ),
                    node(
                        "catalogue",
                        "Catalogue API",
                        "layout-grid",
                        "The apps a business can use.",
                        (980, 876, 181, 84),
                    ),
                    node(
                        "database",
                        "Relational database",
                        "database",
                        "A schema per domain and a role per service. The ledger and the audit trail are hash-chained.",
                        (150, 1126, 700, 84),
                    ),
                    node(
                        "scheduler",
                        "Scheduler",
                        "clock",
                        "Jobs, on a database of their own.",
                        (880, 1126, 130, 84),
                    ),
                    node(
                        "migrations",
                        "Migrations",
                        "layers",
                        "Every schema change, before a rollout.",
                        (1034, 1126, 130, 84),
                    ),
                    node(
                        "telemetry",
                        "Telemetry",
                        "activity",
                        "Traces, logs and metrics from every service and the browser. Thirty days kept, dashboards in git.",
                        (1194, 376, 200, 584),
                    ),
                    node(
                        "alerts",
                        "Alerts",
                        "bell",
                        "Rules on server errors, memory, disk, certificates and backups.",
                        (1194, 1126, 200, 84),
                    ),
                    plain(
                        "chat",
                        "Team chat",
                        "message-square",
                        "Every alert lands in the team's channel, the moment it fires.",
                        (1194, 1246, 200, 84),
                    ),
                ],
                vec![
                    round_trip("browser", "tunnel"),
                    round_trip("tunnel", "router"),
                    round_trip("router", "frontends"),
                    round_trip("router", "gateway"),
                    round_trip("router", "identity"),
                    round_trip("gateway", "agent"),
                    round_trip("gateway", "accounts"),
                    round_trip("gateway", "operations"),
                    round_trip("gateway", "accounting"),
                    round_trip("gateway", "payments"),
                    round_trip("gateway", "catalogue"),
                    edge("device", "control", "identity"),
                    both_ways(edge("gateway", "identity", "identity")),
                    edge("accounts", "mail", "identity"),
                    edge("accounts", "agent", "agent"),
                    edge("operations", "agent", "agent"),
                    edge("accounting", "agent", "agent"),
                    edge("payments", "agent", "agent"),
                    edge("catalogue", "agent", "agent"),
                    edge("gateway", "sessions", "session"),
                    edge("accounts", "database", "state"),
                    edge("operations", "database", "state"),
                    edge("accounting", "database", "state"),
                    edge("payments", "database", "state"),
                    edge("catalogue", "database", "state"),
                    edge("stripe", "webhooks", "webhook"),
                    edge("router", "telemetry", "telemetry"),
                    edge("access", "telemetry", "telemetry"),
                    edge("application", "telemetry", "telemetry"),
                    edge("telemetry", "alerts", "telemetry"),
                    edge("alerts", "chat", "telemetry"),
                ],
            ),
            diagram(
                "How a change ships",
                vec![zone("delivery", "Delivery", (0, 0, 1200, 290))],
                vec![
                    step(
                        "merge",
                        "Merge",
                        "A key that can only start a build",
                        (70, 44, 56, 56),
                    ),
                    step(
                        "checkout",
                        "Checkout",
                        "The commit, in the cluster",
                        (275, 44, 56, 56),
                    ),
                    step(
                        "checks",
                        "CI checks",
                        "The repository's own, against a real database",
                        (480, 180, 56, 56),
                    ),
                    step(
                        "build",
                        "Build image",
                        "Rootless, tagged with the commit",
                        (685, 44, 56, 56),
                    ),
                    step(
                        "pin",
                        "Pin image",
                        "In the deployment repository",
                        (890, 44, 56, 56),
                    ),
                    step(
                        "deploy",
                        "Deploy",
                        "The cluster syncs itself, migrations first",
                        (1095, 44, 56, 56),
                    ),
                ],
                vec![
                    edge("merge", "checkout", "delivery"),
                    edge("checkout", "build", "delivery"),
                    edge("checkout", "checks", "delivery"),
                    edge("checks", "build", "delivery"),
                    edge("build", "pin", "delivery"),
                    edge("pin", "deploy", "delivery"),
                ],
            ),
            heading("Questions about the design"),
            faq(&[
                (
                    "What happens to one request, step by step?",
                    "1. The browser calls an app's /api path, carrying only its session cookie.\n\n2. The router sees an API path and hands it to the gateway. There is no route to the API itself.\n\n3. The gateway refuses an unsafe request from another origin, finds the session and checks it is still live: a sign-out anywhere else ends it here.\n\n4. It checks the business has this app. If the account service does not answer, it trusts what it knew for ten minutes at most, then refuses.\n\n5. If the user's token is about to expire it is refreshed, once, however many requests are in flight.\n\n6. The token is exchanged for one meant only for this API and this business, and a proof is signed binding it to this exact request.\n\n7. The cookie, and anything else the client should not have sent, is stripped, and the request goes on.\n\n8. The API checks the signature, its own name as the audience, the proof, and that the user passed two-factor for this business. Only then does the query run, scoped to that business.",
                ),
                (
                    "Why do tokens never reach the browser?",
                    "Because anything the browser holds, a script injected into the page can \
                     take. The browser keeps only a signed, HttpOnly cookie. Such a script can \
                     still make requests while the page is open, but it cannot carry a token \
                     away to use later.",
                ),
                (
                    "Why does every API get its own token?",
                    "A token minted for accounting is worthless at operations, so one leak \
                     opens one door, for one business. Every token is also bound to a key, \
                     and each request carries a fresh proof signed with it, so a copied token \
                     on its own does nothing.",
                ),
                (
                    "Why does the gateway not issue tokens itself?",
                    "The identity provider is the only issuer, so there is exactly one place \
                     that decides who someone is. The gateway only asks it for tokens and \
                     passes them on, and every service checks them for itself rather than \
                     trusting the gateway.",
                ),
                (
                    "Why not check sign-in at the router?",
                    "An auth check bolted onto the router fails open: any route that forgets \
                     it is unprotected. Here an API has no route at all, so one without a \
                     gateway entry is simply unreachable, and the network denies everything \
                     that is not allowed by name.",
                ),
                (
                    "Where do businesses and roles live?",
                    "At the identity provider: a business is an organization and its groups \
                     are the roles. The token already says who belongs where and as what, so \
                     the database keeps no copy that could drift.",
                ),
                (
                    "How are services kept out of each other's data?",
                    "Each one connects as its own database role and owns one schema. Domains \
                     read each other only through narrow views, and no service can rewrite \
                     the hash chain that seals the ledger and the audit trail.",
                ),
                (
                    "When is two-factor needed?",
                    "Whenever someone acts for a business. If the sign-in comes back without \
                     a second factor, the gateway asks once more, then refuses.",
                ),
                (
                    "What does it trade away?",
                    "One node and one replica of everything: simple and cheap while in \
                     development, and the first thing to change before customers.\n\n\
                     Tenant isolation is a business id in every query, enforced by checks in \
                     CI, rather than row-level security in the database.\n\n\
                     Everything passes through one gateway, uploads included: one path to \
                     secure and watch, and one path to scale.\n\n\
                     Being reachable only on a private network keeps the attack surface tiny, \
                     and means there is no public entrance yet.",
                ),
                (
                    "Where does it stand?",
                    "Identity, the gateway, the database and the infrastructure run in \
                     development. The product APIs and frontends are being rebuilt on top of \
                     them, database first.",
                ),
            ]),
        ],
    }
}
