// use std::env::args;

/// todo: add changing options by system args
fn fill_options() {
    use std::fs::*;

    let new = read_to_string("src/handler/options_template.rs")
        .unwrap()
        .replace("/*", "")
        .replace("*/", "")
        .replace("<<TOKEN>>", include_str!("options/token.txt"))
        .replace("<<OUT_CHANNEL_ID>>", include_str!("options/out_channel_id.txt"))
        .replace("<<OWNER>>", include_str!("options/owner.txt"));

    write("src/handler/options.rs", new).unwrap();
}

fn main() {
    fill_options();
    let key = include_str!("options/key.txt");
    println!("cargo:rustc-env=LITCRYPT_ENCRYPT_KEY={}", key);
}
