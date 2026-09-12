#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(thiserror::Error, Debug)]
enum StatusConversionError {
    #[error("{0}")]
    InvalidStringInput(String)
}

impl TryFrom<String> for Status {
    type Error = StatusConversionError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.to_lowercase() == String::from("todo") {
            return Ok(Status::ToDo)
        }

        if value.to_lowercase() == String::from("inprogress") {
            return Ok(Status::InProgress)
        }

        if value.to_lowercase() == String::from("done") {
            return Ok(Status::Done)
        }

        Err(StatusConversionError::InvalidStringInput(
            "To make conversion value should be either `ToDo`, `InProgress` or `Done`".into()
        ))
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.to_lowercase() == "todo" {
            return Ok(Status::ToDo)
        }

        if value.to_lowercase() == "inprogress" {
            return Ok(Status::InProgress)
        }

        if value.to_lowercase() == "done" {
            return Ok(Status::Done)
        }

        Err(StatusConversionError::InvalidStringInput(
            "To make conversion value should be either `ToDo`, `InProgress` or `Done`".into()
        ))
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

        let empty = String::new();
        assert!(Status::try_from(empty).is_err());

        let invalid = "InValid".to_string();
        assert!(Status::try_from(invalid).is_err());
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);

        assert!(Status::try_from("").is_err());
        assert!(Status::try_from("invalid").is_err());
    }
}
