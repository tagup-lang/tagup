mod adapter;
use std::fs;

use parser::parse;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if let Ok(current_dir) = std::env::current_dir() {
        let template_path = current_dir.join(args[1].as_str());
        let adapter_path = current_dir.join(args[2].as_str());
        println!("Template path: {}", template_path.display());
        println!("Adapter path: {}", adapter_path.display());

        if let Ok(adapter) = adapter::Adapter::load(&adapter_path) {
            let source = fs::read_to_string(&template_path).unwrap();
            let program = parse(&source);
            adapter.run(program);
        }
    }
}
