// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

use core::str;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseStatusError {
    #[error("Invalid")]
    InvalidStatus
}

impl TryFrom<String> for Status {
    type Error = ParseStatusError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let status = parse_status(&value)?;
        Ok(status) 
    }
}

impl TryFrom<&str> for Status {
    type Error = ParseStatusError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let status = parse_status(value)?;
        Ok(status)
    }
}

fn parse_status(status: &str) -> Result<Status, ParseStatusError> {
    match status.to_lowercase().as_str() {
        "todo" => Ok(Status::ToDo),
        "inprogress" => Ok(Status::InProgress),
        "done" => Ok(Status::Done),
        _ => Err(ParseStatusError::InvalidStatus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
