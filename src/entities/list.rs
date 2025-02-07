use serde::Deserialize;
/// Used for ser/de of list resources
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct List {
    id: String,
    title: String,
}
