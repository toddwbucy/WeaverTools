//! conforms: analysis-summary-reports-the-run-and-its-conditions
//! The explicitly named operator deposit, per analysis Spec section 5.
//! Only observed device and build identity cross; no sibling-path search.

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Deposit {
    pub device_model: Option<String>,
    pub commit: Option<String>,
    pub toolchain: Option<String>,
    pub driver: Option<String>,
    pub engine_libraries: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeIdentity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<Box<RawValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolchain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_libraries: Option<BTreeMap<String, String>>,
}

impl Deposit {
    pub fn read(path: &str) -> Result<Self, String> {
        let bytes =
            std::fs::read(path).map_err(|e| format!("deposit {path} does not open: {e}"))?;
        serde_json::from_slice(&bytes).map_err(|e| format!("deposit {path} is unreadable: {e}"))
    }

    pub fn code_identity(&self, stack: Option<Box<RawValue>>) -> Option<CodeIdentity> {
        if stack.is_none()
            && self.commit.is_none()
            && self.toolchain.is_none()
            && self.driver.is_none()
            && self.engine_libraries.is_none()
        {
            return None;
        }
        Some(CodeIdentity {
            stack,
            commit: self.commit.clone(),
            toolchain: self.toolchain.clone(),
            driver: self.driver.clone(),
            engine_libraries: self.engine_libraries.clone(),
        })
    }
}
