use std::fmt::{self, Display, Formatter};

// Обёртка, чтобы вызывать fmt у std-контейнеров
#[repr(transparent)]
pub struct DefaultFormatted<T>(pub T);

impl<T: Display> Display for DefaultFormatted<Option<T>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.0 {
            Some(x) => x.fmt(f),
            None => Ok(()),
        }
    }
}
