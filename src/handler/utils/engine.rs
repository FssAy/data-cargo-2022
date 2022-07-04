pub mod selector;
pub mod test;



use command_engine::*;



pub fn build_engine() -> Engine {
    let engine = EngineBuilder::new()
        .buffer(32)
        .help_caller("help")
        .build()
        .unwrap();

    let engine_th = engine.clone();
    tokio::task::block_in_place(|| {
        /* Adding new commands */
        let _ = engine_th.add_blocking(selector::Selector);
        let _ = engine_th.add_blocking(test::Test);
    });

    engine
}
