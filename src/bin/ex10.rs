use ready_set_boole::curve::map;

fn main() -> Result<(), String> {
    let args: Vec<u16> = std::env::args()
        .skip(1)
        .map(|arg| {
            arg.parse()
                .map_err(|_| format!("Failed to parse '{}' as u16", arg))
        })
        .collect::<Result<_, _>>()?;

    match args.len() {
        2 => {
            let x = args[0];
            let y = args[1];
            Ok(println!("({}, {}) => {}", x, y, map(x, y)))
        }
        _ => Err("Usage: cargo run -q --bin ex10 x y".to_string()),
    }
}
