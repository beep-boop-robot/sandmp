use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Particle {
    Air,
    Sand,
    Boundary
}