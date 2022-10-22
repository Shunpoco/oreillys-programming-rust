use std::io::Write;
use std::fs::File;

// Trait!
fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n").unwrap();
    out.flush()
}

fn exec_sayhello()-> std::io::Result<()> {
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    let mut bytes = vec![];
    say_hello(&mut bytes)?;
    assert_eq!(bytes, b"hello world\n");

    Ok(())
}

fn main() {
    exec_sayhello();
}
