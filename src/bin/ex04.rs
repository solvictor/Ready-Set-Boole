use ready_set_boole::truth_table::print_truth_table;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.len() {
        1 => Ok(print_truth_table(&args[0])),
        _ => Err("Usage: cargo run -q --bin ex04 'formula'"),
    }
}
