use vtio::event::shell::CommandEnd;
use vtio_control_base::AnsiEncode;

fn main() {
    let mut buf = Vec::new();

    // Test CommandEnd with exit code
    let mut cmd = CommandEnd { exit_code: Some(42) };
    cmd.encode_ansi_into(&mut buf).unwrap();
    let result = String::from_utf8(buf.clone()).unwrap();
    println!("CommandEnd with exit_code: {:?}", result);

    buf.clear();

    // Test CommandEnd without exit code
    let mut cmd = CommandEnd { exit_code: None };
    cmd.encode_ansi_into(&mut buf).unwrap();
    let result = String::from_utf8(buf.clone()).unwrap();
    println!("CommandEnd without exit_code: {:?}", result);
}
