use std::{io, str::Utf8Error};
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    //    #[error(transparent)]
    //    Io(#[from] io::Error),
    #[error(transparent)]
    Utf8(#[from] Utf8Error),

    #[error(transparent)]
    Image(#[from] Exception),
}

#[derive(ThisError, Debug)]
#[error("kind: {kind:?}, description: {description}")]
pub struct Exception {
    kind: ExceptionType,
    description: String,
}

impl Exception {
    pub fn new(kind: ExceptionType, description: String) -> Self {
        Exception { kind, description }
    }
}

c_enum_block! {
    /// Wrapper of [ExceptionType](http://www.graphicsmagick.org/api/types.html#exceptiontype).
    ExceptionType;
    (graphicsmagick_sys::ExceptionType_UndefinedException, UndefinedException);
    (graphicsmagick_sys::ExceptionType_EventException, EventException);
    (graphicsmagick_sys::ExceptionType_ExceptionEvent, ExceptionEvent);
    (graphicsmagick_sys::ExceptionType_ResourceEvent, ResourceEvent);
    (graphicsmagick_sys::ExceptionType_ResourceLimitEvent, ResourceLimitEvent);
    (graphicsmagick_sys::ExceptionType_TypeEvent, TypeEvent);
    (graphicsmagick_sys::ExceptionType_AnnotateEvent, AnnotateEvent);
    (graphicsmagick_sys::ExceptionType_OptionEvent, OptionEvent);
    (graphicsmagick_sys::ExceptionType_DelegateEvent, DelegateEvent);
    (graphicsmagick_sys::ExceptionType_MissingDelegateEvent, MissingDelegateEvent);
    (graphicsmagick_sys::ExceptionType_CorruptImageEvent, CorruptImageEvent);
    (graphicsmagick_sys::ExceptionType_FileOpenEvent, FileOpenEvent);
    (graphicsmagick_sys::ExceptionType_BlobEvent, BlobEvent);
    (graphicsmagick_sys::ExceptionType_StreamEvent, StreamEvent);
    (graphicsmagick_sys::ExceptionType_CacheEvent, CacheEvent);
    (graphicsmagick_sys::ExceptionType_CoderEvent, CoderEvent);
    (graphicsmagick_sys::ExceptionType_ModuleEvent, ModuleEvent);
    (graphicsmagick_sys::ExceptionType_DrawEvent, DrawEvent);
    (graphicsmagick_sys::ExceptionType_RenderEvent, RenderEvent);
    (graphicsmagick_sys::ExceptionType_ImageEvent, ImageEvent);
    (graphicsmagick_sys::ExceptionType_WandEvent, WandEvent);
    (graphicsmagick_sys::ExceptionType_TemporaryFileEvent, TemporaryFileEvent);
    (graphicsmagick_sys::ExceptionType_TransformEvent, TransformEvent);
    (graphicsmagick_sys::ExceptionType_XServerEvent, XServerEvent);
    (graphicsmagick_sys::ExceptionType_X11Event, X11Event);
    (graphicsmagick_sys::ExceptionType_UserEvent, UserEvent);
    (graphicsmagick_sys::ExceptionType_MonitorEvent, MonitorEvent);
    (graphicsmagick_sys::ExceptionType_LocaleEvent, LocaleEvent);
    (graphicsmagick_sys::ExceptionType_DeprecateEvent, DeprecateEvent);
    (graphicsmagick_sys::ExceptionType_RegistryEvent, RegistryEvent);
    (graphicsmagick_sys::ExceptionType_ConfigureEvent, ConfigureEvent);
    (graphicsmagick_sys::ExceptionType_WarningException, WarningException);
    (graphicsmagick_sys::ExceptionType_ExceptionWarning, ExceptionWarning);
    (graphicsmagick_sys::ExceptionType_ResourceWarning, ResourceWarning);
    (graphicsmagick_sys::ExceptionType_ResourceLimitWarning, ResourceLimitWarning);
    (graphicsmagick_sys::ExceptionType_TypeWarning, TypeWarning);
    (graphicsmagick_sys::ExceptionType_AnnotateWarning, AnnotateWarning);
    (graphicsmagick_sys::ExceptionType_OptionWarning, OptionWarning);
    (graphicsmagick_sys::ExceptionType_DelegateWarning, DelegateWarning);
    (graphicsmagick_sys::ExceptionType_MissingDelegateWarning, MissingDelegateWarning);
    (graphicsmagick_sys::ExceptionType_CorruptImageWarning, CorruptImageWarning);
    (graphicsmagick_sys::ExceptionType_FileOpenWarning, FileOpenWarning);
    (graphicsmagick_sys::ExceptionType_BlobWarning, BlobWarning);
    (graphicsmagick_sys::ExceptionType_StreamWarning, StreamWarning);
    (graphicsmagick_sys::ExceptionType_CacheWarning, CacheWarning);
    (graphicsmagick_sys::ExceptionType_CoderWarning, CoderWarning);
    (graphicsmagick_sys::ExceptionType_ModuleWarning, ModuleWarning);
    (graphicsmagick_sys::ExceptionType_DrawWarning, DrawWarning);
    (graphicsmagick_sys::ExceptionType_RenderWarning, RenderWarning);
    (graphicsmagick_sys::ExceptionType_ImageWarning, ImageWarning);
    (graphicsmagick_sys::ExceptionType_WandWarning, WandWarning);
    (graphicsmagick_sys::ExceptionType_TemporaryFileWarning, TemporaryFileWarning);
    (graphicsmagick_sys::ExceptionType_TransformWarning, TransformWarning);
    (graphicsmagick_sys::ExceptionType_XServerWarning, XServerWarning);
    (graphicsmagick_sys::ExceptionType_X11Warning, X11Warning);
    (graphicsmagick_sys::ExceptionType_UserWarning, UserWarning);
    (graphicsmagick_sys::ExceptionType_MonitorWarning, MonitorWarning);
    (graphicsmagick_sys::ExceptionType_LocaleWarning, LocaleWarning);
    (graphicsmagick_sys::ExceptionType_DeprecateWarning, DeprecateWarning);
    (graphicsmagick_sys::ExceptionType_RegistryWarning, RegistryWarning);
    (graphicsmagick_sys::ExceptionType_ConfigureWarning, ConfigureWarning);
    (graphicsmagick_sys::ExceptionType_ErrorException, ErrorException);
    (graphicsmagick_sys::ExceptionType_ExceptionError, ExceptionError);
    (graphicsmagick_sys::ExceptionType_ResourceError, ResourceError);
    (graphicsmagick_sys::ExceptionType_ResourceLimitError, ResourceLimitError);
    (graphicsmagick_sys::ExceptionType_TypeError, TypeError);
    (graphicsmagick_sys::ExceptionType_AnnotateError, AnnotateError);
    (graphicsmagick_sys::ExceptionType_OptionError, OptionError);
    (graphicsmagick_sys::ExceptionType_DelegateError, DelegateError);
    (graphicsmagick_sys::ExceptionType_MissingDelegateError, MissingDelegateError);
    (graphicsmagick_sys::ExceptionType_CorruptImageError, CorruptImageError);
    (graphicsmagick_sys::ExceptionType_FileOpenError, FileOpenError);
    (graphicsmagick_sys::ExceptionType_BlobError, BlobError);
    (graphicsmagick_sys::ExceptionType_StreamError, StreamError);
    (graphicsmagick_sys::ExceptionType_CacheError, CacheError);
    (graphicsmagick_sys::ExceptionType_CoderError, CoderError);
    (graphicsmagick_sys::ExceptionType_ModuleError, ModuleError);
    (graphicsmagick_sys::ExceptionType_DrawError, DrawError);
    (graphicsmagick_sys::ExceptionType_RenderError, RenderError);
    (graphicsmagick_sys::ExceptionType_ImageError, ImageError);
    (graphicsmagick_sys::ExceptionType_WandError, WandError);
    (graphicsmagick_sys::ExceptionType_TemporaryFileError, TemporaryFileError);
    (graphicsmagick_sys::ExceptionType_TransformError, TransformError);
    (graphicsmagick_sys::ExceptionType_XServerError, XServerError);
    (graphicsmagick_sys::ExceptionType_X11Error, X11Error);
    (graphicsmagick_sys::ExceptionType_UserError, UserError);
    (graphicsmagick_sys::ExceptionType_MonitorError, MonitorError);
    (graphicsmagick_sys::ExceptionType_LocaleError, LocaleError);
    (graphicsmagick_sys::ExceptionType_DeprecateError, DeprecateError);
    (graphicsmagick_sys::ExceptionType_RegistryError, RegistryError);
    (graphicsmagick_sys::ExceptionType_ConfigureError, ConfigureError);
    (graphicsmagick_sys::ExceptionType_FatalErrorException, FatalErrorException);
    (graphicsmagick_sys::ExceptionType_ExceptionFatalError, ExceptionFatalError);
    (graphicsmagick_sys::ExceptionType_ResourceFatalError, ResourceFatalError);
    (graphicsmagick_sys::ExceptionType_ResourceLimitFatalError, ResourceLimitFatalError);
    (graphicsmagick_sys::ExceptionType_TypeFatalError, TypeFatalError);
    (graphicsmagick_sys::ExceptionType_AnnotateFatalError, AnnotateFatalError);
    (graphicsmagick_sys::ExceptionType_OptionFatalError, OptionFatalError);
    (graphicsmagick_sys::ExceptionType_DelegateFatalError, DelegateFatalError);
    (graphicsmagick_sys::ExceptionType_MissingDelegateFatalError, MissingDelegateFatalError);
    (graphicsmagick_sys::ExceptionType_CorruptImageFatalError, CorruptImageFatalError);
    (graphicsmagick_sys::ExceptionType_FileOpenFatalError, FileOpenFatalError);
    (graphicsmagick_sys::ExceptionType_BlobFatalError, BlobFatalError);
    (graphicsmagick_sys::ExceptionType_StreamFatalError, StreamFatalError);
    (graphicsmagick_sys::ExceptionType_CacheFatalError, CacheFatalError);
    (graphicsmagick_sys::ExceptionType_CoderFatalError, CoderFatalError);
    (graphicsmagick_sys::ExceptionType_ModuleFatalError, ModuleFatalError);
    (graphicsmagick_sys::ExceptionType_DrawFatalError, DrawFatalError);
    (graphicsmagick_sys::ExceptionType_RenderFatalError, RenderFatalError);
    (graphicsmagick_sys::ExceptionType_ImageFatalError, ImageFatalError);
    (graphicsmagick_sys::ExceptionType_WandFatalError, WandFatalError);
    (graphicsmagick_sys::ExceptionType_TemporaryFileFatalError, TemporaryFileFatalError);
    (graphicsmagick_sys::ExceptionType_TransformFatalError, TransformFatalError);
    (graphicsmagick_sys::ExceptionType_XServerFatalError, XServerFatalError);
    (graphicsmagick_sys::ExceptionType_X11FatalError, X11FatalError);
    (graphicsmagick_sys::ExceptionType_UserFatalError, UserFatalError);
    (graphicsmagick_sys::ExceptionType_MonitorFatalError, MonitorFatalError);
    (graphicsmagick_sys::ExceptionType_LocaleFatalError, LocaleFatalError);
    (graphicsmagick_sys::ExceptionType_DeprecateFatalError, DeprecateFatalError);
    (graphicsmagick_sys::ExceptionType_RegistryFatalError, RegistryFatalError);
    (graphicsmagick_sys::ExceptionType_ConfigureFatalError, ConfigureFatalError);
}
