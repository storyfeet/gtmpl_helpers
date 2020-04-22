use gtmpl_value::Value;
use std::process::Command;

pub fn shell_expand(_args: &Value) -> Result<Value, String> {
    //TODO
    unimplemented! {}
}

pub fn exec(args: &[Value]) -> Result<Value, String> {
    if args.len() == 1 {
        Err("Must have something to exec")?;
    }
    let c = Command::new(args[0].to_string())
        .args(args[1..].into_iter().map(|v| v.to_string()))
        .output()
        .map_err(|e| {
            format!(
                "Could not run program '{}' due to error {}",
                args[0].to_string(),
                e
            )
        })?;

    Ok(Value::String(
        String::from_utf8(c.stdout).map_err(|_| "Could not convert bytes result to string")?,
    ))
}
