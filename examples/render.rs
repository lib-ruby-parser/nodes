use lib_ruby_parser_nodes::LiquidTemplate;

fn render(template_src: &str, output_path: &str) {
    let rendered = LiquidTemplate::new(template_src).render();
    std::fs::write(output_path, rendered).unwrap();
}

fn main() {
    render("examples/nodes.liquid", "NODES.md");
    render("examples/messages.liquid", "MESSAGES.md");
}
