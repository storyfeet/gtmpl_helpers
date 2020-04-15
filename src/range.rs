use gtmpl_value::{Number, Value};
pub fn is_list(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("Is list expected 1 elem, got{}", args.len()));
    }
    Ok(Value::Bool(match args[0] {
        Value::Map(_) | Value::Array(_) => true,
        _ => false,
    }))
}

pub fn v_as_list(v: &Value) -> Vec<Value> {
    match v {
        Value::Map(m) => m.values().map(|v| v.clone()).collect(),
        Value::Array(l) => l.iter().map(|v| v.clone()).collect(),
        Value::Nil | Value::NoValue => Vec::new(),
        _ => vec![v.clone()],
    }
}

pub fn as_list(args: &[Value]) -> Result<Value, String> {
    if args.len() == 0 {
        return Err("'as_list' expected at least 1 elem".to_string());
    }
    let mut res = Vec::new();
    for a in args {
        res.extend(v_as_list(a));
    }
    Ok(Value::Array(res))
}

pub fn safe_len(args: &[Value]) -> Result<Value, String> {
    let mut res = 0;
    for a in args {
        res += match a {
            Value::Map(m) => m.len(),
            Value::Array(l) => l.len(),
            Value::String(s) => s.len(),
            _ => 0,
        }
    }
    Ok(Value::Number(Number::from(res)))
}
