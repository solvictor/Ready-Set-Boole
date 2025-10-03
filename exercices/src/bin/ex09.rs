use exercices::eval::eval_set;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        return Err(
            "Usage: cargo run -q --bin ex09 'formula' 's00 s01 s02 ...' 's10 s11 s12 ...' ..."
                .to_string(),
        );
    }

    let formula = &args[0];
    let sets: Vec<Vec<i32>> = args
        .iter()
        .skip(1)
        .map(|set| {
            set.split_whitespace()
                .map(|e| {
                    e.parse()
                        .map_err(|_| format!("Failed to parse '{}' as i32", e))
                })
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()?;

    Ok(println!("{:?}", eval_set(formula, sets)))
}
