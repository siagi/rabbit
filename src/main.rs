mod models;
mod services;
use models::{CraftMode, Developer, Post, PostKind, PostStatus, Room, Status, Visibility};
use services::{
    open_to_work, published_posts, published_posts_in_room, search_developers, search_posts,
};
fn main() {
    println!("::Start App::Welcome In The Rabbit Hole::");
    let rooms = vec![
        Room {
            slug: String::from("main"),
            name: String::from("Main Burrow"),
            description: String::from("General DDS room"),
        },
        Room {
            slug: String::from("rust-pl"),
            name: String::from("Rust PL"),
            description: String::from("Polish Rust room"),
        },
        Room {
            slug: String::from("ratatui-builders"),
            name: String::from("Ratatui Builders"),
            description: String::from("Terminal UI builders"),
        },
        Room {
            slug: String::from("backend-cave"),
            name: String::from("Backend Cave"),
            description: String::from("Backend, infra and databases"),
        },
    ];
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

    println!();
    println!("Rooms:");
    for room in &rooms {
        println!("{} - {}", room.slug, room.name)
    }

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

    for developer in open_to_work(&developers) {
        println!("{}", developer.display_line());
    }

    println!();
    println!("Latest posts:");
    for post in published_posts(&posts) {
        if post.status.is_visible() {
            println!();
            println!("{}", post.display_title());
            println!("{}", post.summary_line());
            println!("{}", post.body_markdown);
        }
    }

    for developer in search_developers(&developers, "rust") {
        println!("{}", developer.display_line())
    }

    for post in published_posts_in_room(&posts, "rust-pl") {
        println!();
        println!("{}", post.display_title());
        println!("{}", post.summary_line());
    }

    for post in search_posts(&posts, "ownership") {
        println!("{}", post.summary_line());
    }
}
