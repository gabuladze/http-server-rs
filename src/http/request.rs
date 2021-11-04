use super::method::Method;
struct Request {
    method: Method,
    query_string: Option<String>,
    path: String,
}
