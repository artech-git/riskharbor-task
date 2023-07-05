
use std::panic::Location;
use std::backtrace::Backtrace;
use thiserror::Error;

#[derive(Error, Debug)]
struct BackendError<T> {
    location: Location<'a>
    backtrace: Backtrace,
    error: ErrorEvent
}


#[derive(Debug, Error)]
enum ErrorEvent {
    
}