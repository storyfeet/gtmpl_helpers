use gtmpl_value::{Number, Value};

pub fn has(args: &[Value]) -> Result<Value, String> {
    Ok(Value::Bool(match args.get(0) {
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => (*n) >= Number::from(0),
        Some(Value::NoValue) | Some(Value::Nil) => false,
        Some(Value::String(s)) => s.len() != 0,
        Some(Value::Array(a)) => a.len() != 0,
        Some(Value::Map(m)) => m.len() != 0,
        _ => return Err("First Expr must be boolable".to_string()),
    }))
}

pub fn b_sel(args: &[Value]) -> Result<Value, String> {
    let b_val = match args.get(0) {
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => (*n) >= Number::from(0),
        Some(Value::NoValue) | Some(Value::Nil) => false,
        Some(Value::String(s)) => s.len() != 0,
        Some(Value::Array(a)) => a.len() != 0,
        Some(Value::Map(m)) => m.len() != 0,
        _ => return Err("First Expr must be boolable".to_string()),
    };

    if b_val {
        return args
            .get(1)
            .map(|m| m.clone())
            .ok_or("Ok Expression not supplied".to_string());
    }
    Ok(args
        .get(2)
        .map(|v| v.clone())
        .unwrap_or(Value::String(String::new())))
}
pub fn v_match(args: &[Value]) -> Result<Value, String> {
    if args.len() <= 3 {
        return Err("match requires at least three args".to_string());
    }
    let mut it = args[1..].iter();
    while let Some(n) = it.next() {
        if n == &args[0] || n == &Value::String("$else".to_string()) {
            return it
                .next()
                .map(|m| m.clone())
                .ok_or("Match doesn't have valid response".to_string());
        }
        it.next();
    }
    Err("match not found".to_string())
}
