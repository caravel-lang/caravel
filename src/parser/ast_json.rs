use super::ast;

pub trait JsonBuilder {
    fn to_json(&self) -> String;
}

impl JsonBuilder for ast::ExpressionNode {
    fn to_json(&self) -> String {
        match self.get_value() {
            ast::ExpressionValue::IntLiteral(val) => {
                format!(r#"{{"type": "int_literal", "value": {}}}"#, val.to_string())
            }
            ast::ExpressionValue::FloatLiteral(val) => format!(
                r#"{{"type": "float_literal", "value": {}}}"#,
                val.to_string()
            ),
            ast::ExpressionValue::StringLiteral(val) => {
                format!(r#"{{"type": "string_literal", "value": "{}"}}"#, val)
            }
            ast::ExpressionValue::CharLiteral(val) => format!(
                r#"{{"type": "char_literal", "value": "{}"}}"#,
                val.to_string()
            ),

            ast::ExpressionValue::BinaryOp(lhs, op, rhs) => format!(
                r#"{{"type": "binary_op", "lhs": {}, "op": "{}", "rhs": {}}}"#,
                lhs.to_json(),
                op.value,
                rhs.to_json()
            ),
        }
    }
}

impl JsonBuilder for ast::BodyNode {
    fn to_json(&self) -> String {
        let mut json = "".to_owned();

        json.push_str(r#"{"type": "body", "statements": ["#);

        let statements = self.get_statements();

        for (index, node) in statements.iter().enumerate() {
            json.push_str(&format!("{}", node.to_json())[..]);

            if index < statements.len() - 1 {
                json.push(',');
            }
        }

        json.push_str("]}");

        json
    }
}
