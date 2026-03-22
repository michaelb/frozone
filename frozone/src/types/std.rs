use crate::{
    Freezable,
    types::{assume_frozen, container_derive_impl},
};

use std::ffi::{OsStr, OsString};
use std::fs::{DirBuilder, DirEntry, File, FileType, Metadata, OpenOptions, Permissions, ReadDir};
use std::io::{Error, ErrorKind, Repeat, Sink, Stderr, Stdin, Stdout};
use std::net::{Incoming, TcpListener, TcpStream, UdpSocket};
use std::path::{Component, Iter, Path, PathBuf, Prefix, PrefixComponent};
use std::process::{
    Child, ChildStderr, ChildStdin, ChildStdout, Command, ExitCode, ExitStatus, Output, Stdio,
};
use std::sync::{
    Barrier, BarrierWaitResult, Condvar, Mutex, MutexGuard, Once, OnceState, RwLock,
    RwLockReadGuard, RwLockWriteGuard, WaitTimeoutResult,
};

use std::thread::{Builder, JoinHandle, LocalKey, Thread, ThreadId};
use std::time::SystemTime;

container_derive_impl!(
    Mutex<T>,
    MutexGuard<'_, T>,
    RwLock<T>,
    RwLockReadGuard<'_, T>,
    RwLockWriteGuard<'_, T>,
    JoinHandle<T>,
    LocalKey<T>
);

assume_frozen!(
    OsStr,
    OsString,
    DirBuilder,
    DirEntry,
    File,
    FileType,
    Metadata,
    OpenOptions,
    Permissions,
    ReadDir,
    Error,
    ErrorKind,
    Stdin,
    Stderr,
    Stdout,
    Repeat,
    Sink,
    TcpListener,
    TcpStream,
    UdpSocket,
    Incoming<'_>,
    Path,
    PathBuf,
    Component<'_>,
    Iter<'_>,
    Prefix<'_>,
    PrefixComponent<'_>,
    ChildStdout,
    ChildStderr,
    ChildStdin,
    Child,
    Command,
    ExitStatus,
    ExitCode,
    Stdio,
    Output,
    Barrier,
    BarrierWaitResult,
    Condvar,
    Once,
    OnceState,
    WaitTimeoutResult,
    Builder,
    Thread,
    ThreadId,
    SystemTime
);
