//! conforms: spu-header-read-touches-no-device
//! conforms: spu-devices-from-the-binding
//! conforms: spu-admission-judges-room-reach-and-width
//! conforms: spu-headroom-is-a-construction-parameter
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
    /// Step two: the artifact is present and could not be read, its header or
    /// its size on disk. Both reads are of the artifact and both are free.
    Unreadable,
    /// Step three, on the condition that reads nothing. Carries the family,
    /// because the no-silent-substitution test reads the name.
    Family(FamilyRefusal),
    /// Step three, on a condition that reads the driver, where no driver is
    /// compiled in to read.
    Device,
    /// Step three, on the set's own shape: an ordinal that appears twice. Two
    /// shards of one artifact on one card is not a two-device admission, and
    /// judging the pair as distinct would pass room on half the real demand and
    /// skip the peer check the pair would need. Free, so judged with the width.
    DuplicateDevice { ordinal: u32 },
    /// Step three, on a condition the driver answered. Carries which device and
    /// which condition, because an operator meeting this needs to know whether
    /// to free a card or to reassign the binding.
    #[cfg(feature = "cuda")]
    DeviceRefused(crate::gpu::DeviceRefusal),
    /// Step four, which is not written. Named distinctly so that a build with a
    /// device attached is not told it has a device problem.
    #[cfg(feature = "cuda")]
    BackendNotBuilt,
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
            AdmitRefusal::DuplicateDevice { .. } => LifecycleRefusal::DeviceCannotAdmit,
            #[cfg(feature = "cuda")]
            AdmitRefusal::DeviceRefused(_) => LifecycleRefusal::DeviceCannotAdmit,
            // The floor's closed set carries no case for a step this binary did
            // not build, and inventing one would be a floor edit this act has no
            // ruling for. It crosses as a device refusal, which is the nearest
            // true statement: this process cannot take the devices it was given.
            #[cfg(feature = "cuda")]
            AdmitRefusal::BackendNotBuilt => LifecycleRefusal::DeviceCannotAdmit,
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
        judge_distinct(&binding.devices)?;
        // The shard each device must hold, read from the filesystem rather than
        // declared by the artifact. The width was judged above, so the divisor
        // is known non-zero.
        let shard_bytes = artifact::on_disk_bytes(&path).ok_or(AdmitRefusal::Unreadable)?
            / binding.devices.len() as u64;
        judge_room_and_reach(&binding.devices, shard_bytes, headroom)?;

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

/// The set's ordinals must be distinct, judged with the free conditions.
fn judge_distinct(devices: &[DeviceOrdinal]) -> Result<(), AdmitRefusal> {
    for (index, device) in devices.iter().enumerate() {
        if devices[..index].contains(device) {
            return Err(AdmitRefusal::DuplicateDevice { ordinal: device.0 });
        }
    }
    Ok(())
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
    _shard_bytes: u64,
    _headroom: Headroom,
) -> Result<(), AdmitRefusal> {
    // No driver is compiled in, so there is no device this build can admit to.
    // Refusing here rather than pretending the conditions passed is what keeps
    // the no-feature build from claiming a residency it cannot hold.
    Err(AdmitRefusal::Device)
}

/// The room and reach conditions, asked of the driver.
///
/// The shard size is the artifact's bytes divided across the assigned set, read
/// from the filesystem at admit. It is an over-estimate of what a shard costs
/// in device memory for a quantized artifact and an under-estimate of the
/// working allocations beside it, which is why the headroom term sits in the
/// inequality and why charter section 9 stages a measurement to replace both.
#[cfg(feature = "cuda")]
fn judge_room_and_reach(
    devices: &[DeviceOrdinal],
    shard_bytes: u64,
    headroom: Headroom,
) -> Result<(), AdmitRefusal> {
    crate::gpu::room_and_reach(devices, shard_bytes, headroom.0)
        .map_err(AdmitRefusal::DeviceRefused)
}

#[cfg(not(feature = "cuda"))]
fn load_shards(_path: &Path, _devices: &[DeviceOrdinal]) -> Result<(), AdmitRefusal> {
    Err(AdmitRefusal::Device)
}

/// Step four: take the devices in shard order and load each shard.
///
/// **This step is not written and refuses rather than pretending.** Loading a
/// shard means driving a backend, and the backend seam is Spec section 4's, the
/// decode submodule, which this act has not reached. The refusal names that
/// rather than reporting a device condition, so an operator meeting it is told
/// what is missing instead of being sent to look at a card.
///
/// The three steps before this one are real on this build: a binding that
/// resolves wrongly, an artifact whose header will not read, a width outside
/// the family's declaration, and a device set without room or reach are each
/// refused here exactly as they would be once step four lands.
#[cfg(feature = "cuda")]
fn load_shards(_path: &Path, _devices: &[DeviceOrdinal]) -> Result<(), AdmitRefusal> {
    Err(AdmitRefusal::BackendNotBuilt)
}

#[cfg(not(feature = "cuda"))]
fn free_shards(_resident: &Resident) {}

/// Nothing is allocated on a device by this build, because step four refuses
/// before anything is taken, so there is nothing here to free. This is a
/// no-operation with a stated ground rather than an empty function.
#[cfg(feature = "cuda")]
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

    /// A GGUF whose header carries exactly the one key the family lookup
    /// needs, so an admit against it walks past steps one and two.
    fn llama_gguf() -> std::path::PathBuf {
        use std::io::Write;
        let dir = std::env::temp_dir().join(format!("weaver-spu-residency-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("a scratch dir");
        let path = dir.join("llama.gguf");
        let mut file = Vec::new();
        file.extend_from_slice(b"GGUF");
        file.extend_from_slice(&3u32.to_le_bytes());
        file.extend_from_slice(&0u64.to_le_bytes());
        file.extend_from_slice(&1u64.to_le_bytes());
        let key = b"general.architecture";
        file.extend_from_slice(&(key.len() as u64).to_le_bytes());
        file.extend_from_slice(key);
        file.extend_from_slice(&8u32.to_le_bytes());
        let value = b"llama";
        file.extend_from_slice(&(value.len() as u64).to_le_bytes());
        file.extend_from_slice(value);
        let mut handle = std::fs::File::create(&path).expect("a fixture file");
        handle.write_all(&file).expect("the fixture is written");
        path
    }

    /// **An ordinal that appears twice refuses on the set's shape, on the admit
    /// path.** Two shards of one artifact on one card is not a two-device
    /// admission: judged as distinct, the pair would pass room on half the real
    /// demand and skip the peer check.
    ///
    /// The fixture resolves and its header reads, so the admit walks past the
    /// free steps and the refusal read here is the shape judgment's own. A
    /// first version of this test called `judge_distinct` directly and could
    /// not fail under the call-site removal, which is why this one drives
    /// `admit`.
    ///
    /// Perturbation: remove the `judge_distinct` call from `admit` and this
    /// test fails, because the refusal becomes the driver condition's rather
    /// than the shape's. Watched under exactly that removal.
    #[test]
    fn a_duplicate_ordinal_refuses_on_the_admit_path() {
        let mut residency = Residency::new();
        let binding = ModelBinding {
            artifact: ArtifactRef(llama_gguf().to_string_lossy().into_owned()),
            devices: vec![DeviceOrdinal(0), DeviceOrdinal(0)],
        };
        assert_eq!(
            residency.admit(&binding, Headroom(0)).err(),
            Some(AdmitRefusal::DuplicateDevice { ordinal: 0 }),
            "the refusal is the set's shape, before any driver condition"
        );
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
