use gtmpl_value::Value;

/// svg properties x y in that order
/// Optional param units on the end, else assume px
pub fn xy(args: &[Value]) -> Result<Value, String> {
    let u = match args.get(3) {
        Some(v) => v.to_string(),
        None => "px".to_string(),
    };
    let y = args.get(1).ok_or("Y not supplied".to_string())?;
    let x = args.get(0).ok_or("X not supplied".to_string())?;
    Ok(Value::String(format!(r#"x="{}{2}" y="{}{2}" "#, x, y, u)))
}

/// svg properties x y width and height, in that order
/// Optional param units on the end, else assume px
pub fn xywh(args: &[Value]) -> Result<Value, String> {
    let u = match args.get(4) {
        Some(v) => v.to_string(),
        None => "px".to_string(),
    };
    let h = args.get(3).ok_or("H not supplied".to_string())?;
    let w = args.get(2).ok_or("W not supplied".to_string())?;
    let y = args.get(1).ok_or("Y not supplied".to_string())?;
    let x = args.get(0).ok_or("X not supplied".to_string())?;
    Ok(Value::String(format!(
        r#"x="{}{4}" y="{}{4}" width="{}{4}" height="{}{4}" "#,
        x, y, w, h, u
    )))
}
/// svg properties x y width and height, in that order
/// Optional param units on the end, else assume px
pub fn xy12(args: &[Value]) -> Result<Value, String> {
    let u = match args.get(4) {
        Some(v) => v.to_string(),
        None => "px".to_string(),
    };
    let y2 = args.get(3).ok_or("Y2 not supplied".to_string())?;
    let x2 = args.get(2).ok_or("X2 not supplied".to_string())?;
    let y = args.get(1).ok_or("Y2 not supplied".to_string())?;
    let x = args.get(0).ok_or("X1 not supplied".to_string())?;
    Ok(Value::String(format!(
        r#"x1="{}{4}" y1="{}{4}" x2="{}{4}" y2="{}{4}" "#,
        x, y, x2, y2, u
    )))
}

/// Svg properties fill stroke and stroke-width
/// @args : fill, stroke, stroke-width , [Units]
pub fn fl_stk(args: &[Value]) -> Result<Value, String> {
    let u = match args.get(3) {
        Some(v) => v.to_string(),
        None => "px".to_string(),
    };
    let f = args.get(0).ok_or("Fill not supplied".to_string())?;
    let s = args.get(1).ok_or("Stroke not supplied".to_string())?;
    let w = args.get(2).ok_or("StrokeWidth not supplied".to_string())?;
    //TODO add px only for numbers
    Ok(Value::String(format!(
        r#"fill="{}" stroke="{}" stroke-width="{}{}" "#,
        f, s, w, u
    )))
}

pub fn font(args: &[Value]) -> Result<Value, String> {
    let sz = args.get(0).ok_or("Font Size not supplied".to_string())?;
    let ff = args
        .get(1)
        .map(|s| format!(r#"font-family="{}" "#, s))
        .unwrap_or(String::new());

    let u = match args.get(2) {
        Some(v) => v.to_string(),
        None => "px".to_string(),
    };
    Ok(Value::String(format!(r#"font-size="{}{2}" {}"#, sz, ff, u)))
}

fn _xml_es(s: &str) -> String {
    let mut res = String::new();
    for c in s.chars() {
        match c {
            '&' => res.push_str("&amp;"),
            '>' => res.push_str("&gt;"),
            '<' => res.push_str("&lt;"),
            '\"' => res.push_str("&quot;"),
            '\'' => res.push_str("&apos;"),
            cv => res.push(cv),
        }
    }
    res
}

pub fn xml_es(args: &[Value]) -> Result<Value, String> {
    let mut res = String::new();
    for a in args {
        res.push_str(&_xml_es(&a.to_string()));
    }
    Ok(Value::String(res))
}
