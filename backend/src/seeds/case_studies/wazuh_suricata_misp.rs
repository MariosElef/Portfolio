use crate::{
    models::CaseStudyDocument,
    seeds::blocks::{
        both_ways, diagram, edge, emphasis, faq, heading, list, node, outside, text, zone,
    },
};

/// The case study at `/projects/wazuh-suricata-misp`.
pub fn study() -> CaseStudyDocument {
    CaseStudyDocument {
        project: "wazuh-suricata-misp".into(),
        blocks: vec![
            text(
                "Hospitals and critical infrastructure need to know when they are under \
                 attack, but their data cannot leave the building raw, and a security alert \
                 is only worth something if no one could have changed it on the way. \
                 **This platform** watches the network on site, sends only what is needed \
                 to the cloud, and makes **every alert provable**, from the moment it is \
                 raised to a permanent record on a blockchain.\n\n\
                 Built at **IANUS Technologies** for a research consortium, which set out \
                 what the platform had to do and why. **The code and the integration are \
                 mine**, written on my own from start to finish.",
            ),
            heading("What I built"),
            list(&[
                "**Tamper-evident alerts.** Every alert is signed with HMAC-SHA256 the \
                 moment Suricata raises it and checked again when it reaches the cloud. A \
                 mismatch emails the security team at once.",
                "**Proof that lasts.** A verifier checks every alert a third time, batches a \
                 hundred at a time into a Merkle tree and anchors the root on Hyperledger \
                 Fabric, signing in through Keycloak.",
                "**Threat intelligence.** Every IP address, domain and file hash is checked \
                 against MISP, kept current from CIRCL's public feed.",
                "**Two clusters.** On-site and cloud Kubernetes clusters, with agents \
                 enrolling across them and everything deployed from GitLab CI.",
            ]),
            heading("The architecture"),
            diagram(
                "An alert, from the network to the record",
                vec![
                    zone("site", "On site", (0, 0, 1164, 120)),
                    zone("cloud", "Cloud", (0, 170, 1164, 120)),
                    zone("integrity", "Integrity", (0, 340, 1164, 120)),
                    zone("intel", "Threat intelligence", (0, 510, 1164, 120)),
                ],
                vec![
                    node(
                        "filebeat",
                        "Filebeat",
                        "share-2",
                        "Streams raw network events out.",
                        (20, 36, 190, 84),
                    ),
                    emphasis(
                        "suricata",
                        "Suricata",
                        "radar",
                        "Watches the network and raises alerts.",
                        (240, 36, 200, 84),
                    ),
                    node(
                        "signer",
                        "Alert signer",
                        "signature",
                        "Signs every alert the moment it is raised.",
                        (480, 36, 200, 84),
                    ),
                    node(
                        "agent",
                        "Wazuh agent",
                        "send",
                        "Ships signed alerts to the cloud, encrypted.",
                        (720, 36, 200, 84),
                    ),
                    node(
                        "journal",
                        "System log",
                        "scroll-text",
                        "Logins and service events on the host.",
                        (960, 36, 200, 84),
                    ),
                    outside(
                        "kafka",
                        "Kafka",
                        "waypoints",
                        "Another team's event stream.",
                        (20, 206, 190, 84),
                    ),
                    node(
                        "indexer",
                        "Wazuh indexer",
                        "search",
                        "Stores every alert, searchable.",
                        (480, 206, 200, 84),
                    ),
                    emphasis(
                        "manager",
                        "Wazuh manager",
                        "server",
                        "Checks each signature on arrival, and every address against MISP.",
                        (720, 206, 200, 84),
                    ),
                    node(
                        "email",
                        "Email alerts",
                        "mail",
                        "Tells the security team the moment an alert was changed.",
                        (960, 206, 200, 84),
                    ),
                    outside(
                        "anomaly",
                        "Anomaly detector",
                        "scan-eye",
                        "Another team's, on the raw stream.",
                        (20, 376, 190, 84),
                    ),
                    outside(
                        "keycloak",
                        "Keycloak",
                        "key-round",
                        "Issues the token for the blockchain.",
                        (240, 376, 200, 84),
                    ),
                    emphasis(
                        "verifier",
                        "Integrity verifier",
                        "badge-check",
                        "Checks every alert again, a Merkle tree per hundred.",
                        (480, 376, 200, 84),
                    ),
                    outside(
                        "blockchain",
                        "Hyperledger Fabric",
                        "blocks",
                        "Keeps each tree's root, for good.",
                        (980, 376, 184, 84),
                    ),
                    node(
                        "dashboard",
                        "Wazuh dashboard",
                        "layout-dashboard",
                        "Where analysts search and investigate.",
                        (240, 206, 200, 84),
                    ),
                    node(
                        "misp",
                        "MISP",
                        "shield-alert",
                        "Known malicious addresses, domains and file hashes.",
                        (720, 546, 200, 84),
                    ),
                    outside(
                        "circl",
                        "CIRCL feed",
                        "globe",
                        "Public threat intelligence, imported on a schedule.",
                        (960, 546, 204, 84),
                    ),
                ],
                vec![
                    edge("suricata", "signer", "alert"),
                    edge("signer", "agent", "alert"),
                    edge("journal", "agent", "alert"),
                    edge("agent", "manager", "alert"),
                    edge("manager", "indexer", "alert"),
                    edge("indexer", "dashboard", "alert"),
                    edge("suricata", "filebeat", "stream"),
                    edge("filebeat", "kafka", "stream"),
                    edge("kafka", "anomaly", "stream"),
                    edge("manager", "email", "tamper"),
                    edge("indexer", "verifier", "integrity"),
                    both_ways(edge("verifier", "keycloak", "integrity")),
                    edge("verifier", "blockchain", "integrity"),
                    both_ways(edge("manager", "misp", "intel")),
                    edge("circl", "misp", "intel"),
                ],
            ),
            heading("Questions about the design"),
            faq(&[
                (
                    "Why two clusters?",
                    "The data comes from inside a hospital's network and should not \
                     leave it raw. Collection runs on site; only the alerts travel \
                     to the cloud, over an encrypted connection, where they are \
                     stored, checked and searched.",
                ),
                (
                    "Why sign every alert?",
                    "An alert is evidence, and evidence is only worth something if \
                     it is untouched. Each one is signed on site the moment it is \
                     raised, then checked twice in the cloud: by the Wazuh manager \
                     on arrival, and again by the integrity verifier. A mismatch \
                     emails the security team at once.",
                ),
                (
                    "Why a blockchain?",
                    "So the record cannot be rewritten later, even by whoever runs \
                     the cloud. Only a Merkle root goes on chain, one for every \
                     hundred alerts, so the chain stays small while every alert in \
                     a batch can be checked against it.",
                ),
                (
                    "Where does threat intelligence come in?",
                    "Every IP address, domain and file hash in an alert is looked \
                     up in MISP, which imports CIRCL's public feed on a schedule, \
                     so known attackers are flagged as they appear.",
                ),
                (
                    "Where does it stand?",
                    "It runs end to end, from detection on site to the root on the \
                     blockchain. Looking an alert up on the blockchain for a \
                     forensic audit is the next step.",
                ),
            ]),
        ],
    }
}
