use exercices::operators::adder;
use itertools::Itertools as _;

fn main() -> Result<(), String> {
    let args: Vec<u32> = std::env::args()
        .skip(1)
        .map(|arg| {
            arg.parse()
                .map_err(|_| format!("Failed to parse '{}' as u32", arg))
        })
        .collect::<Result<_, _>>()?;

    if args.is_empty() {
        return Err("Usage: cargo run -q --bin ex00 n1 n2 n3 ...".to_string());
    }

    let calculation = args.iter().map(u32::to_string).join(" + ");
    let result_adder = args.iter().fold(0, |sum, &n| adder(sum, n));
    let result_stdlib = args.iter().sum::<u32>();
    println!("{calculation} = {result_adder} (adder)");
    println!("{calculation} = {result_stdlib} (stdlib)");

    Ok(())
}
