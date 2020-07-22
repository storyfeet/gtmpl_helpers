use gtmpl_value::Value;

use std::path::{Path, PathBuf};

fn path_gstr(p: &Path) -> Value {
    Value::String(p.display().to_string())
}

fn join_args(args: &[Value]) -> PathBuf {
    let mut path = PathBuf::new();
    for v in args {
        path.push(v.to_string());
    }
    path
}

pub fn parent(args: &[Value]) -> Result<Value, String> {
    if let Some(p) = join_args(args).parent() {
        Ok(Value::String(p.display().to_string()))
    } else {
        Ok(Value::String("".to_string()))
    }
}

pub fn join(args: &[Value]) -> Result<Value, String> {
    Ok(path_gstr(&join_args(args)))
}

pub fn bread_crumbs(args: &[Value]) -> Result<Value, String> {
    let mut path = join_args(args);
    let mut res = Vec::new();
    res.push(path_gstr(&path));
    while let Some(par) = path.parent() {
        res.insert(0, path_gstr(&par));
        path = PathBuf::from(par);
    }
    Ok(Value::Array(res))
}

pub fn base_name(args: &[Value]) -> Result<Value, String> {
    join_args(args)
        .file_name()
        .and_then(|v| v.to_str())
        .map(|v| Value::String(v.to_string()))
        .ok_or("Could not get file base_name".to_string())
}
pub fn base_name_sure(args: &[Value]) -> Result<Value, String> {
    Ok(Value::String(
        join_args(args)
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("")
            .to_string(),
    ))
}
