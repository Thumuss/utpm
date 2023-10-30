use serde_json::json;

use crate::utils::{
    paths::d_packages,
    state::{Responses, Result},
};

pub fn run(res: &mut Responses) -> Result<bool> {
    res.push(json!({
        "path": d_packages(),
        "message": format!("Packages are located at: '{}'", d_packages()),
    }));
    Ok(true)
}
