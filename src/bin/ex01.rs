use itertools::Itertools as _;
use ready_set_boole::operators::multiplier;

fn main() -> Result<(), String> {
    let args: Vec<u32> = std::env::args()
        .skip(1)
        .map(|arg| {
            arg.parse()
                .map_err(|_| format!("Failed to parse '{}' as u32", arg))
        })
        .collect::<Result<_, _>>()?;

    if args.is_empty() {
        return Err("Usage: cargo run -q --bin ex01 n1 n2 n3 ...".to_string());
    }

    let calculation = args.iter().map(u32::to_string).join(" * ");
    let result_multiplier = args.iter().fold(1, |mul, &n| multiplier(mul, n));
    let result_stdlib = args.iter().product::<u32>();
    println!("{calculation} = {result_multiplier} (multiplier)");
    println!("{calculation} = {result_stdlib} (stdlib)");

    Ok(())
}
