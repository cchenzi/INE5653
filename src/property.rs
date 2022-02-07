use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct InputProperty {
    pub day: String,
    pub month: String,
    pub year: String,
    pub description: String,
    pub value: String,
    pub size: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Property {
    pub id: uuid::Uuid,
    pub value: String,
}
