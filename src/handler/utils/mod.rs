mod uuid;
pub mod engine;
pub mod markers;



lazy_static! {
    pub static ref UUID: String = uuid::get_uuid();

    pub static ref ENGINE: command_engine::Engine = engine::build_engine();

    pub static ref U2800: String = unsafe {
        String::from_utf8_unchecked(vec![0xE2, 0xA0, 0x80])
    };
}
