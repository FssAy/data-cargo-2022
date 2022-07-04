use std::os::windows::process::CommandExt;


/// Returns an uuid, or if failed creates a random one
pub fn get_uuid() -> String {
    let data: String = lc!("cmd|/C|wmic csproduct get UUID|RUID-|\
        1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM");

    let parts = data.split("|").collect::<Vec<&str>>();

    let get_part = |i: usize| {
        *unsafe {parts.get_unchecked(i)}
    };

    let random_uuid = || {
        format!(
            "{}{}",
            get_part(3),
            random_string::generate(64, get_part(4))
        )
    };

    if let Ok(output) = std::process::Command::new(get_part(0))
        .args(&[get_part(1), get_part(2)])
        .creation_flags(0x08000000)
        .output() {
        let mut output = output.stdout;
        output.drain(4..40);
        output[4] = 0x2D;
        println!("{:?}", output);
        String::from_utf8(output).unwrap_or(random_uuid()).trim().to_string()
    } else {
        random_uuid()
    }
}
