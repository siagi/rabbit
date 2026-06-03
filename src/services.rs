use crate::models::{Developer, Ping, PingStatus, Post, Room};

pub fn search_developers<'a>(developerts: &'a [Developer], query: &str) -> Vec<&'a Developer> {
    let query = query.to_lowercase();
    developerts
        .iter()
        .filter(|developer| {
            developer.handle.to_lowercase().contains(&query)
                || developer.location.to_lowercase().contains(&query)
                || developer
                    .stack
                    .iter()
                    .any(|technology| technology.to_lowercase().contains(&query))
                || developer.status.label().contains(&query)
                || developer.craft_mode.label().contains(&query)
        })
        .collect()
}

pub fn open_to_work<'a>(developers: &'a [Developer]) -> Vec<&'a Developer> {
    developers
        .iter()
        .filter(|developer| developer.status.is_open_to_work())
        .collect()
}

pub fn published_posts<'a>(posts: &'a [Post]) -> Vec<&'a Post> {
    posts
        .iter()
        .filter(|post| post.status.is_visible())
        .collect()
}

pub fn posts_in_room<'a>(posts: &'a [Post], room_slug: &str) -> Vec<&'a Post> {
    posts
        .iter()
        .filter(|post| post.room_slug == room_slug)
        .collect()
}

pub fn published_posts_in_room<'a>(posts: &'a [Post], room_slug: &str) -> Vec<&'a Post> {
    posts
        .iter()
        .filter(|post| post.room_slug == room_slug && post.status.is_visible())
        .collect()
}

pub fn find_room_by_slug<'a>(rooms: &'a [Room], slug: &str) -> Option<&'a Room> {
    rooms.iter().find(|room| room.slug == slug)
}

pub fn search_posts<'a>(posts: &'a [Post], query: &str) -> Vec<&'a Post> {
    let query = query.to_lowercase();
    posts
        .iter()
        .filter(|post| {
            post.display_title().to_lowercase().contains(&query)
                || post.body_markdown.to_lowercase().contains(&query)
                || post
                    .tags
                    .iter()
                    .any(|tag| tag.to_lowercase().contains(&query))
                || post.author_handle.to_lowercase().contains(&query)
                || post.room_slug.to_lowercase().contains(&query)
        })
        .collect()
}

pub fn create_ping(
    id: &str,
    from_handle: &str,
    to_handle: &str,
    message: &str,
    created_at: &str,
) -> Ping {
    Ping {
        id: id.to_string(),
        from_handle: from_handle.to_string(),
        to_handle: to_handle.to_string(),
        message: message.to_string(),
        status: PingStatus::Pending,
        created_at: created_at.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        CraftMode, Developer, Post, PostKind, PostStatus, Room, Status, Visibility,
    };

    fn sample_developers() -> Vec<Developer> {
        vec![
            Developer {
                handle: String::from("@anna_rust"),
                stack: vec![String::from("Rust"), String::from("Backend")],
                location: String::from("Remote"),
                status: Status::Available,
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
                stack: vec![String::from("Rust"), String::from("Systems")],
                location: String::from("Zion"),
                status: Status::Lurking,
                craft_mode: CraftMode::LearningMode,
            },
        ]
    }

    fn sample_posts() -> Vec<Post> {
        vec![
            Post {
                id: String::from("post_1"),
                author_handle: String::from("@michal"),
                room_slug: String::from("rust-pl"),
                kind: PostKind::Summary,
                title: Some(String::from("Rust ownership notes")),
                body_markdown: String::from("# Ownership"),
                tags: vec![String::from("rust")],
                visibility: Visibility::Room,
                status: PostStatus::Published,
            },
            Post {
                id: String::from("post_2"),
                author_handle: String::from("@anna_rust"),
                room_slug: String::from("rust-pl"),
                kind: PostKind::Signal,
                title: None,
                body_markdown: String::from("I finally understood lifetimes."),
                tags: vec![String::from("rust"), String::from("lifetimes")],
                visibility: Visibility::Room,
                status: PostStatus::Draft,
            },
            Post {
                id: String::from("post_3"),
                author_handle: String::from("@kubadev"),
                room_slug: String::from("backend-cave"),
                kind: PostKind::BuildLog,
                title: Some(String::from("Postgres debugging")),
                body_markdown: String::from("Indexes matter."),
                tags: vec![String::from("postgres")],
                visibility: Visibility::Room,
                status: PostStatus::Published,
            },
        ]
    }

    #[test]
    fn search_developers_finds_by_stack() {
        let developers = sample_developers();

        let results = search_developers(&developers, "rust");

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn search_developers_finds_by_location() {
        let developers = sample_developers();

        let results = search_developers(&developers, "kraków");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].handle, "@kubadev");
    }

    #[test]
    fn open_to_work_excludes_busy_developers() {
        let developers = sample_developers();

        let results = open_to_work(&developers);

        assert_eq!(results.len(), 2);
        assert!(
            results
                .iter()
                .all(|developer| developer.status.is_open_to_work())
        );
    }

    #[test]
    fn published_posts_excludes_drafts() {
        let posts = sample_posts();

        let results = published_posts(&posts);

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|post| post.status.is_visible()));
    }

    #[test]
    fn posts_in_room_returns_all_posts_from_room() {
        let posts = sample_posts();

        let results = posts_in_room(&posts, "rust-pl");

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn published_posts_in_room_excludes_drafts() {
        let posts = sample_posts();

        let results = published_posts_in_room(&posts, "rust-pl");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "post_1");
    }

    #[test]
    fn find_room_by_slug_returns_room() {
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
        ];

        let room = find_room_by_slug(&rooms, "rust-pl");

        assert!(room.is_some());
        assert_eq!(room.unwrap().name, "Rust PL");
    }

    #[test]
    fn find_room_by_slug_returns_none_for_missing_room() {
        let rooms = vec![Room {
            slug: String::from("main"),
            name: String::from("Main Burrow"),
            description: String::from("General DDS room"),
        }];

        let room = find_room_by_slug(&rooms, "missing-room");

        assert!(room.is_none());
    }

    #[test]
    fn search_posts_finds_by_tag() {
        let posts = sample_posts();

        let results = search_posts(&posts, "postgres");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "post_3");
    }
}
