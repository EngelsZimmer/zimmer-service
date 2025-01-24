use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}
