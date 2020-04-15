//use gtmpl::Template;
use gtmpl_value::{Number, Value};
pub fn mul(args: &[Value]) -> Result<Value, String> {
    let mut res = Number::from(1);
    for a in args {
        if let Value::Number(n2) = a {
            if let (Some(f1), Some(f2)) = (res.as_f64(), n2.as_f64()) {
                res = Number::from(f1 * f2);
            } else if let (Some(i1), Some(i2)) = (res.as_i64(), n2.as_i64()) {
                res = Number::from(i1 * i2);
            } else if let (Some(u1), Some(u2)) = (res.as_u64(), n2.as_u64()) {
                res = Number::from(u1 * u2);
            } else {
                return Err("Numbers not compatible".to_string());
            }
        } else {
            return Err(format!("not a number {:?}", a));
        }
    }
    Ok(Value::Number(res))
}

pub fn div(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Divide requires exactly 2 arguments".to_string());
    }

    if let (Value::Number(a), Value::Number(b)) = (&args[0], &args[1]) {
        if let (Some(f1), Some(f2)) = (a.as_f64(), b.as_f64()) {
            Ok(Value::Number(Number::from(f1 / f2)))
        } else if let (Some(i1), Some(i2)) = (a.as_i64(), b.as_i64()) {
            Ok(Value::Number(Number::from(i1 + i2)))
        } else if let (Some(u1), Some(u2)) = (a.as_u64(), b.as_u64()) {
            Ok(Value::Number(Number::from(u1 + u2)))
        } else {
            Err("Numbers not compatible".to_string())
        }
    } else {
        Err("Divide requires 2 Number arguments".to_string())
    }
}

pub fn add(args: &[Value]) -> Result<Value, String> {
    let mut res = Number::from(0);
    for a in args {
        if let Value::Number(n2) = a {
            if let (Some(f1), Some(f2)) = (res.as_f64(), n2.as_f64()) {
                res = Number::from(f1 + f2);
            } else if let (Some(i1), Some(i2)) = (res.as_i64(), n2.as_i64()) {
                res = Number::from(i1 + i2);
            } else if let (Some(u1), Some(u2)) = (res.as_u64(), n2.as_u64()) {
                res = Number::from(u1 + u2);
            } else {
                return Err("Numbers not compatible".to_string());
            }
        } else {
            return Err(format!("not a number {:?}", a));
        }
    }
    Ok(Value::Number(res))
}

pub fn sub(args: &[Value]) -> Result<Value, String> {
    if args.len() == 0 {
        return Err("No Args".to_string());
    }
    let n = match &args[0] {
        Value::Number(n) => n,
        _ => return Err("Cannot sub non num".to_string()),
    };

    if args.len() == 0 {
        if let Some(f) = n.as_f64() {
            return Ok(Value::Number(Number::from(-f)));
        } else if let Some(i) = n.as_i64() {
            return Ok(Value::Number(Number::from(-i)));
        }
        return Err("Too big to negate".to_string());
    }
    let mut res = Number::from(0);
    for a in &args[1..] {
        if let Value::Number(n2) = a {
            if let (Some(f1), Some(f2)) = (res.as_f64(), n2.as_f64()) {
                res = Number::from(f1 - f2);
            } else if let (Some(i1), Some(i2)) = (res.as_i64(), n2.as_i64()) {
                res = Number::from(i1 - i2);
            } else if let (Some(u1), Some(u2)) = (res.as_u64(), n2.as_u64()) {
                if u1 < u2 {
                    return Err(format!("Cannot subtract {} from {}", u2, u1));
                }
                res = Number::from(u1 - u2);
            } else {
                return Err("Numbers not compatible".to_string());
            }
        } else {
            return Err(format!("not a number {:?}", a));
        }
    }
    Ok(Value::Number(res))
}
