use validator::ValidationError;

// 自定义请求字段不能为空
pub fn validate_required(field: &str) -> Result<(), ValidationError> {
    if field.is_empty() {
        return Err(ValidationError::new("params_invalid"));
    }

    Ok(())
}
