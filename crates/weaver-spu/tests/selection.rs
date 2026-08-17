//! The two-field key of `weaver-spu-Spec` section 5, against held artifacts.
//!
//! **These read real artifacts and are skipped where none is present**, which
//! is the pattern `markers.rs` uses: the claim is about what a shipped model
//! declares, and a synthetic header asserting it would be this crate agreeing
//! with itself.
#![cfg(feature = "gguf")]

use std::path::{Path, PathBuf};
use weaver_spu::artifact;
use weaver_spu::family::{self, FamilyName};

/// A held artifact, or `None` where this box does not carry it.
fn held(name: &str) -> Option<PathBuf> {
    let path = Path::new("/opt/weaver/models").join(name);
    path.is_file().then_some(path)
}

fn header_of(path: &Path) -> artifact::ArtifactHeader {
    let mut pinned = artifact::pin(path).expect("the artifact pins");
    artifact::read_header(&mut pinned).expect("the header reads")
}

/// **The case that opened #88 and closed as #175: an artifact declaring
/// `llama` and rendering ChatML.**
///
/// Before this act the architecture reached one entry, which handed SmolLM2
/// the Llama 3 stop set it was never trained against and refused the load. The
/// architecture now reaches two, and what separates them is what the artifact
/// renders.
#[test]
fn a_chatml_artifact_declaring_llama_selects_the_chatml_entry() {
    let Some(path) = held("smollm2-360m-instruct-q8_0.gguf") else {
        return;
    };
    let header = header_of(&path);
    assert_eq!(header.family, FamilyName("llama".into()));
    let template = header
        .chat_template
        .as_deref()
        .expect("the artifact declares a chat template");

    let selected = family::select(&header.family, Some(template)).expect("the artifact selects");
    assert!(
        selected.selecting_markers.contains(&"<|im_start|>"),
        "the ChatML entry is the one selected, not the Llama 3 entry"
    );
    assert!(
        !selected.selecting_markers.contains(&"<|begin_of_text|>"),
        "the Llama 3 stop set is what refused this artifact before the act"
    );
}

/// **A contested architecture with no template refuses rather than picking.**
///
/// The fact that would choose is absent, and answering by position is what the
/// compile-time pin and this refusal both exist to prevent.
#[test]
fn a_contested_architecture_without_a_template_refuses() {
    let outcome = family::select(&FamilyName("llama".into()), None);
    assert!(
        matches!(outcome, Err(family::FamilyRefusal::TemplateAbsent(_))),
        "expected a refusal naming the absent template, got {outcome:?}"
    );
}

/// **An architecture the template detector cannot read stays admissible.**
///
/// Gemma4's template returns an error rather than a rendering, measured against
/// the artifact this workshop holds. It resolves because its architecture is
/// carried by one entry and nothing is rendered to reach it, which is the
/// clause of Spec section 5 that keeps an unconditional render out.
#[test]
fn a_family_the_detector_refuses_still_resolves() {
    let Some(path) = held("gemma4-31b-it-Q8_0.gguf") else {
        return;
    };
    let header = header_of(&path);
    assert_eq!(header.family, FamilyName("gemma4".into()));
    family::select(&header.family, header.chat_template.as_deref())
        .expect("an uncontested architecture resolves without rendering");
}

/// **An architecture no entry declares refuses naming it**, which the two-field
/// key leaves unchanged.
#[test]
fn an_uncarried_architecture_still_refuses_by_name() {
    let absent = FamilyName("not-a-family".into());
    assert!(matches!(
        family::select(&absent, None),
        Err(family::FamilyRefusal::UnknownFamily(name)) if name == absent
    ));
}
