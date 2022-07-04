use super::*;
use std::sync::atomic::{Ordering};
use command_engine::shared::{Help, Instruction, Output};
use crate::handler::IS_SELECTED;
use crate::handler::utils::{markers, UUID};


pub struct Selector;

impl CommandInfo for Selector {
    fn caller(&self) -> &str {
        "s"
    }

    fn on_help(&self, _: Instruction) -> String {
        Help::new(lc!("selector"), lc!("selects the desired victim to operate on"))
            .format_structure()
    }
}

#[async_trait]
impl Command for Selector {
    async fn on_execute(&self, ins: Instruction) -> Output {
        println!("ins: {:?}", ins);

        if let Some(uuid) = ins.get_args().get(0) {
            println!("[{}]", uuid);
            println!("[{}]", UUID.clone());
            if UUID.clone().eq(uuid) {
                println!("uuid match");
                IS_SELECTED.store(true, Ordering::SeqCst);
                return Output::new_ok(1, Some(lc!("success")));
            }
        }

        Output::new_error::<markers::Empty>(1, None)
    }
}
