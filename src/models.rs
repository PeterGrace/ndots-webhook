use serde::{Serialize, Deserialize};
use serde_json::{Map, Value};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AdmissionReview {
    /// meta is the kubernetes metadata for the object
    meta: Map<String, Value>,
    /// request is the incoming AdmissionRequest object
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    request: Option<AdmissionRequest>,
    /// response is the outgoing, mutated object.
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    response: Option<AdmissionResponse>

}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AdmissionRequest {
    /// uid is an identifier for the individual request/response
    uid: UID,
    /// kind is the kind of the object
    kind: GroupVersionKind,
    /// resource is the fully-qualified resource being requested
    resource: GroupVersionResource,
    #[serde(rename = "subResource", skip_serializing_if = "Option::is_none")]
    sub_resource: Option<String>,
    /// request_kind is the fully-qualified type of the original API request
    #[serde(rename = "requestKind", skip_serializing_if = "Option::is_none")]
    request_kind: Option<GroupVersionKind>,
    /// request_sub_resource is the name of the subresource of the original API request
    #[serde(rename = "requestSubResource", skip_serializing_if = "Option::is_none")]
    request_sub_resource: Option<String>,
    /// name is the name of the object as presented in the request.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// namespace is the namespace associated with the request, if any.
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    namespace: Option<String>,
    /// operation is the operation being performed, which may be different than requested.
    /// For example, a PATCH might wind up resulting in a CREATE or UPDATE operation.
    operation: Operation,
    /// user_info is information about the requesting user
    #[serde(rename = "userInfo")]
    user_info: UserInfo,
    /// object is the object from the incoming request.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    object: RawExtension,
    /// old_object is the existing object, only populated during DELETE or UPDATE requests.
    #[serde(rename = "oldObject", skip_serializing_if = "Option::is_none")]
    old_object: RawExtension,
    /// dry_run indicates whether the modifications will be persisted for the object.  default false.
    #[serde(rename = "dryRun", skip_serializing_if = "Option::is_none")]
    dry_run: bool,
    /// options is the operation options structure of the operation being performed.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    options: Option<RawExtension>


}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AdmissionResponse {
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct UID {
}
