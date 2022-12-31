use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long, default_value_t = {"data".to_string()})]
    name: String,
    files: Vec<String>,
}

fn convert_file(input: &str, name: &str) -> String {
    format!(
        "const char {name}[]=\"{}\";\n",
        &input
            .replace('\\', "\\\\")
            .replace('\n', "\\n")
            .replace('"', "\\\"")
    )
}

fn main() {
    let args = Args::parse();

    for file in args.files {
        std::fs::write(
            format!("{file}.h"),
            convert_file(
                std::str::from_utf8(&std::fs::read(&file).unwrap()).unwrap(),
                &args.name,
            ),
        )
        .unwrap();
    }
}
