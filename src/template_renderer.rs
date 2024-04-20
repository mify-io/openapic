use openapiv3::Type;
use openapiv3::SchemaKind;
use crate::fsutil::write_string_to_file;
use std::io::Write;
use std::collections::HashMap;
use std::fs;
use std::process::{Command, Stdio};

use askama::Template;
use anyhow::{Ok, Result};
use openapiv3::{Components, OpenAPI};


use openapic::CompileRequest;


#[derive(Template)]
#[template(path = "go/model.go.j2", print = "all")]
struct ModelTemplate {
    #[allow(dead_code)]
    schema: OpenAPI,
    models: HashMap<String, Model>,
}

struct Model {
    name: String,
    schema_kind: SchemaKind,
}


fn process_components(components: &Components) -> Result<Vec<Model>> {
    components.schemas.iter().map(|(name, schema)| {
        let item = schema.as_item().ok_or(anyhow::anyhow!("item is not set"))?;
        Ok(Model {
            name: name.clone(),
            schema_kind: item.schema_kind.clone(),
        })
        // build list of models to render
    }).collect::<Result<Vec<_>>>()
}

fn process_models(schema: &OpenAPI) -> Result<HashMap<String, Model>> {
    let models = match &schema.components {
        Some(c) => process_components(&c)?,
        None => vec![],
    };

    Ok(models.into_iter().map(|m| (m.name.clone(), m)).collect::<HashMap<_, _>>())
}


fn format_with_gofmt(input: &str) -> Result<String> {
    let mut gofmt = Command::new("gofmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    gofmt.stdin.as_mut().unwrap().write_all(input.as_bytes())?;

    let output = gofmt.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(anyhow::anyhow!("failed to format: {:?}", output.status.code()))
    }
}

pub struct TemplateRenderer {}

impl TemplateRenderer {
    pub fn new() -> Self { Self {} }

    pub fn render(&self, req: CompileRequest) -> Result<()> {
        let models = process_models(&req.schema)?;
        let tpl = ModelTemplate {
            schema: req.schema,
            models,
        };
        fs::create_dir_all(&req.output_path)?;
        let file_path = vec![req.output_path, "models.go".to_owned()].join("/");
        let res = tpl.render()?;
        let res = format_with_gofmt(&res)?;
        write_string_to_file(&file_path, &res)?;
        tracing::info!("res: {:?}", res);
        Ok(())

    }
}
