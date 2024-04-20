use anyhow::{Ok, Result};
use template_renderer::TemplateRenderer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use openapic::CompileRequest;
use crate::fsutil::read_input;

mod fsutil;
mod template_renderer;

fn render(req: CompileRequest) -> Result<()> {
    // TODO(https://github.com/mify-io/openapic/issues/3): select renderer based on request type
    let renderer = TemplateRenderer::new();
    renderer.render(req)?;
    Ok(())
}

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::TRACE)
    .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let data = read_input(None)?;
    let req: CompileRequest = serde_json::from_str(&data).expect("Could not deserialize input");
    tracing::info!("data: {:?}", req);

    render(req)?;
    Ok(())
}
