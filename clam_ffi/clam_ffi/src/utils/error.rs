#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum FFIError {
    /// All went fine.
    Ok,

    /// Naughty API call detected.
    NullPointerPassed = 1,
    InvalidStringPassed = 2,
    HandleInitFailed = 3,
    GraphBuildFailed = 4,
    QueryIsNull,
    PhysicsAlreadyShutdown,
    DivisionByZero,
    PhysicsRunning,
    PhysicsFinished,
    PhysicsNotReady,
    StartupDataInvalid,
    SaveFailed,
    UnsupportedMetric,
    PathNotFound,
    NotInCache,
}
