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

fn to_ou64(o: Option<&Value>) -> Option<u64> {
    match o {
        Some(Value::Number(n)) => n.as_u64(),
        Some(_) => None,
        None => None,
    }
}

pub fn num_range(args: &[Value]) -> Result<Value, String> {
    if args.len() == 0 {
        return Err("n_range needs at least one arg".to_string());
    }
    let s1 = to_ou64(args.get(0)).ok_or("num_range requires integers")?;
    let (start, stop) = match to_ou64(args.get(1)) {
        Some(n) => (s1, n),
        None => (0, s1),
    };
    let step = to_ou64(args.get(2)).unwrap_or(1);

    let mut res = Vec::new();
    let mut n = start;
    while n < stop {
        res.push(Value::Number(Number::from(n)));
        n += step;
    }
    Ok(Value::Array(res))
}
