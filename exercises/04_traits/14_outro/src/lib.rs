// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
use std::ops::Add;


#[derive(Copy, Clone, Debug)]
pub struct SaturatingU16 {
    value: u16,
}


impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 {
            value
        }
    }
}


impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: value.into()
        }
    }
}


impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 {
            value: *value
        }
    }
}


impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 {
            value: (*value).into()
        }
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> Self::Output {
        SaturatingU16{
            value: self.value.saturating_add(rhs.value)
        }
    }
}


impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(rhs)
        }
    }
}


impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(*rhs)
        }
    }
}


impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add((*rhs).value)
        }
    }
}



impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}


impl PartialEq<SaturatingU16> for SaturatingU16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        self.value == other.value
    }
}
