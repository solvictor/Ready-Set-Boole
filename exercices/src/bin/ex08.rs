use exercices::sets::powerset;

fn main() -> Result<(), String> {
    let args: Vec<i32> = std::env::args()
        .skip(1)
        .map(|arg| {
            arg.parse()
                .map_err(|_| format!("Failed to parse '{}' as i32", arg))
        })
        .collect::<Result<_, _>>()?;

    for subset in powerset(args) {
        println!("{:?}", subset);
    }

    Ok(())
}
