fn print_usage_and_exit() -> ! {
    eprintln!("Usage: codegen --template <template.liquid> --write-to <outfile>");
    std::process::exit(1);
}

fn try_arg(key: &str) -> Option<String> {
    let idx = std::env::args().position(|e| e == key)?;
    let value = std::env::args().nth(idx + 1)?;
    Some(value)
}

fn get_arg(key: &str) -> String {
    try_arg(key).unwrap_or_else(|| {
        eprintln!("No {} CLI option given", key);
        print_usage_and_exit()
    })
}

fn main() {
    let template_path = get_arg("--template");
    let output_path = get_arg("--write-to");

    let rendered = lib_ruby_parser_nodes::LiquidTemplate::new(template_path).render();
    std::fs::write(output_path, rendered).unwrap();
}
