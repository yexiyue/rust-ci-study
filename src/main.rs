use clap::Parser;
use parse_json::ProcessJson;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let process_json: ProcessJson = ProcessJson::parse();
    process_json.command.run()?;
    Ok(())
}
