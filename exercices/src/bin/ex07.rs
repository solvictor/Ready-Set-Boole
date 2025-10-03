use exercices::sat::sat;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.len() {
        1 => Ok(println!(
            "The formula '{}' {} satisfiable.",
            &args[0],
            if sat(&args[0]) { "is" } else { "is not" }
        )),
        _ => Err("Usage: cargo run -q --bin ex07 'formula'"),
    }
}
