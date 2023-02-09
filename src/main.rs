use clap::Parser;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    dump_schema: Option<String>,
}

fn main() {
    let args = Args::parse();
    if let Some(filename) = args.dump_schema {
        let schema_sdl = api::export_sdl();
        let mut file = std::fs::File::create(&filename).unwrap();
        file.write_all(schema_sdl.as_bytes()).unwrap();
        file.flush().unwrap();
    }

    api::main()
}
