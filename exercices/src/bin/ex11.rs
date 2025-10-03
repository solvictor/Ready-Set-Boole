use exercices::curve::reverse_map;

fn main() -> Result<(), String> {
    let args: Vec<f64> = std::env::args()
        .skip(1)
        .map(|arg| {
            arg.parse()
                .map_err(|_| format!("Failed to parse '{}' as f64", arg))
        })
        .collect::<Result<_, _>>()?;

    match args.len() {
        1 => {
            let x = args[0];
            let (y, z) = reverse_map(x);
            Ok(println!("{} => ({}, {})", x, y, z))
        }
        _ => Err("Usage: cargo run -q --bin ex11 x".to_string()),
    }
}
