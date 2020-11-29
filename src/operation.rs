use serde::{Deserialize, Serialize};
#[serde(rename_all(deserialize = "lowercase"))]
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub enum Operation {
    Set,
    Insert,
    Delete,
    Remove,
    Update,
    Put,
    Get,
    Read,
    Unknown,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        match s {
            "get" => Operation::Get,
            "read" => Operation::Read,
            "set" => Operation::Set,
            "insert" => Operation::Insert,
            "update" => Operation::Update,
            "put" => Operation::Put,
            "delete" => Operation::Delete,
            "remove" => Operation::Remove,
            _ => Operation::Unknown,
        }
    }
}

mod tests {
    #[test]
    fn test_operation_from() {
        use crate::Operation;
        assert_eq!(Operation::Get, Operation::from("get"));
        assert_eq!(Operation::Read, Operation::from("read"));
        assert_eq!(Operation::Set, Operation::from("set"));
        assert_eq!(Operation::Insert, Operation::from("insert"));
        assert_eq!(Operation::Update, Operation::from("update"));
        assert_eq!(Operation::Put, Operation::from("put"));
        assert_eq!(Operation::Delete, Operation::from("delete"));
        assert_eq!(Operation::Remove, Operation::from("remove"));
        assert_eq!(Operation::Unknown, Operation::from("unknown"));
        assert_eq!(Operation::Unknown, Operation::from("blah"));
    }
}
