use serde::Serialize;

use super::endpoints::{ENDPOINTS, EndpointDescriptor, ParameterDescriptor};

#[derive(Debug, Serialize)]
struct EndpointSnapshot {
    operation_id: &'static str,
    method: &'static str,
    path: &'static str,
    parameters: Vec<ParameterSnapshot>,
    request_body_refs: Vec<&'static str>,
    response_codes: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct ParameterSnapshot {
    name: &'static str,
    location: &'static str,
    required: bool,
}

fn condense_endpoints() -> Vec<EndpointSnapshot> {
    let mut snapshots: Vec<EndpointSnapshot> = ENDPOINTS.iter().map(condense_endpoint).collect();
    snapshots.sort_by(|left, right| {
        left.path
            .cmp(right.path)
            .then_with(|| left.method.cmp(right.method))
    });
    snapshots
}

fn condense_endpoint(endpoint: &EndpointDescriptor) -> EndpointSnapshot {
    let mut parameters: Vec<ParameterSnapshot> =
        endpoint.parameters.iter().map(condense_parameter).collect();
    parameters.sort_by(|left, right| {
        left.location
            .cmp(right.location)
            .then_with(|| left.name.cmp(right.name))
    });
    let mut response_codes: Vec<&'static str> = endpoint
        .response_refs
        .iter()
        .map(|response| response.status_code)
        .collect();
    response_codes.sort_unstable();

    EndpointSnapshot {
        operation_id: endpoint.operation_id,
        method: endpoint.method.as_str(),
        path: endpoint.path,
        parameters,
        request_body_refs: endpoint.request_body_refs.to_vec(),
        response_codes,
    }
}

fn condense_parameter(parameter: &ParameterDescriptor) -> ParameterSnapshot {
    ParameterSnapshot {
        name: parameter.name,
        location: parameter.location,
        required: parameter.required,
    }
}

#[test]
fn endpoint_metadata_matches_pinned_snapshot() {
    let snapshots = condense_endpoints();
    insta::assert_yaml_snapshot!("endpoint_metadata", snapshots);
}
