//! conforms: spu-header-read-touches-no-device
//! conforms: spu-devices-from-the-binding
//! conforms: spu-admission-judges-room-reach-and-width
//! conforms: spu-device-authority-is-the-driver
//! conforms: spu-headroom-is-a-construction-parameter
//! conforms: spu-weights-hash-at-admit
//! conforms: spu-hash-failure-sentinel
//! conforms: spu-release-frees-before-answering
//! conforms: spu-no-idempotence-no-retry
//!
//! Admit and release, per `weaver-spu-Spec` section 3.
//!
//! Admit runs the charter's five steps and the first three are free: resolve
//! the binding to an artifact, read what the artifact declares about itself
//! without loading it, judge the assigned devices, take them in shard order and
//! load each shard, confirm. Every step before the fourth is refusable at no
//! cost, which converts the common shape of a bad binding, an artifact present
//! and wrong, into a refusal costing no device work.
//!
//! **The devices are the binding's and this crate selects none.** No device
//! survey, no ranking, no fallback: the archived tree's `auto_select_gpu` does
//! not cross.
//!
//! **Nothing is idempotent and nothing retries.** This crate begins empty,
//! admits once, and dies, so a second admit has no prior residency to match.

use std::path::{Path, PathBuf};

use weaver_types::{DeviceOrdinal, LifecycleRefusal, ModelBinding};

use crate::artifact::{self, ArtifactHeader};
use crate::family::{self, FamilyRefusal};

/// The headroom term, in bytes, held beside the shard on each assigned device.
///
/// **A construction parameter until a measurement replaces it.** Charter
/// section 9 stages the figure and names the entry condition, a measurement on
/// a real artifact against a real device, so the admission inequality takes the
/// headroom from the worker's composition root rather than from an operator
/// election. A builder supplies it before the measurement exists.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Headroom(pub u64);

/// The weights hash that travels with every measurement.
///
/// The sentinel is the empty string, on every failure path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeightsHash(pub String);

impl WeightsHash {
    /// The sentinel: a hash that could not be computed. A hash that cannot be
    /// computed reports that it could not rather than reporting a wrong value,
    /// and apex section 8 rests replay on the identity being right.
    pub fn sentinel() -> Self {
        WeightsHash(String::new())
    }

    pub fn is_sentinel(&self) -> bool {
        self.0.is_empty()
    }
}

/// Why an admit refused, with the step that refused it legible.
///
/// The step identity is the point: **section 10 buys the ordering rather than
/// the parsing**, and a test that could not tell which step refused would pass
/// with the whole ordering inverted.
#[derive(Debug, Clone, PartialEq)]
pub enum AdmitRefusal {
    /// Step one: the binding did not resolve to an artifact.
    Unresolvable,
    /// Step two: the artifact is present and its header could not be read.
    Unreadable,
    /// Step three, on the condition that reads nothing. Carries the family,
    /// because the no-silent-substitution test reads the name.
    Family(FamilyRefusal),
    /// Step three, on a condition that reads the driver.
    Device,
    /// A second admit. This crate admits once.
    AlreadyAttempted,
}

impl From<AdmitRefusal> for LifecycleRefusal {
    fn from(refusal: AdmitRefusal) -> Self {
        match refusal {
            AdmitRefusal::Unresolvable => LifecycleRefusal::ArtifactUnresolvable,
            AdmitRefusal::Unreadable => LifecycleRefusal::ArtifactUnreadable,
            AdmitRefusal::Family(inner) => inner.into(),
            AdmitRefusal::Device => LifecycleRefusal::DeviceCannotAdmit,
            AdmitRefusal::AlreadyAttempted => LifecycleRefusal::OutOfOrder,
        }
    }
}

/// What this process holds after a successful admit.
#[derive(Debug)]
pub struct Resident {
    pub artifact: PathBuf,
    pub header: ArtifactHeader,
    pub devices: Vec<DeviceOrdinal>,
    pub weights_hash: WeightsHash,
}

/// The residency, which begins empty and admits once.
#[derive(Debug, Default)]
pub struct Residency {
    resident: Option<Resident>,
    /// Set once an admit has been attempted, successfully or not. Once-only is
    /// a property of the process rather than of whether the first attempt
    /// happened to succeed.
    admit_attempted: bool,
}

impl Residency {
    pub fn new() -> Self {
        Residency::default()
    }

    pub fn resident(&self) -> Option<&Resident> {
        self.resident.as_ref()
    }

