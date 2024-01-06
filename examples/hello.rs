/// This is a simple example to demonstrate the usage of this library.
fn main() -> Result<(), minreq::Error> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <URL>", args[0]);
        std::process::exit(1);
    }
    let response = minreq::get(&args[1]).send()?;
    let html = response.as_str()?;
    println!("{}", html);
    Ok(())
}
