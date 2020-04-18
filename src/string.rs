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

pub fn wrap(args: &[Value]) -> Result<Value, String> {
    let s = match args.get(0) {
        Some(Value::String(s)) => s,
        _ => return Err("Value 0 not a string".to_string()),
    };
    let n = match args.get(1) {
        Some(Value::Number(n)) => n.as_u64().ok_or("Value 1 not a positive int".to_string())?,
        _ => return Err("Value 1 not a num".to_string()),
    };
    let vs = _wrap(s, n as usize);
    Ok(Value::Array(
        vs.into_iter().map(|v| Value::String(v)).collect(),
    ))
}

fn _wrap(s: &str, mx: usize) -> Vec<String> {
    let mut cword = String::new();
    let mut cline = String::new();
    let mut res: Vec<String> = Vec::new();

    for c in s.chars() {
        if cline.len() + cword.len() > mx {
            if cline.len() == 0 {
                cline.push_str(&cword[..mx]);
                cline.push('-');
                cword = String::from(&cword[mx..]);
            } else {
                cword = cword[1..].to_string();
            }

            res.push(cline);
            cline = "".to_string();
        }
        match c {
            '\n' => {
                cline.push_str(&cword);
                cword.clear();
                res.push(cline);
                cline = "".to_string();
            }
            '-' => {
                cline.push_str(&cword);
                cline.push('-');
                cword = String::from(" ");
            }
            ' ' => {
                cline.push_str(&cword);
                cword = String::from(" ");
            }
            _ => {
                cword.push(c);
            }
        }
    }
    cline.push_str(&cword);
    res.push(cline);
    res
}
