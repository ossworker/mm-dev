use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    /// A field that is a string.
    String,
    /// A field that is a number.
    Number,
    /// A field that is a boolean.
    Boolean,
    /// A field that is an array of strings.
    Date,
    /// A field that is an array .
    Array(Box<FieldType>),
    /// A field that is an object.
    Object(Box<Schema>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SchemaField {
    pub key: String,
    pub label: Option<String>,
    pub field_type: FieldType,
    pub required: bool,
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Schema {
    pub fields: Vec<SchemaField>,
}

impl Schema {
    /// Create a new schema.
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Add a field to the schema.
    pub fn add_field(&mut self, field: SchemaField) {
        self.fields.push(field);
    }

    /// Get the field by key.
    pub fn validate(&self, data: &Value) -> anyhow::Result<()> {
        if !data.is_object {
            return Err(errors::SchemaValidationError(
                "data must be an object".to_string(),
            ));
        }
        let data_obj = data.as_object()?;

        for field in &self.fields {
            if field.required && !data_obj.contains_key(&field.key) {
                return Err(errors::SchemaValidationError(format!(
                    "missing required field: {}",
                    field.key
                )));
            }

            if let Some(value) = data_obj.get(&field.key) {
                self.validate_field_type(&field.field_type, value, &field.key)?;
            }
        }

        Ok(())
    }

    fn validate_field_type(
        &self,
        field_type: &FieldType,
        value: &Value,
        path: &str,
    ) -> anyhow::Result<()> {
        match field_type {
            FieldType::String => {
                if !value.is_string() {
                    return Err(errors::SchemaValidationError(format!(
                        "field {} must be a string",
                        path
                    )));
                }
            }
            FieldType::Number => {
                if !value.is_number() {
                    return Err(errors::SchemaValidationError(format!(
                        "field {} must be a number",
                        path
                    )));
                }
            }
            FieldType::Boolean => {
                if !value.is_boolean() {
                    return Err(errors::SchemaValidationError(format!(
                        "field {} must be a boolean",
                        path
                    )));
                }
            }
            FieldType::Date => {
                if !value.is_string() {
                    return Err(errors::SchemaValidationError(format!(
                        "field {} must be a date",
                        path
                    )));
                }
            }
            FieldType::Array(item_type) => {
                if !value.is_array() {
                    return Err(errors::SchemaValidationError(format!(
                        "field {} must be an array",
                        path
                    )));
                }
                let array = value.as_array()?;
                for (i, item) in array.iter().enumerate() {
                    let item_path = format!("{}[{}]", path, i);
                    self.validate_field_type(item_type, item, &item_path)?;
                }
            }
            FieldType::Object(sub_schema) => {
                if !value.is_object() {
                    return Err(errors::SchemaValidationError(format!(
                        "field {} must be an object",
                        path
                    )));
                }
                sub_schema.validate(value)?;
            }
        }

        Ok(())
    }
}
