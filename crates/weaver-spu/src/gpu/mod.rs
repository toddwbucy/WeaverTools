//! conforms: spu-device-authority-is-the-driver
//!
//! The device queries the admission judgment reads, per `weaver-spu-Spec`
//! section 3.
//!
//! **The device judgment reads the driver rather than this crate's own
//! accounting.** Charter section 10 held the question open with the archived
//! tree's answer recorded as a preference without a reason: that tree held its
//! own allocation ledger as the authority and marked the driver query as
//! diagnostics only, which prefers the number that cannot see the thing the
//! check exists for. This crate holds no fleet ledger to prefer, and the case
//! it exists to catch is a device occupied by something this program did not
//! put there, so the authority is what the device reports free at the moment of
//! admission.
//!
//! **The cost is named rather than hidden:** the queries below run on the admit
//! path, which happens twice per residency and never inside a turn.
//!
//! This module compiles only under the `cuda` feature. The no-feature build
//! reaches none of it, which is what keeps the family surface testable on a
//! machine with no device.

use std::sync::Arc;

use cudarc::driver::{CudaContext, sys};
use weaver_types::DeviceOrdinal;

/// Why the driver refused a device set.
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceRefusal {
    /// The ordinal names no device, or the driver would not open it.
    Unreachable { ordinal: u32 },
    /// The one inequality, read per device: room for this device's shard plus
    /// the residency's headroom.
    NoRoom {
        ordinal: u32,
        free: u64,
        needed: u64,
    },
    /// A sharded forward exchanges activations across the set, so a set without
    /// peer access is a set that cannot serve. Discovered at admit rather than
    /// at the first turn.
    NoPeerAccess { from: u32, to: u32 },
}

/// Judge room and reach across the assigned set.
///
/// Room first, per device, then reach across each pair. A single-device set is
/// judged on room alone: there is no pair to exchange activations across, so
/// asking the driver about peer access would be asking a question the topology
/// does not pose.
pub fn room_and_reach(
    devices: &[DeviceOrdinal],
    shard_bytes: u64,
    headroom_bytes: u64,
) -> Result<(), DeviceRefusal> {
    let mut contexts = Vec::with_capacity(devices.len());
    for device in devices {
        let context = CudaContext::new(device.0 as usize)
            .map_err(|_| DeviceRefusal::Unreachable { ordinal: device.0 })?;
        contexts.push((device.0, context));
    }

    let needed = shard_bytes.saturating_add(headroom_bytes);
    for (ordinal, context) in &contexts {
        let (free, _total) = context
            .mem_get_info()
            .map_err(|_| DeviceRefusal::Unreachable { ordinal: *ordinal })?;
        let free = free as u64;
        if free < needed {
            return Err(DeviceRefusal::NoRoom {
                ordinal: *ordinal,
                free,
                needed,
            });
        }
    }

    // Reach, across each ordered pair. Peer access is not guaranteed symmetric
    // by the driver's interface, so both directions are asked rather than one
    // being inferred from the other.
    for (from_ordinal, from_context) in &contexts {
        for (to_ordinal, to_context) in &contexts {
            if from_ordinal == to_ordinal {
                continue;
            }
            if !can_access_peer(from_context, to_context) {
                return Err(DeviceRefusal::NoPeerAccess {
                    from: *from_ordinal,
                    to: *to_ordinal,
                });
            }
        }
    }

    Ok(())
}

/// Ask the driver whether peer access holds from one device to another.
///
/// `cuDeviceCanAccessPeer` has no safe wrapper in this cudarc line, so the call
/// is made directly. A driver error is read as no access rather than
/// propagated: the question asked is whether the set can serve, and a driver
/// that cannot answer it has not said yes.
fn can_access_peer(from: &Arc<CudaContext>, to: &Arc<CudaContext>) -> bool {
    let mut answer: std::ffi::c_int = 0;
    let status =
        unsafe { sys::cuDeviceCanAccessPeer(&mut answer, from.cu_device(), to.cu_device()) };
    status == sys::CUresult::CUDA_SUCCESS && answer != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// These run against whatever device this machine has, and skip where it
    /// has none. A skip is printed rather than silent, because a suite that
    /// reports success for tests it did not run is the failure mode this
    /// corpus treats as worse than a red result.
    fn device_present() -> bool {
        CudaContext::new(0).is_ok()
    }

    /// **The authority is what the device reports free at the moment of
    /// admission.** A shard that plainly fits is admitted, which is the
    /// positive half without which the refusal below would pass against a
    /// function that refused unconditionally.
    #[test]
    fn a_shard_that_fits_is_admitted() {
        if !device_present() {
            eprintln!("SKIP a_shard_that_fits_is_admitted: no device on this machine");
            return;
        }
        assert_eq!(
            room_and_reach(&[DeviceOrdinal(0)], 1024 * 1024, 1024 * 1024),
            Ok(())
        );
    }

    /// The one inequality, read per device. An absurd shard refuses on room and
    /// the refusal names the device and both numbers, so an operator meeting it
    /// knows whether to free a card or reassign the binding.
    ///
    /// Perturbation: change the `free < needed` comparison in `room_and_reach`
    /// to `free < shard_bytes`, dropping the headroom term, and this test still
    /// passes, which is why the headroom's presence is argued at its own clause
    /// rather than rested on this test. Deleting the comparison entirely fails
    /// it. Watched under that deletion.
    #[test]
    fn a_shard_larger_than_the_device_refuses_on_room() {
        if !device_present() {
            eprintln!("SKIP a_shard_larger_than_the_device_refuses_on_room: no device");
            return;
        }
        let absurd = 1024u64 * 1024 * 1024 * 1024; // a terabyte
        match room_and_reach(&[DeviceOrdinal(0)], absurd, 0) {
            Err(DeviceRefusal::NoRoom {
                ordinal, needed, ..
            }) => {
                assert_eq!(ordinal, 0);
                assert_eq!(needed, absurd);
            }
            other => panic!("a terabyte shard refuses on room, got {other:?}"),
        }
    }

    /// An ordinal naming no device refuses as unreachable rather than as a room
    /// condition, because the two send an operator to different places.
    #[test]
    fn an_ordinal_naming_no_device_refuses_as_unreachable() {
        let absent = DeviceOrdinal(250);
        assert_eq!(
            room_and_reach(&[absent], 1024, 0),
            Err(DeviceRefusal::Unreachable { ordinal: 250 })
        );
    }
}
