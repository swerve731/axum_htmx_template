use crate::app::auth_service::error::AuthError;

pub fn is_valid_password(password: &str) -> Result<bool, AuthError> {
    let min_length = 8;
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    // let has_special_char = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;':\",.<>?/".contains(c));

    if password.len() >= min_length && has_uppercase && has_lowercase && has_digit {
        Ok(true)
    } else {
        Err(AuthError::InvalidPassword {
            has_uppercase,
            has_lowercase,
            has_digit,
            min_length,
            is_long_enough: password.len() >= min_length,
        })
    }
}

pub fn is_valid_email(email: &str) -> Result<bool, AuthError> {
    // without regex
    let at = email.find('@');
    let dot = email.rfind('.');
    if let Some(at) = at {
        if let Some(dot) = dot {
            if at < dot && dot < email.len() - 1 {
                return Ok(true);
            }
        }
    } 
    Err(AuthError::InvalidEmail)
}