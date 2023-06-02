pub trait Resumable {
    fn resum(&self) -> String {
        "Resuming with no templat".to_string()
    }
}

pub struct PressArticle {
    pub author: String,
    pub place: String,
    pub title: String,
    pub content: String,
}

impl Resumable for PressArticle {
    fn resum(&self) -> String {
        format!(
            "{} a été écrit par {} à {}",
            self.title, self.author, self.place
        )
    }
}

pub struct Tweet {
    pub user: String,
    pub content: String,
    pub is_retweet: bool,
    pub is_response: bool,
}

impl Resumable for Tweet {
    fn resum(&self) -> String {
        format!("{}: {}", self.user, self.content)
    }
}
