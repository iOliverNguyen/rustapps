use gpui::Keystroke;
use once_cell::sync::Lazy;
use std::panic::Location;
use tracing::error;

pub static KEY_SPACE: Lazy<Keystroke> = Lazy::new(|| Keystroke::parse("space").unwrap());

pub trait ResultExt<E> {
    type Ok;

    fn log_err(self) -> Option<Self::Ok>;
}

impl<T, E> ResultExt<E> for Result<T, E>
where
    E: std::fmt::Debug,
{
    type Ok = T;

    #[track_caller]
    fn log_err(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                let caller = Location::caller();
                error!("{}:{}: {:?}", caller.file(), caller.line(), error);
                None
            }
        }
    }
}
