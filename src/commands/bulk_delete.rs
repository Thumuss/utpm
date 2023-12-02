use serde_json::json;

use crate::utils::state::{Error, ResponseKind::*, Responses, Result};

use super::{unlink, BulkDeleteArgs, UnlinkArgs};

pub fn run(cmd: &BulkDeleteArgs, res: &mut Responses) -> Result<bool> {
    let mut vec: Vec<Error> = Vec::new();
    for name in &cmd.names {
        let name_and_version = name
            .split(":")
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
        let ulnk = UnlinkArgs {
            delete_namespace: false,
            name: Some(name_and_version[0].to_owned()),
            yes: true,
            namespace: cmd.namespace.to_owned(),
            version: if name_and_version.len() > 1 {
                Some(semver::Version::parse(name_and_version[1].as_str()).unwrap())
            } else {
                None
            },
        };
        match unlink::run(&ulnk, res) {
            Ok(_) => (),
            Err(err) => {
                vec.push(err);
            }
        };
    }
    res.pushs(vec![
        Value(json!({
            "success": cmd.names.len() - vec.len(),
            "failed": vec.len(),
        })),
        Message(format!(
            "{}/{} successful",
            cmd.names.len() - vec.len(),
            cmd.names.len()
        )),
    ]);
    Ok(true)
}
