pub mod choose_target;
pub mod confirm_murder;
pub mod perms;
pub mod react;
pub mod serenity_ext;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
