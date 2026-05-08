use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.len() != 3 {
        eprintln!(
            "usage: {} <schema-json-path> <generated-output-path>",
            arguments[0]
        );
        eprintln!("  output path may be:");
        eprintln!("    src/generated/types.rs           (typify types)");
        eprintln!("    src/generated/endpoints.rs       (endpoint metadata)");
        eprintln!("    src/generated/resources/         (per-resource modules; trailing slash)");
        return Ok(ExitCode::FAILURE);
    }

    let schema_text = fs::read_to_string(&arguments[1])?;
    let output_path = arguments[2].as_str();

    if output_path.ends_with("types.rs") {
        let generated_source = nbx_codegen::render_typify_types(&schema_text)?;
        fs::write(output_path, generated_source)?;
    } else if output_path.ends_with("endpoints.rs") {
        let openapi_metadata = nbx_codegen::extract_endpoint_metadata(&schema_text)?;
        let target_endpoint_metadata = nbx_codegen::filter_endpoint_metadata(
            &openapi_metadata.endpoints,
            nbx_codegen::V0_1_TARGET_ENDPOINT_PATHS,
        )?;
        let generated_source = nbx_codegen::render_endpoint_descriptors(&target_endpoint_metadata);
        fs::write(output_path, generated_source)?;
    } else if output_path.ends_with('/') || Path::new(output_path).is_dir() {
        let directory = Path::new(output_path);
        fs::create_dir_all(directory)?;
        let outputs = nbx_codegen::render_resource_modules(&schema_text)?;
        for (filename, source) in outputs {
            fs::write(directory.join(filename), source)?;
        }
    } else {
        eprintln!("unrecognized output path: {output_path}");
        return Ok(ExitCode::FAILURE);
    }

    Ok(ExitCode::SUCCESS)
}
