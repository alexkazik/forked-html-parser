use ownable::{IntoOwned, ToBorrowed, ToOwned};
use serde::Serialize;
use std::borrow::{Borrow, Cow};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

#[derive(Default, Debug, Serialize, PartialEq, Eq, IntoOwned, ToBorrowed, ToOwned)]
#[serde(transparent)]
pub struct Attribute<'a>(pub Cow<'a, str>);

impl<'a> Attribute<'a> {
    pub fn as_str(&self) -> &str {
        self.0.borrow()
    }
}

impl<S: Into<Cow<'static, str>>> From<S> for Attribute<'static> {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

impl<'a> Deref for Attribute<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> PartialEq<str> for Attribute<'a> {
    fn eq(&self, other: &str) -> bool {
        self.deref().eq(other)
    }
}

impl<'a> PartialEq<&str> for Attribute<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.deref().eq(*other)
    }
}

impl<'a> PartialEq<str> for &Attribute<'a> {
    fn eq(&self, other: &str) -> bool {
        (*self).deref().eq(other)
    }
}

#[derive(Default, Serialize, PartialEq, Eq, IntoOwned, ToBorrowed, ToOwned)]
#[serde(transparent)]
pub struct Text<'a>(pub Cow<'a, str>);

impl<'a> Text<'a> {
    pub fn as_str(&self) -> &str {
        self.0.borrow()
    }
}

impl Debug for Text<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<S: Into<Cow<'static, str>>> From<S> for Text<'static> {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

impl<'a> Deref for Text<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> PartialEq<str> for Text<'a> {
    fn eq(&self, other: &str) -> bool {
        self.deref().eq(other)
    }
}

impl<'a> PartialEq<&str> for Text<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.deref().eq(*other)
    }
}

impl<'a> PartialEq<str> for &Text<'a> {
    fn eq(&self, other: &str) -> bool {
        (*self).deref().eq(other)
    }
}