    /// Run the charter's five steps in order.
    pub fn admit(
        &mut self,
        binding: &ModelBinding,
        headroom: Headroom,
    ) -> Result<&Resident, AdmitRefusal> {
        if self.admit_attempted {
            return Err(AdmitRefusal::AlreadyAttempted);
        }
        self.admit_attempted = true;

        // Step one. Resolve the binding to an artifact. Free.
        let path = artifact::resolve(&binding.artifact).map_err(|_| AdmitRefusal::Unresolvable)?;

        // Step two. Read what the artifact declares about itself, without
        // loading it and without touching a device. Free.
        let header = artifact::read_header(&path).map_err(|_| AdmitRefusal::Unreadable)?;

        // Step three, cheapest first. The width condition is a comparison
        // between the binding's count and the family's declaration and reads
        // nothing; the room and reach conditions each cost a driver query. A
        // set failing more than one condition refuses on the cheapest, which is
        // what puts the width refusal inside a test on a machine with no device.
        family::judge_width(&header.family, binding.devices.len() as u32)
            .map_err(AdmitRefusal::Family)?;
        judge_room_and_reach(&binding.devices, &header, headroom)?;

        // Step four. Take the devices in shard order and load each shard. The
        // binding's order is the shard order.
        load_shards(&path, &binding.devices)?;

        // The weights hash is computed at admit from the bytes this process
        // loaded rather than from a manifest handed to it, and computed fresh
        // with no cache across an artifact change.
        let weights_hash = artifact::weights_hash(&path);

        // Step five. Confirm.
        self.resident = Some(Resident {
            artifact: path,
            header,
            devices: binding.devices.clone(),
            weights_hash,
        });
        Ok(self.resident.as_ref().expect("set immediately above"))
    }

    /// Free the device, then answer.
    ///
    /// Stop serving, free the weights and the working allocations and the cache
    /// together because they are one residency, then confirm. A confirmation is
    /// a fact about the device rather than a statement of intent, and the
    /// archived tree's inverse ordering is what its own record names as
    /// producing an overcommit. The free happens inside this function, so the
    /// caller cannot answer before it: the ordering is by construction rather
    /// than by a comment asking the caller to wait.
    pub fn release(&mut self) -> Result<(), LifecycleRefusal> {
        let Some(resident) = self.resident.take() else {
            return Err(LifecycleRefusal::NoResidency);
        };
        free_shards(&resident);
        drop(resident);
        Ok(())
    }
}

/// The room and reach conditions, each of which costs a driver query.
///
/// **The device judgment reads the driver rather than this crate's own
/// accounting.** This crate holds no fleet ledger to prefer, and the case the
/// check exists to catch is a device occupied by something this program did not
/// put there, so the authority is what the device reports free at the moment of
/// admission. Each assigned device must have room for its shard plus the
/// residency's headroom, the one inequality read per device, and the devices
/// must be able to reach each other, because a sharded forward exchanges
/// activations across them and a set without peer access cannot serve.
#[cfg(not(feature = "cuda"))]
fn judge_room_and_reach(
    _devices: &[DeviceOrdinal],
    _header: &ArtifactHeader,
    _headroom: Headroom,
) -> Result<(), AdmitRefusal> {
    // No driver is compiled in, so there is no device this build can admit to.
    // Refusing here rather than pretending the conditions passed is what keeps
    // the no-feature build from claiming a residency it cannot hold.
    Err(AdmitRefusal::Device)
}

#[cfg(not(feature = "cuda"))]
fn load_shards(_path: &Path, _devices: &[DeviceOrdinal]) -> Result<(), AdmitRefusal> {
    Err(AdmitRefusal::Device)
}

#[cfg(not(feature = "cuda"))]
fn free_shards(_resident: &Resident) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::family::FamilyName;
    use weaver_types::ArtifactRef;

    /// **A second admit refuses on the ordering rather than re-running the
    /// steps.** Nothing is idempotent: this crate begins empty, admits once,
    /// and dies.
    ///
    /// Perturbation: remove the `admit_attempted` guard from `admit` and this
    /// test fails, because the second call runs step one again and refuses as
    /// `Unresolvable` rather than as `AlreadyAttempted`. Watched under exactly
    /// that removal.
    #[test]
    fn a_second_admit_refuses_on_the_ordering() {
        let mut residency = Residency::new();
        let binding = ModelBinding {
            artifact: ArtifactRef("/nonexistent/artifact".into()),
            devices: vec![DeviceOrdinal(0)],
        };
        let first = residency.admit(&binding, Headroom(0)).err();
        assert_eq!(
            first,
            Some(AdmitRefusal::Unresolvable),
            "the fixture refuses at step one"
        );
        let second = residency.admit(&binding, Headroom(0)).err();
        assert_eq!(
            second,
            Some(AdmitRefusal::AlreadyAttempted),
            "the second admit refuses on the ordering, not on the artifact"
        );
    }

    /// **Release with nothing resident refuses.**
    #[test]
    fn release_without_residency_refuses() {
        let mut residency = Residency::new();
        assert_eq!(residency.release(), Err(LifecycleRefusal::NoResidency));
    }

    /// The floor's refusal set carries no family-naming case, so the name is
    /// lost at the boundary by design and kept on this side of it. This pins
    /// that the conversion is total rather than lossy in some other way.
    #[test]
    fn a_family_refusal_crosses_the_seam_as_a_floor_case() {
        let refusal = AdmitRefusal::Family(FamilyRefusal::UnknownFamily(FamilyName("x".into())));
        assert_eq!(
            LifecycleRefusal::from(refusal),
            LifecycleRefusal::ArtifactUnreadable
        );
    }
}
