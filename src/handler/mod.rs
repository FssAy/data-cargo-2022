pub mod options;
pub mod utils;
mod handler;

pub use handler::Handler;

use std::sync::atomic::AtomicBool;
use options::*;
use serenity::async_trait;


use serenity::model::prelude::*;
use crate::handler::utils::*;
use super::*;

lazy_static! {
    /// Marks if this target is selected or not
    pub static ref IS_SELECTED: AtomicBool = AtomicBool::new(false);

    /// Holds a list of users who can call commands
    pub static ref ADMINS: RwLock<Vec<u64>> = RwLock::new(vec![OWNER]);
}
