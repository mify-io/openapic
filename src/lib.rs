use openapiv3::OpenAPI;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompileRequest {
    pub input_path: String,
    pub output_path: String,
    pub is_verbose: bool,
    pub schema: OpenAPI
}
