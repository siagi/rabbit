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
}
