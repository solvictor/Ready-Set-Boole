use exercices::eval::eval_formula;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.len() {
        1 => Ok(println!("{}", eval_formula(&args[0]))),
        _ => Err("Usage: cargo run -q --bin ex03 'formula'"),
    }
}
