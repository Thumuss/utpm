use serde_json::json;

use crate::utils::{
    paths::d_packages,
    state::{ResponseKind::*, Responses, Result},
};

pub fn run(res: &mut Responses) -> Result<bool> {
    res.pushs(vec![
        Value(json!({
            "path": d_packages(),
        })),
        Message(format!("Packages are located at: '{}'", d_packages())),
    ]);
    Ok(true)
}
