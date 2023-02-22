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

impl<T0: Display, T1: Display> Display for DefaultFormatted<(T0, T1)> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0 .0.fmt(f)?;
        self.0 .1.fmt(f)?;
        Ok(())
    }
}

impl<T0: Display, T1: Display, T2: Display> Display for DefaultFormatted<(T0, T1, T2)> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0 .0.fmt(f)?;
        self.0 .1.fmt(f)?;
        self.0 .2.fmt(f)?;
        Ok(())
    }
}

impl<T0: Display, T1: Display, T2: Display, T3: Display> Display
    for DefaultFormatted<(T0, T1, T2, T3)>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0 .0.fmt(f)?;
        self.0 .1.fmt(f)?;
        self.0 .2.fmt(f)?;
        self.0 .3.fmt(f)?;
        Ok(())
    }
}
