use JackStatus;

pub type JackResult<T> = Result<T, JackError>;

#[derive(Debug, Fail)]
pub enum JackError {
    #[fail(display = "NUL byte in client name")]
    NulError,
    #[fail(display = "jack_client_open() failed")]
    JackOpenFailed(JackStatus),
    #[fail(display = "Programmer error: this should never happen")]
    ProgrammerError,
    #[fail(display = "Invalid port passed to function")]
    InvalidPort,
    #[fail(display = "A port matching that name could not be found.")]
    PortNotFound,
    #[fail(display = "Error code {} in {}", code, from)]
    UnknownErrorCode {
        from: &'static str,
        code: i32
    },
    #[fail(display = "Could not register port (see docs for more details)")]
    PortRegistrationFailed,
    #[fail(display = "Invalid port passed to function: `from` must be output, `to` must be input")]
    InvalidPortFlags,
    #[fail(display = "Invalid port passed to function: the types of both ports must be equal")]
    InvalidPortType,
    #[fail(display = "This action requires the port to be owned by the client")]
    PortNotMine,
    #[fail(display = "The new buffer size was not a power of two.")]
    NotPowerOfTwo,
}
