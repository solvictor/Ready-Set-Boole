use exercices::gray_code::gray_code;

fn main() -> Result<(), String> {
    let args: Vec<u32> = std::env::args()
        .skip(1)
        .map(|arg| {
            arg.parse()
                .map_err(|_| format!("Failed to parse '{}' as u32", arg))
        })
        .collect::<Result<_, _>>()?;

    if args.is_empty() {
        return Err("Usage: cargo run -q --bin ex02 n1 n2 n3 ...".to_string());
    }

    for n in args {
        println!("{:08b}", gray_code(n));
    }

    Ok(())
}
