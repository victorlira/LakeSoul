use datafusion::logical_expr::{col, lit, Expr, Operator};
use datafusion::scalar::ScalarValue;

pub struct Parser {
}

impl Parser {

    pub fn parse(str: String) -> Expr {
        let (op, left, right) = Parser::parse_filter_str(str);
        // println!("op: {}, left: {}, right: {}", op, left, right);
        match op.as_str() {
            "not" => {
                let inner = Parser::parse(right);
                print!("{:?}", inner);
                Expr::not(inner)
            }
            "eq" => {
                let column = col(left.as_str());
                let value = ScalarValue::Utf8(Some(right)); // todo: datatype conversion 
                let value = Expr::Literal(value); 
                column.eq(value)
            }
            "noteq" => {
                let column = col(left.as_str());
                let value = ScalarValue::Utf8(Some(right)); // todo: datatype conversion
                let value = Expr::Literal(value);
                column.not_eq(value)
            }
            "or" => {
                let left_expr = Parser::parse(left);
                let right_expr = Parser::parse(right);
                left_expr.or(right_expr)
            }
            "and" => {
                let left_expr = Parser::parse(left);
                let right_expr = Parser::parse(right);
                left_expr.and(right_expr)
            }
            "gt" => {
                let column = col(left.as_str());
                let value = ScalarValue::Utf8(Some(right)); // todo: datatype conversion
                let value = Expr::Literal(value);
                column.gt(value)
            }
            "gteq" => {
                let column = col(left.as_str());
                let value = ScalarValue::Utf8(Some(right)); // todo: datatype conversion
                let value = Expr::Literal(value);
                column.gt_eq(value)
            }
            "lt" => {
                let column = col(left.as_str());
                let value = ScalarValue::Utf8(Some(right)); // todo: datatype conversion
                let value = Expr::Literal(value);
                column.lt(value)
            }
            "lteq" => {
                let column = col(left.as_str());
                let value = ScalarValue::Utf8(Some(right)); // todo: datatype conversion
                let value = Expr::Literal(value);
                column.lt_eq(value)
            }

            _ => 
                Expr::Wildcard
        }
    }

    fn parse_filter_str(filter: String) -> (String, String, String) {
        let op_offset = filter.find('(').unwrap();
        let (op, filter) = filter.split_at(op_offset);
        if !filter.ends_with(")") {
            panic!("Invalid filter string");
        }
        let filter = &filter[1..filter.len()-1];
        let mut k:i8 = 0;
        let mut left_offset:usize = 0;
        for (i, ch) in filter.chars().enumerate() {
            match ch {
                '(' => 
                    k += 1,
                ')' => 
                    k -= 1,
                ',' => 
                    if k==0 {
                        left_offset = i
                    },
                _ => {}
            }
        }
        if k != 0 {
            panic!("Invalid filter string");
        }
        let (left,right) = filter.split_at(left_offset);
        if op.eq("not") {
            (op.to_string(), left.to_string(), right[0..].to_string())
        } else {
            (op.to_string(), left.to_string(), right[2..].to_string())
        }
    }


}

#[cfg(test)]
mod tests {
    use std::result::Result;
    use crate::filter::Parser;

    #[test]
    fn test_filter_parser() -> Result<(), String> {
        let s = String::from("or(lt(a.b.c, 2.0), gt(a.b.c, 3.0))");
        // let parser = Parser::new();
        Parser::parse(s);
        Ok(())
    }

    #[test]
    fn test_filter_parser_not() -> Result<(), String> {
        let s = String::from("not(eq(a.c, 2.9))");
        Parser::parse(s);
        Ok(())
    }
}