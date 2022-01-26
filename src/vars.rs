// [[file:../spdkit.note::005016d8][005016d8]]
use super::*;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Vars {
    pub random_seed: Option<u64>,
}

impl Vars {
    /// Construct `Dimer` object from environment variables
    pub fn from_env() -> Self {
        match envy::prefixed("SPDKIT_").from_env::<Self>() {
            Ok(vars) => vars,
            Err(error) => {
                panic!("invalid env vars: {error:?}");
            }
        }
    }
}
// 005016d8 ends here
