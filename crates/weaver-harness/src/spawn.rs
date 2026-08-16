//! conforms: harness-fork-to-exec-three-calls
//! conforms: harness-organ-argv-carries-construction-parameters
//! conforms: harness-organ-ends-from-descriptor-three
//! conforms: harness-child-flag-clear-unconditional
//!
//! The organ fork, per `weaver-harness-Spec` section 2.2. Between fork and
//! exec the child performs three calls, `dup2`, `fcntl`, and `execve`, and
//! nothing else: all three are async-signal-safe, and the bound is the safety
//! argument rather than a style, because the worker holds the writer's thread
//! at every fork and a child of a multithreaded process may safely run only
//! async-signal-safe calls before its exec.

use std::ffi::CString;
use std::path::Path;

use crate::channel::{ChildEnd, place_child_ends};

/// Forks an organ binary, handing it the ends it is owed at descriptor 3
/// upward in the channels' own order: the lifecycle channel every organ holds
/// takes 3, and an organ's further channel takes the next number, so the
/// gate's single end sits where the SPU's first does.
///
/// # Safety
///
/// The caller must be the serving thread, whose lifetime is the worker's: the
/// gate's parent-death backing fires on the forking thread's termination
/// rather than the process's, so a fork from a short-lived thread would kill
/// the gate spuriously while the interior it guards is healthy.
pub unsafe fn fork_organ(
    binary: &Path,
    ends: &[&ChildEnd],
    arguments: &[String],
) -> Result<nix::unistd::Pid, nix::Error> {
    let program =
        CString::new(binary.as_os_str().as_encoded_bytes()).map_err(|_| nix::Error::EINVAL)?;
    // **Built here, in the parent, before the fork.** The child's three calls
    // are the safety bound of the doc comment above, so a vector assembled
    // after the fork would not be expressible: allocation is not
    // async-signal-safe. The pointers below borrow these, so both outlive the
    // exec that reads them.
    let arguments: Vec<CString> = arguments
        .iter()
        .map(|argument| CString::new(argument.as_bytes()).map_err(|_| nix::Error::EINVAL))
        .collect::<Result<_, _>>()?;
    let mut argv: Vec<*const nix::libc::c_char> = Vec::with_capacity(arguments.len() + 2);
    argv.push(program.as_ptr());
    argv.extend(arguments.iter().map(|argument| argument.as_ptr()));
    argv.push(std::ptr::null());
    // SAFETY: the caller's contract above, and the child body below runs only
    // async-signal-safe calls.
    match unsafe { nix::unistd::fork()? } {
        nix::unistd::ForkResult::Parent { child } => Ok(child),
        nix::unistd::ForkResult::Child => {
            // Call one and two, per the enumeration: dup2 each end into place
            // and clear the flag on each unconditionally.
            // SAFETY: in the child, before exec, with ends this process made.
            //
            // **A failed placement does not reach the exec.** The child cannot
            // return an error to the parent, but it can refuse to run the
            // organ: an organ started with no channel at descriptor 3 would
            // report as a residency problem at the parent's first exchange
            // rather than as the placement fault it is. The distinct status
            // is what lets the parent tell the two apart.
            if unsafe { place_child_ends(ends) }.is_err() {
                // SAFETY: _exit is async-signal-safe and does not unwind.
                unsafe { nix::libc::_exit(PLACEMENT_FAILED) };
            }
            // Call three.
            let envp = [std::ptr::null()];
            // SAFETY: execve is async-signal-safe; on success it does not
            // return, and on failure the child exits without unwinding.
            unsafe {
                nix::libc::execve(program.as_ptr(), argv.as_ptr(), envp.as_ptr());
                nix::libc::_exit(EXEC_FAILED);
            }
        }
    }
}

/// The child exited because it could not place the ends it was handed, which
/// is a placement fault and not a residency one.
pub const PLACEMENT_FAILED: i32 = 126;

/// The child exited because the organ binary could not be exec'd.
pub const EXEC_FAILED: i32 = 127;
