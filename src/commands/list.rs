use colored::Colorize;
use serde_json::{json, Map, Value};
use std::fs;

use crate::utils::{
    paths::d_packages,
    state::{Responses, Result},
};

pub fn run(res: &mut Responses) -> Result<bool> {
    let typ = d_packages();
    let mut msg = "".to_string();
    msg += &format!("{}\n", "Tree listing of your packages\n".bold());
    let dirs = fs::read_dir(&typ)?;

    let mut data: Vec<Value> = vec![];

    for dir_res in dirs {
        let dir = dir_res?;
        msg += &format!("@{}:\n", dir.file_name().to_str().unwrap().green().bold());
        let subupdirs = fs::read_dir(dir.path())?;

        let mut map = Map::new();
        let mut list: Vec<Value> = vec![];

        for dir_res in subupdirs {
            let dir = dir_res?;

            msg += &format!("  {}:\n", dir.file_name().to_str().unwrap().green().bold());

            let subdirs = fs::read_dir(dir.path())?;

            let mut map2 = Map::new();
            let mut list2: Vec<Value> = vec![];

            for sub_dir_res in subdirs {
                let subdir = sub_dir_res?;
                list2.push(json!(subdir.file_name().to_str()));
                msg += &format!("    - {}\n", subdir.file_name().to_str().unwrap().green());
            }

            let array2 = Value::Array(list2);
            map2.insert(dir.file_name().to_str().unwrap().into(), array2);
            let dir_dataa: Value = Value::Object(map2);

            list.push(dir_dataa)
        }
        let array = Value::Array(list);
        map.insert(dir.file_name().to_str().unwrap().into(), array);
        let dir_data: Value = Value::Object(map);

        data.push(dir_data);
    }
    res.push(json!({
        "message": msg,
        "data": data
    }));
    Ok(true)
}
