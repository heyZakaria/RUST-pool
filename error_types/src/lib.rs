
use chrono::Local;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: &str, field_value: &str, err: &str) -> Self {
        Self {
            form_values: (field_name.to_string(), field_value.to_string()),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn new(name: String, password: String) -> Self {
        Self { name, password }
    }

    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.trim().is_empty() {
            return Err(FormError::new("name", &self.name, "Username is empty"));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                &self.password,
                "Password should be at least 8 characters long",
            ));
        }

        let mut has_alpha = false;
        let mut has_numeric = false;
        let mut has_symbol = false;

        for c in self.password.chars() {
            if c.is_ascii_alphabetic() {
                has_alpha = true;
            } else if c.is_ascii_digit() {
                has_numeric = true;
            } else if !c.is_alphanumeric() && !c.is_whitespace() {
                has_symbol = true;
            }
        }

        if !(has_alpha && has_numeric && has_symbol) {
            return Err(FormError::new(
                "password",
                &self.password,
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }

        Ok(())
    }
}
