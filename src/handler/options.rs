use std::sync::Arc;

pub const OUT_CHANNEL_ID: u64 = 000000000000000000;

pub const OWNER: u64 = 000000000000000000;

lazy_static! {
    pub static ref TOKEN: Arc<String> = Arc::new(lc!("xxxxxxxxxxxxxxxxxxxxxxxx.yyyyyy.zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz"));
}
