use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Schwema {
    when: String,
    i: String,
}
