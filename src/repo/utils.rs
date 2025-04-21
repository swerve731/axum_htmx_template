use askama::Template;
use axum::response::Html;


use super::error::RepoError;


#[derive(Template)]
#[template(path="auth/fragments/error/bad_password.html")]
pub struct BadPassword {
    has_uppercase: bool,
    has_lowercase: bool,
    has_digit: bool,
    min_length: usize,
    long_enough: bool,
}

pub fn is_valid_password(password: &str) -> Result<bool, RepoError> {
    let min_length = 8;
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    // let has_special_char = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;':\",.<>?/".contains(c));

    if password.len() >= min_length && has_uppercase && has_lowercase && has_digit {
        Ok(true)
    } else {
        Err(RepoError::ValidationError { 
            body: Html(BadPassword {
                has_uppercase,
                has_lowercase,
                has_digit,
                min_length,
                long_enough: password.len() >= min_length,
            }.render()?)
        })
    }
}

pub fn is_valid_email(email: &str) -> Result<bool, RepoError> {
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
    Err(RepoError::ValidationError { 
        body: Html(format!("Please enter a valid email."))
    })
}
