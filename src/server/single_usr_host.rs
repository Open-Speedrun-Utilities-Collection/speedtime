// Speedrun Timing Server
// Backend agnostic interface over stopwatch timing and inter-process communication

/// Simple backend host for single user single protocol use.
///
/// Note:
///  - Only allows one backend transfer protocal
///  - Only allows one connection per backend
///
/// Primarily intended for creating local timing utilities.
pub struct SimpleBackendHost;
