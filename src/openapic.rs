use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use anyhow::Result;
use clap::Parser;
use openapiv3::OpenAPI;

use openapic::CompileRequest;

use crate::schema_normalizer::SchemaNormalizer;

mod schema_normalizer;

#[derive(Parser)]
struct MyArgs {
    #[arg(short, long)]
    my_option: bool,
}

#[derive(Parser)]
#[command(version = "1.0", author = "Your Name")]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,

    /// Output file
    #[arg(short, long)]
    output: String,

    /// Verbose mode
    #[arg(short, long)]
    verbose: bool,
}

pub fn read_file_to_string(file_path: &str) -> Result<String> {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new(file_path);
    let mut file = File::open(&path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}


fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::TRACE)
    .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let args: Args = Args::parse();

    tracing::info!("input file: {}", args.input);
    tracing::info!("output file: {}", args.output);
    let data = read_file_to_string(&args.input)?;
    let openapi: OpenAPI = serde_yaml::from_str(&data).expect("Could not deserialize input");

    let normalizer = SchemaNormalizer::new();

    let schema = normalizer.resolve_schema(openapi)?;
    let req = CompileRequest {
        input_path: args.input.clone(),
        output_path: args.output.clone(),
        is_verbose: args.verbose,
        schema,
    };
    let serialized = serde_json::to_string(&req)?;
    println!("{}", serialized);
    // TODO(https://github.com/mify-io/openapic/issues/1): call backend

    Ok(())
}
