pub enum InterfaceError {
    SDLInitializationError,
    WindowCreationError,
    CanvasCreationError,
    EventPumpCreationError,
    OtherError(String),
}