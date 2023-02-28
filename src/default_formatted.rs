/// Обёртка, чтобы вызывать fmt у std-контейнеров
#[repr(transparent)]
pub struct DefaultFormatted<T>(pub T);
