use JackStatus;
error_chain! {
    types {
        Error, ErrorKind, ChainErr, JackResult;
    }
    errors {
        NulError {
            description("NUL byte in client name")
        }
        JackOpenFailed(code: JackStatus) {
            description("Unable to connect to JACK server.")
                display("jack_client_open() failed: code {}", code.bits())
        }
        ProgrammerError {
            description("Programmer error: this should never happen")
                display("A programmer somewhere has made a mistake.")
        }
        InvalidPort {
            description("Invalid port passed to function")
        }
        UnknownErrorCode(from: &'static str, code: i32) {
            description("Unknown error code.")
                display("Error code {} in {}", code, from)
        }
        InvalidPortFlags {
            description("Invalid port type passed to function: `from` must be output, `to` must be input")
        }
        InvalidPortType {
            description("Invalid port type passed to function: the types of both ports must be equal")
        }
    }
}
