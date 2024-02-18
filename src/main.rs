use std::io::Write;

use carbon1dot1_assembler::{
    assembler::assemble, lower_labels::lower_labels, name_mangling::mangle, parser::parse,
};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap()]
    input_file: String,
    #[clap(short, long, default_value_t=String::from("./out.bin"))]
    output_file: String,
}

fn main() {
    let args = Args::parse();
    let text = std::fs::read_to_string(&args.input_file).expect("failed to read the input file");
    let ast = parse(&text, &args.input_file);
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    // println!(
    //     "{}",
    //     lowered
    //         .iter()
    //         .map(|e| format!("{e}"))
    //         .collect::<Vec<String>>()
    //         .join("\n")
    // );
    let assembled = assemble(lowered);
    std::fs::File::create(args.output_file)
        .and_then(|mut f| f.write_all(&assembled))
        .expect("Failed to write to the output file.");
}
