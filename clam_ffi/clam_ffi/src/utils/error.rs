#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum FFIError {
    /// All went fine.
    Ok,

    /// Naughty API call detected.
    NullPointerPassed = 1,
    InvalidStringPassed = 2,
    HandleInitFailed,
    LoadTreeFailed,
    GraphBuildFailed,
    QueryIsNull,
    PhysicsAlreadyShutdown,
    DivisionByZero,
    PhysicsRunning,
    PhysicsFinished,
    PhysicsNotReady,
    StartupDataInvalid,
    SaveFailed,
    UnsupportedMetric,
    ScoringFunctionNotFound,
    PathNotFound,
    NotInCache,
    TooManyLabels,
    ColoringFailed,
    NotImplemented,
}
