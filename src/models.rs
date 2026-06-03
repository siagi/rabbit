#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Available = 0,
    SideQuests = 1,
    Lurking = 2,
    Busy = 3,
    Stale = 4,
}

impl Status {
    pub fn icon(&self) -> &'static str {
        match self {
            Status::Available => "●",
            Status::SideQuests => "◐",
            Status::Lurking => "○",
            Status::Busy => "×",
            Status::Stale => "☠",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Status::Available => "available",
            Status::SideQuests => "open to side quests",
            Status::Lurking => "lurking",
            Status::Busy => "busy",
            Status::Stale => "stale",
        }
    }

    pub fn is_open_to_work(&self) -> bool {
        match self {
            Status::Available | Status::SideQuests | Status::Lurking => true,
            Status::Busy | Status::Stale => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CraftMode {
    HumanOwnedAiAssisted,
    ManualFirst,
    LearningMode,
}

impl CraftMode {
    pub fn label(&self) -> &'static str {
        match self {
            CraftMode::HumanOwnedAiAssisted => "human-owned / AI-assisted",
            CraftMode::ManualFirst => "manual-first",
            CraftMode::LearningMode => "learning-mode",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Developer {
    pub handle: String,
    pub stack: Vec<String>,
    pub location: String,
    pub status: Status,
    pub craft_mode: CraftMode,
}

impl Developer {
    pub fn stack_label(&self) -> String {
        self.stack.join(" / ")
    }

    pub fn display_line(&self) -> String {
        format!(
            "{} {} ({}) — {} — {} — {}",
            self.status.icon(),
            self.handle,
            self.status.label(),
            self.stack_label(),
            self.location,
            self.craft_mode.label()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostKind {
    Signal,
    BuildLog,
    Question,
    Summary,
    Resource,
}
impl PostKind {
    pub fn label(&self) -> &'static str {
        match self {
            PostKind::Signal => "signal",
            PostKind::BuildLog => "build log",
            PostKind::Question => "question",
            PostKind::Summary => "summary",
            PostKind::Resource => "resource",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    Private,
    Room,
    Burrow,
    Public,
}
impl Visibility {
    pub fn label(&self) -> &'static str {
        match self {
            Visibility::Private => "private",
            Visibility::Room => "room",
            Visibility::Burrow => "burrow",
            Visibility::Public => "public",
        }
    }
}
//TOASK: Why here is PartialEq and Eq also why Debug and Clone  ?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostStatus {
    Draft,
    Published,
    Archived,
}
impl PostStatus {
    pub fn label(&self) -> &'static str {
        match self {
            PostStatus::Draft => "draft",
            PostStatus::Published => "published",
            PostStatus::Archived => "archived",
        }
    }
    pub fn is_visible(&self) -> bool {
        match self {
            PostStatus::Published => true,
            PostStatus::Draft | PostStatus::Archived => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Post {
    pub id: String,
    pub author_handle: String,
    pub room_slug: String,
    pub kind: PostKind,
    pub title: Option<String>,
    pub body_markdown: String,
    pub tags: Vec<String>,
    pub visibility: Visibility,
    pub status: PostStatus,
}
//TOASK: why here we have self with "&" and above without in match ?
impl Post {
    pub fn display_title(&self) -> String {
        match &self.title {
            Some(title) => title.clone(),
            None => String::from("(untitled)"),
        }
    }
    pub fn tags_label(&self) -> String {
        if self.tags.is_empty() {
            String::from("no tags")
        } else {
            self.tags.join(", ")
        }
    }

    pub fn summary_line(&self) -> String {
        format!(
            "{} · {} · {} · {}",
            self.author_handle,
            self.kind.label(),
            self.room_slug,
            self.tags_label()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PingStatus {
    Pending,
    Accepted,
    Declined,
    Delivered,
}

impl PingStatus {
    pub fn label(&self) -> &'static str {
        match self {
            PingStatus::Pending => "pending",
            PingStatus::Accepted => "accepted",
            PingStatus::Declined => "declined",
            PingStatus::Delivered => "delivered",
        }
    }
    pub fn is_open(&self) -> bool {
        match self {
            PingStatus::Pending | PingStatus::Delivered => true,
            PingStatus::Accepted | PingStatus::Declined => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ping {
    pub id: String,
    pub from_handle: String,
    pub to_handle: String,
    pub message: String,
    pub status: PingStatus,
    pub created_at: String,
}

impl Ping {
    pub fn summary_line(&self) -> String {
        format! {"{} · {} → {} · {}", self.id, self.from_handle, self.to_handle, self.status.label()}
    }

    pub fn display(&self) -> String {
        format! {"{}\n{}\n\n{}",self.summary_line(), self.created_at, self.message }
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    pub slug: String,
    pub name: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn available_status_is_open_to_work() {
        assert!(Status::Available.is_open_to_work());
    }
    #[test]
    fn side_quests_status_is_open_to_work() {
        assert!(Status::SideQuests.is_open_to_work());
    }

    #[test]
    fn busy_status_is_not_open_to_work() {
        assert!(!Status::Busy.is_open_to_work());
    }

    #[test]
    fn stale_status_is_not_open_to_work() {
        assert!(!Status::Stale.is_open_to_work());
    }

    #[test]
    fn craft_mode_has_human_owned_label() {
        assert_eq!(
            CraftMode::HumanOwnedAiAssisted.label(),
            "human-owned / AI-assisted"
        );
    }
    #[test]
    fn status_has_correct_labels() {
        assert_eq!(Status::Stale.label(), "stale");
    }
    #[test]
    fn developer_stack_label_join_stack() {
        let developer = Developer {
            handle: String::from("@michal"),
            stack: vec![
                String::from("Rust"),
                String::from("Ratatui"),
                String::from("AI tools"),
            ],
            location: String::from("Remote"),
            status: Status::SideQuests,
            craft_mode: CraftMode::HumanOwnedAiAssisted,
        };
        assert_eq!(developer.stack_label(), "Rust / Ratatui / AI tools");
    }
    #[test]
    fn lurkin_status_is_open_to_work() {
        assert!(Status::Lurking.is_open_to_work())
    }
    #[test]
    fn draft_post_is_not_visible() {
        assert!(!PostStatus::Draft.is_visible())
    }
    #[test]
    fn published_post_is_visible() {
        assert!(PostStatus::Published.is_visible())
    }
    #[test]
    fn post_without_title_has_fallback_title() {
        let post = Post {
            id: String::from("post_1"),
            author_handle: String::from("@michal"),
            room_slug: String::from("ratatui-builders"),
            kind: PostKind::Signal,
            title: None,
            body_markdown: String::from("AI-assisted. Human-owned."),
            tags: vec![String::from("ai"), String::from("craft")],
            visibility: Visibility::Room,
            status: PostStatus::Published,
        };
        assert_eq!(post.display_title(), "(untitled)")
    }
    #[test]
    fn post_tags_are_joined() {
        let post = Post {
            id: String::from("post_1"),
            author_handle: String::from("@michal"),
            room_slug: String::from("ratatui-builders"),
            kind: PostKind::Summary,
            title: Some(String::from("Rabbit Core 01")),
            body_markdown: String::from("# Rabbit Core 01"),
            tags: vec![
                String::from("rust"),
                String::from("rabbit"),
                String::from("learning"),
            ],
            visibility: Visibility::Room,
            status: PostStatus::Published,
        };
        assert_eq!(post.tags_label(), "rust, rabbit, learning");
    }

    #[test]
    fn pending_ping_is_open() {
        assert!(PingStatus::Pending.is_open());
    }

    #[test]
    fn delivered_ping_is_open() {
        assert!(PingStatus::Delivered.is_open());
    }

    #[test]
    fn accepted_ping_is_not_open() {
        assert!(!PingStatus::Accepted.is_open());
    }

    #[test]
    fn declined_ping_is_not_open() {
        assert!(!PingStatus::Declined.is_open());
    }

    #[test]
    fn ping_summary_line_contains_sender_and_receiver() {
        let ping = Ping {
            id: String::from("ping_1"),
            from_handle: String::from("@michal"),
            to_handle: String::from("@anna_rust"),
            message: String::from("Hej, quest?"),
            status: PingStatus::Pending,
            created_at: String::from("2026-05-31T12:00:00Z"),
        };

        assert!(ping.summary_line().contains("@michal"));
        assert!(ping.summary_line().contains("@anna_rust"));
    }
}
