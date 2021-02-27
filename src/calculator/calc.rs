
mod calc {

    pub enum Sign {
        Plus, Minus
    }

    pub fn calculate(expr: &str) -> f64 {
        let first = get_number_position(expr, 0);
        if let Ok(pos) = first {
            let first = &expr[0..first].parse::<f64>();
            if first.is_ok() {
                let sign_pos = get_sign(expr, pos + 1);
                if let Ok(pos) = sign_pos {
                }
            }
        }
        0.0;
    }

    // TODO:
    // 1) use [u8] where possible, those &str shenanigans make the parsing
    //      tedious very quickly
    // 2) extract auxiliary parser struct that handles mutable position.

    fn get_number_position(expr: &str, pos: usize) -> Result<usize, String> {

        let expr_as_bytes = expr.as_bytes();

        let mut ind = pos;
        let dot = '.' as u8;
        let mut met_dot = false;
        while ind < expr.len() {
            match expr_as_bytes[pos] {
                t if t.is_ascii_digit() => {
                    ind += 1;
                }
                dot if !met_dot => {
                    met_dot = true;
                    ind += 1;
                }
                dot if met_dot => {
                    let msg = format!("too many dots around. '{}', pos {}", expr, ind);
                    return Err(msg);
                }
                _ => {
                    break;
                }
            }
        }
        Ok(ind)
    }

    fn get_sign(expr: &str, pos: usize) -> Result<usize, String> {

        let expr_as_bytes = expr.as_bytes();
        let mut ind = pos;
        while ind < expr.len() {
            match expr_as_bytes[ind] as char {
                c if c.is_ascii_whitespace() => {
                    ind += 1;
                },
                '+' => {
                    return Ok(ind);
                },
                '-' => {
                    return Ok(ind);
                },
                c => {
                    return Err("Unexpected char".to_string());
                }
            }
        }
        Err("Unexpected end of the string, no sign was found".to_string())
    }
}


fn main() {

    fn parse_slice(string: &str, start: usize, end: usize) {
        let result = &string[start..end].parse::<f64>();
        println!("'{}' -> {:?}", &string[start..end], result);
    }

    let result = calc::calculate("15");
    println!("Result is: {}", result);

    let expr = "15 1488 16.38";
    parse_slice(expr, 0, expr.len());
    parse_slice(expr, 0, 3);
    parse_slice(expr, 0, 2);
    parse_slice(expr, 0, 4);
    parse_slice(expr, 3, 7);

    parse_slice(expr, 8, 13);
    parse_slice(expr, 8, 11);
}
