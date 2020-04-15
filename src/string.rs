use gtmpl_value::Value;

pub fn ccat(args: &[Value]) -> Result<Value, String> {
    let mut res = String::new();
    for v in args {
        res.push_str(&v.to_string());
    }
    Ok(Value::String(res))
}

fn _sep(sp: &str, args: &[Value]) -> Result<String, String> {
    let mut first = true;
    let mut res = String::new();
    for a in args {
        if !first {
            res.push_str(sp);
        }
        match a {
            Value::Array(l) => res.push_str(&_sep(sp, l)?),
            v => res.push_str(&v.to_string()),
        }

        first = false;
    }
    Ok(res)
}

/// first arg is seperator, after that flatten lists and add seperated by the separator
pub fn sep(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Nothing to separate".to_string());
    }
    let sp = args[0].to_string();
    Ok(Value::String(_sep(&sp, &args[1..])?))
}
