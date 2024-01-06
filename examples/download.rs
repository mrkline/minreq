/// This example shows how a [`Response`] is a [`std::io::read`],
/// so it can be easily copied to [`std::io::write`], such as stdout
/// or a file.
fn main() -> Result<(), minreq::Error> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <URL>", args[0]);
        std::process::exit(1);
    }
    let mut response = minreq::get(&args[1]).send_lazy()?;
    std::io::copy(&mut response, &mut std::io::stdout().lock())?;
    Ok(())

}
