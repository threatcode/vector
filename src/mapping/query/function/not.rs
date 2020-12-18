use super::prelude::*;

#[derive(Debug)]
pub(in crate::mapping) struct NotFn {
    query: Box<dyn Function>,
}

impl NotFn {
    pub(in crate::mapping) fn new(query: Box<dyn Function>) -> Self {
        Self { query }
    }
}

impl Function for NotFn {
    fn execute(&self, ctx: &Event) -> Result<QueryValue> {
        self.query.execute(ctx).and_then(|v| match v {
            QueryValue::Value(Value::Boolean(b)) => Ok(Value::Boolean(!b).into()),
            QueryValue::Value(v) => Err(format!("unable to perform NOT on {:?} value", v)),
            v => Err(format!("unable to perform NOT on {:?} value", v)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{log_event, mapping::query::path::Path};

    #[test]
    fn not() {
        let cases = vec![
            (
                log_event! {
                    crate::config::log_schema().message_key().clone() => "".to_string(),
                    crate::config::log_schema().timestamp_key().clone() => chrono::Utc::now(),
                },
                Err("path .foo not found in event".to_string()),
                NotFn::new(Box::new(Path::from(vec![vec!["foo"]]))),
            ),
            (
                log_event! {
                    crate::config::log_schema().message_key().clone() => "".to_string(),
                    crate::config::log_schema().timestamp_key().clone() => chrono::Utc::now(),
                },
                Ok(Value::Boolean(false)),
                NotFn::new(Box::new(Literal::from(Value::Boolean(true)))),
            ),
            (
                log_event! {
                    crate::config::log_schema().message_key().clone() => "".to_string(),
                    crate::config::log_schema().timestamp_key().clone() => chrono::Utc::now(),
                },
                Ok(Value::Boolean(true)),
                NotFn::new(Box::new(Literal::from(Value::Boolean(false)))),
            ),
            (
                log_event! {
                    crate::config::log_schema().message_key().clone() => "".to_string(),
                    crate::config::log_schema().timestamp_key().clone() => chrono::Utc::now(),
                },
                Err("unable to perform NOT on Bytes(b\"not a bool\") value".to_string()),
                NotFn::new(Box::new(Literal::from(Value::from("not a bool")))),
            ),
        ];

        for (input_event, exp, query) in cases {
            assert_eq!(query.execute(&input_event), exp.map(QueryValue::Value));
        }
    }
}
