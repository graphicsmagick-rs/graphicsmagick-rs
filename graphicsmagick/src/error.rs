//! Crate level errors.

use num_enum::{FromPrimitive, IntoPrimitive};
use thiserror::Error as ThisError;

/// Crate result.
pub type Result<T> = std::result::Result<T, Error>;

/// Crate error.
#[non_exhaustive]
#[derive(ThisError, Debug)]
pub enum Error {
    /// GraphicsMagick Exception.
    #[error(transparent)]
    Exception(#[from] Exception),
}

/// Wrapper of `graphicsmagick_sys::ExceptionType` and ExceptionInfo.
#[derive(ThisError, Debug, Eq, PartialEq)]
#[error("kind: {kind:?}, description: {description}")]
pub struct Exception {
    kind: ExceptionType,
    description: String,
}

impl Exception {
    pub fn new(kind: ExceptionType, description: String) -> Self {
        Exception { kind, description }
    }

    pub fn get_exception_type(&self) -> ExceptionType {
        self.kind
    }
}

/// Wrapper of [ExceptionType](http://www.graphicsmagick.org/api/types.html#exceptiontype).
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum ExceptionType {
    UndefinedException = graphicsmagick_sys::ExceptionType_UndefinedException,
    EventException = graphicsmagick_sys::ExceptionType_EventException,
    ExceptionEvent = graphicsmagick_sys::ExceptionType_ExceptionEvent,
    /// Alias: ResourceLimitEvent
    ResourceEvent = graphicsmagick_sys::ExceptionType_ResourceEvent,
    /// Alias: AnnotateEvent
    TypeEvent = graphicsmagick_sys::ExceptionType_TypeEvent,
    OptionEvent = graphicsmagick_sys::ExceptionType_OptionEvent,
    DelegateEvent = graphicsmagick_sys::ExceptionType_DelegateEvent,
    MissingDelegateEvent = graphicsmagick_sys::ExceptionType_MissingDelegateEvent,
    CorruptImageEvent = graphicsmagick_sys::ExceptionType_CorruptImageEvent,
    FileOpenEvent = graphicsmagick_sys::ExceptionType_FileOpenEvent,
    BlobEvent = graphicsmagick_sys::ExceptionType_BlobEvent,
    StreamEvent = graphicsmagick_sys::ExceptionType_StreamEvent,
    CacheEvent = graphicsmagick_sys::ExceptionType_CacheEvent,
    CoderEvent = graphicsmagick_sys::ExceptionType_CoderEvent,
    ModuleEvent = graphicsmagick_sys::ExceptionType_ModuleEvent,
    /// Alias: RenderEvent
    DrawEvent = graphicsmagick_sys::ExceptionType_DrawEvent,
    ImageEvent = graphicsmagick_sys::ExceptionType_ImageEvent,
    WandEvent = graphicsmagick_sys::ExceptionType_WandEvent,
    TemporaryFileEvent = graphicsmagick_sys::ExceptionType_TemporaryFileEvent,
    TransformEvent = graphicsmagick_sys::ExceptionType_TransformEvent,
    XServerEvent = graphicsmagick_sys::ExceptionType_XServerEvent,
    X11Event = graphicsmagick_sys::ExceptionType_X11Event,
    UserEvent = graphicsmagick_sys::ExceptionType_UserEvent,
    MonitorEvent = graphicsmagick_sys::ExceptionType_MonitorEvent,
    LocaleEvent = graphicsmagick_sys::ExceptionType_LocaleEvent,
    DeprecateEvent = graphicsmagick_sys::ExceptionType_DeprecateEvent,
    RegistryEvent = graphicsmagick_sys::ExceptionType_RegistryEvent,
    ConfigureEvent = graphicsmagick_sys::ExceptionType_ConfigureEvent,
    WarningException = graphicsmagick_sys::ExceptionType_WarningException,
    ExceptionWarning = graphicsmagick_sys::ExceptionType_ExceptionWarning,
    /// Alias: ResourceLimitWarning
    ResourceWarning = graphicsmagick_sys::ExceptionType_ResourceWarning,
    /// Alias: AnnotateWarning
    TypeWarning = graphicsmagick_sys::ExceptionType_TypeWarning,
    OptionWarning = graphicsmagick_sys::ExceptionType_OptionWarning,
    DelegateWarning = graphicsmagick_sys::ExceptionType_DelegateWarning,
    MissingDelegateWarning = graphicsmagick_sys::ExceptionType_MissingDelegateWarning,
    CorruptImageWarning = graphicsmagick_sys::ExceptionType_CorruptImageWarning,
    FileOpenWarning = graphicsmagick_sys::ExceptionType_FileOpenWarning,
    BlobWarning = graphicsmagick_sys::ExceptionType_BlobWarning,
    StreamWarning = graphicsmagick_sys::ExceptionType_StreamWarning,
    CacheWarning = graphicsmagick_sys::ExceptionType_CacheWarning,
    CoderWarning = graphicsmagick_sys::ExceptionType_CoderWarning,
    ModuleWarning = graphicsmagick_sys::ExceptionType_ModuleWarning,
    /// Alias: RenderWarning
    DrawWarning = graphicsmagick_sys::ExceptionType_DrawWarning,
    ImageWarning = graphicsmagick_sys::ExceptionType_ImageWarning,
    WandWarning = graphicsmagick_sys::ExceptionType_WandWarning,
    TemporaryFileWarning = graphicsmagick_sys::ExceptionType_TemporaryFileWarning,
    TransformWarning = graphicsmagick_sys::ExceptionType_TransformWarning,
    XServerWarning = graphicsmagick_sys::ExceptionType_XServerWarning,
    X11Warning = graphicsmagick_sys::ExceptionType_X11Warning,
    UserWarning = graphicsmagick_sys::ExceptionType_UserWarning,
    MonitorWarning = graphicsmagick_sys::ExceptionType_MonitorWarning,
    LocaleWarning = graphicsmagick_sys::ExceptionType_LocaleWarning,
    DeprecateWarning = graphicsmagick_sys::ExceptionType_DeprecateWarning,
    RegistryWarning = graphicsmagick_sys::ExceptionType_RegistryWarning,
    ConfigureWarning = graphicsmagick_sys::ExceptionType_ConfigureWarning,
    ErrorException = graphicsmagick_sys::ExceptionType_ErrorException,
    ExceptionError = graphicsmagick_sys::ExceptionType_ExceptionError,
    /// Alias: ResourceLimitError
    ResourceError = graphicsmagick_sys::ExceptionType_ResourceError,
    /// Alias: AnnotateError
    TypeError = graphicsmagick_sys::ExceptionType_TypeError,
    OptionError = graphicsmagick_sys::ExceptionType_OptionError,
    DelegateError = graphicsmagick_sys::ExceptionType_DelegateError,
    MissingDelegateError = graphicsmagick_sys::ExceptionType_MissingDelegateError,
    CorruptImageError = graphicsmagick_sys::ExceptionType_CorruptImageError,
    FileOpenError = graphicsmagick_sys::ExceptionType_FileOpenError,
    BlobError = graphicsmagick_sys::ExceptionType_BlobError,
    StreamError = graphicsmagick_sys::ExceptionType_StreamError,
    CacheError = graphicsmagick_sys::ExceptionType_CacheError,
    CoderError = graphicsmagick_sys::ExceptionType_CoderError,
    ModuleError = graphicsmagick_sys::ExceptionType_ModuleError,
    /// Alias: RenderError
    DrawError = graphicsmagick_sys::ExceptionType_DrawError,
    ImageError = graphicsmagick_sys::ExceptionType_ImageError,
    WandError = graphicsmagick_sys::ExceptionType_WandError,
    TemporaryFileError = graphicsmagick_sys::ExceptionType_TemporaryFileError,
    TransformError = graphicsmagick_sys::ExceptionType_TransformError,
    XServerError = graphicsmagick_sys::ExceptionType_XServerError,
    X11Error = graphicsmagick_sys::ExceptionType_X11Error,
    UserError = graphicsmagick_sys::ExceptionType_UserError,
    MonitorError = graphicsmagick_sys::ExceptionType_MonitorError,
    LocaleError = graphicsmagick_sys::ExceptionType_LocaleError,
    DeprecateError = graphicsmagick_sys::ExceptionType_DeprecateError,
    RegistryError = graphicsmagick_sys::ExceptionType_RegistryError,
    ConfigureError = graphicsmagick_sys::ExceptionType_ConfigureError,
    FatalErrorException = graphicsmagick_sys::ExceptionType_FatalErrorException,
    ExceptionFatalError = graphicsmagick_sys::ExceptionType_ExceptionFatalError,
    /// Alias: ResourceLimitFatalError
    ResourceFatalError = graphicsmagick_sys::ExceptionType_ResourceFatalError,
    /// Alias: AnnotateFatalError
    TypeFatalError = graphicsmagick_sys::ExceptionType_TypeFatalError,
    OptionFatalError = graphicsmagick_sys::ExceptionType_OptionFatalError,
    DelegateFatalError = graphicsmagick_sys::ExceptionType_DelegateFatalError,
    MissingDelegateFatalError = graphicsmagick_sys::ExceptionType_MissingDelegateFatalError,
    CorruptImageFatalError = graphicsmagick_sys::ExceptionType_CorruptImageFatalError,
    FileOpenFatalError = graphicsmagick_sys::ExceptionType_FileOpenFatalError,
    BlobFatalError = graphicsmagick_sys::ExceptionType_BlobFatalError,
    StreamFatalError = graphicsmagick_sys::ExceptionType_StreamFatalError,
    CacheFatalError = graphicsmagick_sys::ExceptionType_CacheFatalError,
    CoderFatalError = graphicsmagick_sys::ExceptionType_CoderFatalError,
    ModuleFatalError = graphicsmagick_sys::ExceptionType_ModuleFatalError,
    /// Alias: RenderFatalError
    DrawFatalError = graphicsmagick_sys::ExceptionType_DrawFatalError,
    ImageFatalError = graphicsmagick_sys::ExceptionType_ImageFatalError,
    WandFatalError = graphicsmagick_sys::ExceptionType_WandFatalError,
    TemporaryFileFatalError = graphicsmagick_sys::ExceptionType_TemporaryFileFatalError,
    TransformFatalError = graphicsmagick_sys::ExceptionType_TransformFatalError,
    XServerFatalError = graphicsmagick_sys::ExceptionType_XServerFatalError,
    X11FatalError = graphicsmagick_sys::ExceptionType_X11FatalError,
    UserFatalError = graphicsmagick_sys::ExceptionType_UserFatalError,
    MonitorFatalError = graphicsmagick_sys::ExceptionType_MonitorFatalError,
    LocaleFatalError = graphicsmagick_sys::ExceptionType_LocaleFatalError,
    DeprecateFatalError = graphicsmagick_sys::ExceptionType_DeprecateFatalError,
    RegistryFatalError = graphicsmagick_sys::ExceptionType_RegistryFatalError,
    ConfigureFatalError = graphicsmagick_sys::ExceptionType_ConfigureFatalError,

    #[num_enum(default)]
    Unknown = u32::MAX,
}
