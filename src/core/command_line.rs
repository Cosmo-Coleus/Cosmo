
/// Réprésente la ligne de commande utilisable dans le mode [`InputMode::Command`](type@InputMode::Command)
pub struct CommandLine {
    pub buffer: String,
}

impl CommandLine {
    pub const fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }
}
