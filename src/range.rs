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

pub fn compare_numbers(na: &Number, nb: &Number) -> Option<std::cmp::Ordering> {
    if let (Some(fa), Some(fb)) = (na.as_f64(), nb.as_f64()) {
        return fa.partial_cmp(&fb);
    }
    None
}

pub fn compare_values(va: &Value, vb: &Value) -> Option<std::cmp::Ordering> {
    match (va, vb) {
        (Value::String(sa), Value::String(sb)) => Some(sa.cmp(sb)),
        (Value::Number(na), Value::Number(nb)) => compare_numbers(na, nb),
        _ => None,
    }
}

pub fn sort_on(args: &[Value]) -> Result<Value, String> {
    let sval = match args.get(0) {
        Some(Value::String(s)) => s,
        _ => Err("Sort on requires a property name to sort on")?,
    };
    let mut ts = match args.get(1) {
        Some(Value::Array(a)) => a.clone(),
        _ => Err("Sort on requires an array to sort as second param")?,
    };
    //TODO consider allowing dot in property choice
    ts.sort_by(|a, b| {
        if let (Value::Map(ma), Value::Map(mb)) = (a, b) {
            match (ma.get(sval), mb.get(sval)) {
                (Some(va), Some(vb)) => {
                    return compare_values(va, vb).unwrap_or(std::cmp::Ordering::Equal)
                }
                _ => return std::cmp::Ordering::Equal,
            }
        }
        std::cmp::Ordering::Equal
    });
    //a2.sort_by()
    Ok(Value::Array(ts))
}

pub fn v_to_json(v: &Value) -> String {
    match v {
        Value::Map(m) | Value::Object(m) => {
            let mut res = "{".to_string();
            let mut precoma = "";
            let pre2 = if m.len() > 4 { ",\n\t" } else { "," };
            for (k, v) in m {
                res.push_str(&format!(r#"{}"{}":{}"#, precoma, k, v_to_json(v)));
                precoma = pre2;
            }
            res.push_str("}");
            res
        }
        Value::Array(a) => {
            let mut res = "[".to_string();
            let mut precoma = "";
            let pre2 = if a.len() > 5 { ",\n\t" } else { "," };
            for v in a {
                res.push_str(&format!(r"{}{}", precoma, v_to_json(v)));
                precoma = pre2
            }
            res.push(']');
            res
        }
        Value::String(s) => {
            let mut res = "\"".to_string();
            for c in s.chars() {
                match c {
                    //TODO handle unicode points and backspace
                    '\\' => res.push_str("\\\\"),
                    '"' => res.push_str("\\\""),
                    '\n' => res.push_str("\\n"),
                    '\r' => res.push_str("\\r"),
                    '\t' => res.push_str("\\t"),
                    '/' => res.push_str("\\/"),
                    cv => res.push(cv),
                }
            }
            res.push('\"');
            res
        }
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                f.to_string()
            } else if let Some(i) = n.as_i64() {
                i.to_string()
            } else {
                n.as_u64().unwrap().to_string()
            }
        }
        _ => "null".to_string(),
    }
}

///convert the value to a json string
pub fn to_json(args: &[Value]) -> Result<Value, String> {
    if args.len() == 0 {
        return Ok(Value::String("null".to_string()));
    }
    if args.len() == 1 {
        return Ok(Value::String(v_to_json(&args[0])));
    }

    let mut res = "[".to_string();
    let mut precoma = "";
    let pre2 = if args.len() > 5 { ",\n\t" } else { "," };
    for v in args {
        res.push_str(&format!(r"{}{}", precoma, v_to_json(v)));
        precoma = pre2;
    }
    res.push(']');
    Ok(Value::String(res))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_number_prints_a_string() {
        let v = Value::Number(Number::from(43));
        let s = v_to_json(&v);
        assert_eq!(s, "43");
    }
}
