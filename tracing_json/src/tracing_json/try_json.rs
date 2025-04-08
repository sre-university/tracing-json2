use serde::Serialize;

/// used to debug print json
///
/// ```
/// #[instrument(level = "info", name = "PubSub:publish", skip_all, fields(some_struct=?TryJson(&some_struct)))]
/// fn some_func(some_struct: SomeStruct) {}
///
/// let span = span!(Level::TRACE, "foo_span", some_struct=?TryJson(&some_struct));
///
/// info!(some_struct = ?TryJson(&resp), "handle webhook");
/// ```
pub struct TryJson<'a, T: Serialize>(pub &'a T);

impl<T> std::fmt::Debug for TryJson<'_, T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let json = serde_json::to_string(self.0);
        match json {
            Ok(json) => write!(f, "{}", json),
            Err(e) => write!(f, "error serializing to json: {}", e),
        }
    }
}
