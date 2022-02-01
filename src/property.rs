use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Property {
    pub created_at: String,
    pub description: String,
    pub value: String,
    pub size: u16,
}
