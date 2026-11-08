use crate::{
    models::{CaseStudyDocument, StoredBlock},
    seeds::blocks::{faq, heading, list, text},
};

/// The case study at `/projects/stackup`.
pub fn study() -> CaseStudyDocument {
    CaseStudyDocument {
        project: "stackup".into(),
        blocks: vec![
            text(
                "Starting a full-stack project means stitching together a frontend, a \
                 backend, a database, Docker Compose, CI and lint rules before \
                 writing a line of the product, and templates go stale the week they \
                 are published. **stackup** runs each framework's **official \
                 scaffolder**, adds the packages chosen, and writes the glue around \
                 them: Docker Compose, local HTTPS, CI, lint rules, environment files \
                 and a git repository. It ships no template of its own, so every \
                 project is **as current as the tools**.\n\nThen it stays useful: the \
                 same full-screen menu starts, stops, migrates, backs up and deploys \
                 every project it made.",
            ),
            heading("What I built"),
            list(&[
                "**A full-screen menu.** Built with Ratatui: a wizard with a live \
                 preview of the file tree and the Docker Compose file, a project list \
                 showing what is running, and a terminal inside the app. Keys follow \
                 Vim, with a command palette and fuzzy search.",
                "**Official tools, not templates.** Nuxt, Vue or Topcoat in front, \
                 Axum or FastAPI behind, and Postgres, Redis, MongoDB, Keycloak and \
                 local HTTPS beside them, each made by its own generator at its \
                 latest version.",
                "**Layers that hold.** Every generated project has its layering \
                 checked in CI, so a route can never reach the database directly. \
                 stackup checks its own code the same way.",
                "**One plan, two runners.** Every action is compiled into a list of \
                 steps. The command line and the menu run the same steps, and \
                 `--dry-run` prints them.",
                "**Deploy without a registry.** Images are streamed to the server \
                 over SSH and started there with Docker Compose.",
                "**Tested on three systems.** Over 300 tests, run in CI on Linux, \
                 macOS and Windows, with release builds for five platforms.",
            ]),
            heading("In a terminal"),
            StoredBlock::Code {
                lang: "sh".into(),
                code: "stackup new my-app --frontend nuxt --backend axum \\\n  --services postgres,redis --tools lint,ci,git\ncd my-app\nstackup up".into(),
            },
            heading("Questions about the design"),
            faq(&[
                (
                    "Why not templates?",
                    "A template is out of date the day after it ships. The official \
                     scaffolders are kept current by the people who build the \
                     frameworks, so stackup runs them and writes only what they leave \
                     out: the files that make the pieces work together.",
                ),
                (
                    "Why a menu and a command line?",
                    "The menu is for exploring the choices with a live preview; the \
                     command line is for scripts and repeat runs. The wizard's \
                     choices become the same command-line flags, parsed by the same \
                     code, and a test checks they survive the round trip.",
                ),
                (
                    "Where does it stand?",
                    "Version 0.1, with every feature above working. Release builds \
                     and install scripts are ready, with Homebrew, Arch, Scoop and \
                     winget packages waiting on the first tagged release. Windows \
                     builds and passes its tests in CI but has not yet run on a real \
                     machine.",
                ),
            ]),
        ],
    }
}
