use command_engine::shared::{Instruction, Output};
use super::*;

pub struct Test;

impl CommandInfo for Test {
    fn caller(&self) -> &str {
        "test"
    }
}

#[async_trait]
impl Command for Test {
    async fn on_execute(&self, _: Instruction) -> Output {
        Output::new_ok(1, Some("hello world"))
    }
}
