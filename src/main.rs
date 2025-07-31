use std::io::Write;

use carbon1dot1_assembler::{
    assembler::assemble, disassembler::disassemble_from_file, lower_labels::lower_labels, name_mangling::mangle, parser::parse,
};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap()]
    input_file: String,
    #[clap(short, long, default_value_t=String::from("./out.bin"))]
    output_file: String,
    #[clap(short, long)]
    disassemble: bool,
}

fn main() {
    let args = Args::parse();
    
    if args.disassemble {
        // Disassemble mode: input_file should be a binary file
        match disassemble_from_file(&args.input_file) {
            Ok(disassembly) => {
                for line in disassembly {
                    println!("{}", line);
                }
            }
            Err(e) => {
                eprintln!("Error disassembling file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Assembly mode: normal operation
        let text = std::fs::read_to_string(&args.input_file).expect("failed to read the input file");
        let ast = parse(&text, &args.input_file);
        let mangled = mangle(ast);
        //println!(
        //    "{}",
        //    mangled
        //        .iter()
        //        .map(|e| format!("{e}"))
        //        .collect::<Vec<String>>()
        //        .join("\n")
        //);
        let lowered = lower_labels(mangled);
        //println!(
        //    "{}",
        //    lowered
        //        .iter()
        //        .map(|e| format!("{e}"))
        //        .collect::<Vec<String>>()
        //        .join("\n")
        //);
        let assembled = assemble(lowered);
        let mut f = std::fs::File::create(args.output_file).expect("Failed to open  the output file.");
        f.write_all(
            &assembled
                .iter()
                .map(|e| format!("{:08b}", e))
                .collect::<Vec<String>>()
                .join("\n")
                .as_bytes(),
        )
        .unwrap();
        //romgen::generate_schem(&mut f, &assembled, 256).unwrap();
    }
}
