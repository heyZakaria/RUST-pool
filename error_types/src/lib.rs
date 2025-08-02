pub use chrono::Utc;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: &str, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value.to_string()),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err,
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
                &self.password.clone(),
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
                &self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }

        Ok(())
    }
}
