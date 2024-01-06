/// This example demonstrates probably the most complicated part of
/// `minreq`. Useful when making loading bars, for example.
use std::io::Read;

fn main() -> Result<(), minreq::Error> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <URL>", args[0]);
        std::process::exit(1);
    }
    for byte in minreq::get(&args[1]).send_lazy()?.bytes() {
        // The connection could have a problem at any point during the
        // download, so each byte needs to be unwrapped.
        let byte = byte?;

        // The `byte` is the current u8 of data we're iterating
        // through.
        print!("{}", byte as char);

        // Flush the printed text so each char appears on your
        // terminal right away.
        flush();

        // Wait for 50ms so the data doesn't appear instantly fast
        // internet connections, to demonstrate that the body is being
        // printed char-by-char.
        sleep();
    }
    Ok(())
}

// Helper functions

fn flush() {
    use std::io::{stdout, Write};
    stdout().lock().flush().ok();
}

fn sleep() {
    use std::thread::sleep;
    use std::time::Duration;

    sleep(Duration::from_millis(2));
}
