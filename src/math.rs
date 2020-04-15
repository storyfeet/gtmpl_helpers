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
