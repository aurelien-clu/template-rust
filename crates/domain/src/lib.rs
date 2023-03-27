use thiserror::Error;

#[derive(Error, Debug)]
pub enum SomeError {
    #[error("Value too high")]
    ValueTooHigh,
    #[error("Value too low")]
    ValueTooLow,
}

pub fn guess(g: u8) -> Result<(), SomeError> {
    log::trace!("guessing: {}...", g);

    if g > 5 {
        Err(SomeError::ValueTooHigh)
    } else if g < 5 {
        Err(SomeError::ValueTooLow)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_too_low() {
        let result = guess(4);
        match result {
            Err(SomeError::ValueTooLow) => assert!(true),
            _ => assert!(false),
        };
    }
    #[test]
    fn it_is_too_high() {
        let result = guess(6);
        match result {
            Err(SomeError::ValueTooHigh) => assert!(true),
            _ => assert!(false),
        };
    }
    #[test]
    fn it_is_just_right() {
        let result = guess(5);
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false),
        };
    }
}
