use ready_set_boole::normal_forms::conjunctive_normal_form;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.len() {
        1 => Ok(println!("{}", conjunctive_normal_form(&args[0]))),
        _ => Err("Usage: cargo run -q --bin ex06 'formula'"),
    }
}
