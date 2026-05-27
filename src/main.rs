mod models;
use models::{CraftMode, Developer, Post, PostKind, PostStatus, Status, Visibility};
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

    let posts = vec![
        Post {
            id: String::from("post_1"),
            author_handle: String::from("@michal"),
            room_slug: String::from("ratatui-builders"),
            kind: PostKind::Summary,
            title: Some(String::from("Rabbit Core 01 — Models and first tests")),
            body_markdown: String::from(
                "# Rabbit Core 01\n\n## What we built\n\n- Developer model\n- Status enum\n- CraftMode enum\n\n## Key idea\n\nAI-assisted. Human-owned.",
            ),
            tags: vec![
                String::from("rust"),
                String::from("rabbit"),
                String::from("learning"),
            ],
            visibility: Visibility::Room,
            status: PostStatus::Published,
        },
        Post {
            id: String::from("post_2"),
            author_handle: String::from("@anna_rust"),
            room_slug: String::from("rust-pl"),
            kind: PostKind::Signal,
            title: None,
            body_markdown: String::from(
                "Finally understood why `String` owns text and `&str` borrows it.",
            ),
            tags: vec![String::from("rust"), String::from("ownership")],
            visibility: Visibility::Room,
            status: PostStatus::Published,
        },
        Post {
            id: String::from("post_3"),
            author_handle: String::from("@neo"),
            room_slug: String::from("main"),
            kind: PostKind::Question,
            title: Some(String::from("Can terminal apps feel social?")),
            body_markdown: String::from(
                "What would make a terminal-native dev network feel alive without becoming noisy?",
            ),
            tags: vec![String::from("product"), String::from("terminal")],
            visibility: Visibility::Burrow,
            status: PostStatus::Draft,
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

    println!();
    println!("Latest posts:");
    for post in &posts {
        if post.status.is_visible() {
            println!();
            println!("{}", post.display_title());
            println!("{}", post.summary_line());
            println!("{}", post.body_markdown);
        }
    }
}
