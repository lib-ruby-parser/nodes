fn print_usage_and_exit() -> ! {
    eprintln!("Usage: codegen --template <template.liquid> --write-to <outfile>");
    std::process::exit(1);
}

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();

    let mut get_arg = |key: &str| {
        let key_idx = args
            .iter()
            .enumerate()
            .find(|&(_idx, e)| e == key)
            .unwrap_or_else(|| {
                eprintln!("Unable to get {} CLI argument", key);
                print_usage_and_exit()
            })
            .0;
        let _key = args.remove(key_idx);
        if key_idx >= args.len() {
            eprintln!("No {} CLI option given", key);
            print_usage_and_exit();
        }
        let value = args.remove(key_idx);
        value
    };

    let template_path = get_arg("--template");
    let output_path = get_arg("--write-to");

    let rendered = lib_ruby_parser_nodes::LiquidTemplate::new(template_path).render();
    std::fs::write(output_path, rendered).unwrap();
}
