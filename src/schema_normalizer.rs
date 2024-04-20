use anyhow::Result;
use openapiv3::OpenAPI;
use openapiv3::Operation as OApiOperation;

pub struct SchemaNormalizer {}

#[allow(dead_code)]
fn parse_operation(op: &OApiOperation) -> Result<()> {
    use openapiv3::ParameterSchemaOrContent::Content;
    use openapiv3::ParameterSchemaOrContent::Schema;

    let _ = op.parameters.iter().map(|param| {
        param.as_item().map(|pd| {
            let param_data = pd.parameter_data_ref();
            tracing::info!("param name: {:?}", param_data.name);
            match &param_data.format {
                Content(c) => {
                    tracing::info!("content: {:?}", c);
                },
                Schema(s) => {
                    s.as_item().map(|ss| {
                        tracing::info!("schema: {:?}", ss);
                    });
                },
            }
        });
        ""
    }).collect::<Vec<_>>();
    Ok(())
}

impl SchemaNormalizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn resolve_schema(&self, schema: OpenAPI) -> Result<OpenAPI> {
        // TODO(https://github.com/mify-io/openapic/issues/2): implement schema resolving logic
        // schema.components.map(|c| {
            // build map with all schemas
            // replace all references with items
            // corner case 1: recursive types
        // });
        Ok(schema)
    }
}
