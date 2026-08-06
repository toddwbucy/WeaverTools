//! conforms: spu-shard-widths-are-a-set
//! conforms: spu-widths-set-pinned-by-doctest
//! conforms: spu-registry-no-silent-substitution
//!
//! The family surface and its registry, per `weaver-spu-Spec` section 5.
//!
//! **The registry is compile-time and admission consults it.** A family the
//! binary does not carry is a refused admit **naming the family**, which is the
//! archived tree's own no-silent-substitution ruling carried forward from its
//! encoder registry. Nothing here falls back to a nearest match: a substitution
//! that succeeds quietly is how a model runs under the wrong template.
//!
//! **The shard widths are a set rather than a maximum.** The field's type is a
//! set, so a maximum can no longer be declared, only read wrongly. The doctest
//! below reads a declaration carrying a non-contiguous set literal, which is
//! what makes the type unable to express the maximum it replaced:
//!
//! ```
//! use weaver_spu::family::Declaration;
//! // A non-contiguous set: this backend shards across one device or four, and
//! // not across two or three. No maximum describes that, which is the point.
//! const SPARSE: Declaration = Declaration {
//!     family: "sparse-example",
//!     shard_widths: &[1, 4],
//!     template: "{message}",
//! };
//! assert!(SPARSE.shards_across(4));
//! assert!(!SPARSE.shards_across(2));
//! assert!(!SPARSE.shards_across(3));
//! ```

use weaver_types::LifecycleRefusal;

/// A family's name, as the artifact header declares it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FamilyName(pub String);

/// What one family declares about itself, at compile time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Declaration {
    /// The name an artifact header must carry to select this family.
    pub family: &'static str,
    /// **The widths this backend can shard across, as a set.** Membership is
    /// the test, never a comparison against a bound: a set is what lets a
    /// backend declare that it shards across one or four and not two.
    pub shard_widths: &'static [u32],
    /// The template this family renders the harness's canonical messages
    /// through.
    pub template: &'static str,
}

impl Declaration {
    /// Whether this backend shards across exactly this many devices.
    ///
    /// Membership rather than a bound. A wider set refuses against the
    /// declaration rather than against a hidden limit, so the day an N-way path
    /// lands the declaration changes and nothing else does.
    pub const fn shards_across(&self, width: u32) -> bool {
        let mut index = 0;
        while index < self.shard_widths.len() {
            if self.shard_widths[index] == width {
                return true;
            }
            index += 1;
        }
        false
    }
}

/// The compile-time table.
///
/// **Today the salvaged tensor-parallel path is a two-device implementation,**
/// `forward_tp2` with an all-reduce kernel written for a pair, so the declared
/// set is one or two. It is written as a set rather than as the maximum two so
/// that a three-device path arriving without a two-device path is expressible.
pub const REGISTRY: &[Declaration] = &[
    Declaration {
        family: "llama",
        shard_widths: &[1, 2],
        template: "<|start_header_id|>{role}<|end_header_id|>\n\n{message}<|eot_id|>",
    },
    Declaration {
        family: "qwen2",
        shard_widths: &[1, 2],
        template: "<|im_start|>{role}\n{message}<|im_end|>\n",
    },
    Declaration {
        family: "gptoss",
        shard_widths: &[1, 2],
        template: "<|start|>{role}<|message|>{message}<|end|>",
    },
];

/// Why a family lookup or a width judgment refused.
///
/// This is the crate's own vocabulary rather than the floor's, because the
/// floor's refusal set carries no case that names a family, and **what the
/// no-silent-substitution test reads is the family the refusal names.** A
/// refusal arriving on some other ground does not satisfy it, so the name has to
/// survive to the place the test reads.
#[derive(Debug, Clone, PartialEq)]
pub enum FamilyRefusal {
    /// The artifact header names a family this binary does not carry. The name
    /// travels with the refusal rather than being flattened to a generic
    /// unreadable, which is what makes the substitution visible.
    UnknownFamily(FamilyName),
    /// The family is carried, but it does not shard across the requested width.
    WidthNotDeclared {
        family: FamilyName,
        requested: u32,
        declared: &'static [u32],
    },
}

impl From<FamilyRefusal> for LifecycleRefusal {
    /// What crosses the seam. The floor's set is closed at `weaver-types` and
    /// carries no family-naming case, so the name is lost at the boundary and
    /// kept on this side of it. Both cases are admission judgments about a
    /// device set or an artifact this binary cannot serve.
    fn from(refusal: FamilyRefusal) -> Self {
        match refusal {
            FamilyRefusal::UnknownFamily(_) => LifecycleRefusal::ArtifactUnreadable,
            FamilyRefusal::WidthNotDeclared { .. } => LifecycleRefusal::DeviceCannotAdmit,
        }
    }
}

/// Look a family up in the compile-time table.
///
/// **No silent substitution.** A miss is a refusal naming the family, never a
/// nearest match and never a default declaration.
pub fn lookup(name: &FamilyName) -> Result<&'static Declaration, FamilyRefusal> {
    REGISTRY
        .iter()
        .find(|declaration| declaration.family == name.0)
        .ok_or_else(|| FamilyRefusal::UnknownFamily(name.clone()))
}

/// Judge a requested shard width against what the family declares.
///
/// The width condition reads nothing, which is why Spec section 3 judges it
/// before the room and reach conditions that each cost a driver query.
pub fn judge_width(name: &FamilyName, requested: u32) -> Result<(), FamilyRefusal> {
    let declaration = lookup(name)?;
    if declaration.shards_across(requested) {
        return Ok(());
    }
    Err(FamilyRefusal::WidthNotDeclared {
        family: name.clone(),
        requested,
        declared: declaration.shard_widths,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **A family the binary does not carry is refused, and the refusal names
    /// the family.** What this test reads is the name, not the load's outcome,
    /// so a refusal arriving on some other ground does not satisfy it.
    ///
    /// Perturbation: make `lookup` fall back to `REGISTRY[0]` on a miss and
    /// this test fails, because the lookup then succeeds and no refusal is
    /// produced at all. Watched under exactly that substitution.
    #[test]
    fn an_uncarried_family_refuses_by_name() {
        let absent = FamilyName("not-a-family-this-binary-carries".into());
        assert_eq!(
            lookup(&absent),
            Err(FamilyRefusal::UnknownFamily(absent.clone())),
            "the refusal carries the family the header named"
        );
    }

    /// **The width is judged by membership rather than against a bound.**
    ///
    /// Perturbation: change `shards_across` to `width <= max(shard_widths)` and
    /// this test still passes on the registry's contiguous sets, which is why
    /// the non-contiguous case is asserted here as well as pinned by the
    /// module doctest. Under that change the sparse assertion below fails.
    #[test]
    fn a_width_outside_the_declared_set_refuses() {
        let llama = FamilyName("llama".into());
        assert_eq!(judge_width(&llama, 1), Ok(()));
        assert_eq!(judge_width(&llama, 2), Ok(()));
        assert!(matches!(
            judge_width(&llama, 3),
            Err(FamilyRefusal::WidthNotDeclared { requested: 3, .. })
        ));

        // The membership property, on a set no maximum describes.
        const SPARSE: Declaration = Declaration {
            family: "sparse",
            shard_widths: &[1, 4],
            template: "{message}",
        };
        assert!(SPARSE.shards_across(1));
        assert!(!SPARSE.shards_across(2), "a bound would admit this");
        assert!(!SPARSE.shards_across(3), "a bound would admit this");
        assert!(SPARSE.shards_across(4));
    }
}
