mod models;
use models::{CraftMode, Developer, Status};
fn main() {
    println!("::Start App::Welcome In The Rabbit Hole::");
    let developers = vec![
        Developer {
            handle: String::from("@anna_rust"),
            stack: vec![String::from("Rust"), String::from("Backend")],
            location: String::from("Remote"),
            status: Status::Available,
            craft_mode: CraftMode::HumanOwnedAiAssisted,
        },
        Developer {
            handle: String::from("@michal"),
            stack: vec![
                String::from("Rust"),
                String::from("Ratatui"),
                String::from("AI tools"),
            ],
            location: String::from("Remote"),
            status: Status::SideQuests,
            craft_mode: CraftMode::HumanOwnedAiAssisted,
        },
        Developer {
            handle: String::from("@kubadev"),
            stack: vec![String::from("Go"), String::from("Kubernetes")],
            location: String::from("Kraków"),
            status: Status::Busy,
            craft_mode: CraftMode::ManualFirst,
        },
        Developer {
            handle: String::from("@neo"),
            stack: vec![
                String::from("Rust"),
                String::from("Matrix"),
                String::from("Systems"),
            ],
            location: String::from("Zion"),
            status: Status::Lurking,
            craft_mode: CraftMode::LearningMode,
        },
        Developer {
            handle: String::from("@mimas"),
            stack: vec![
                String::from("JS"),
                String::from("Kotlin"),
                String::from("LLM"),
            ],
            location: String::from("Biezanow"),
            status: Status::Stale,
            craft_mode: CraftMode::LearningMode,
        },
    ];
    println!("They said developers are dead.");
    println!("We kept coding.");
    println!();
    println!("Loaded {} developers:", developers.len());
    println!();
    for developer in &developers {
        println!("{}", developer.display_line())
    }
    println!();
    println!("Open to work:");

    for developer in &developers {
        if developer.status.is_open_to_work() {
            println!("{}", developer.display_line())
        }
    }
}
