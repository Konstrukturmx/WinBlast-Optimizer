//! WinBlast0 v1.6.0 — Preset Sharing, Canonical Binary Codec, Isolated Dry-Run
//! Simulation, and Differential Review Model.
//!
//! Provides:
//! - Canonical `WBP1` binary package format with strict bounds checking,
//!   SHA-256 (FIPS 180-4) payload integrity, and RFC 8032 Ed25519 signature verification.
//! - Isolated `SandboxEnv` dry-run evaluation with hardware/OS guards,
//!   dependency/order/cycle/conflict resolution, parameter validation, and
//!   unrecoverable reboot penalty detection.
//! - Lossless, reversible `DifferentialReview` across `Registry`, `Services`,
//!   and `Power` subsystems.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

use crate::backup::state_ledger::StateLedger;
use crate::core::backend::{SandboxEnv, SystemBackend};
use crate::core::risk::{TweakAction, TweakDefinition};
use crate::executor::engine::apply_tweak_with;

/// Canonical binary package header magic (`WBP1`).
pub const PACKAGE_MAGIC: [u8; 4] = *b"WBP1";

/// Current canonical binary format revision.
pub const FORMAT_REVISION_V1: u32 = 1;

/// Domain separation prefix committed into Ed25519 signatures to prevent
/// cross-protocol and cross-version signature replay.
pub const SIGNATURE_DOMAIN_SEPARATOR: &[u8] = b"WinBlast0-WBP1-SIG\x00";

/// Hard upper bounds enforcing OOM and malformed-input protection during decode.
pub const MAX_PACKAGE_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_TITLE_LEN: usize = 128;
pub const MAX_AUTHOR_LEN: usize = 128;
pub const MAX_DESCRIPTION_LEN: usize = 4096;
pub const MAX_TWEAK_COUNT: usize = 1024;
pub const MAX_TWEAK_ID_LEN: usize = 128;
pub const MAX_OVERRIDE_COUNT: usize = 2048;
pub const MAX_PARAM_KEY_LEN: usize = 128;
pub const MAX_PARAM_STRING_LEN: usize = 1024;
pub const MAX_PARAM_BYTES_LEN: usize = 4096;

const HEADER_LEN: usize = 12; // 4 (magic) + 4 (revision) + 4 (payload_len)
const DIGEST_LEN: usize = 32;
const SIGNATURE_LEN: usize = 64;
const MIN_PACKAGE_LEN: usize = HEADER_LEN + DIGEST_LEN + 1;

// ============================================================================
// Errors
// ============================================================================

/// Exhaustive error taxonomy for encoding, decoding, verification, and dry-run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresetError {
    PackageTooLarge { actual: usize, max: usize },
    UnexpectedEof,
    TrailingBytes { extra: usize },
    InvalidMagic { found: [u8; 4] },
    UnsupportedRevision { found: u32, expected: u32 },
    PayloadLengthMismatch { declared: usize, consumed: usize },
    InvalidUtf8,
    EmptyField { field: &'static str },
    FieldTooLong { field: &'static str, actual: usize, max: usize },
    InvalidIdentifier { field: &'static str, value: String },
    InvalidControlChar { field: &'static str },
    CollectionTooLarge { field: &'static str, actual: usize, max: usize },
    InvalidEnumTag { field: &'static str, tag: u8 },
    InvalidGuardRange { min_os_build: u32, max_os_build: u32 },
    DuplicateTweakId { tweak_id: String },
    NonCanonicalOverrideOrder,
    OrphanedOverride { tweak_id: String, param_name: String },
    DigestMismatch,
    MissingRequiredSignature,
    InvalidPublicKey,
    InvalidSignatureEncoding,
    SignatureVerificationFailed,
    OsBuildIncompatible { current: u32, min: Option<u32>, max: Option<u32> },
    GpuVendorIncompatible { required: GpuVendor, detected: String },
    UnknownTweakId { tweak_id: String },
    DependencyUnsatisfied { tweak_id: String, missing_dependency: String },
    ConflictingTweaks { tweak_a: String, tweak_b: String },
    ExecutionFailed { tweak_id: String, reason: String },
}

impl fmt::Display for PresetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PackageTooLarge { actual, max } => {
                write!(f, "package size {actual} exceeds maximum {max} bytes")
            }
            Self::UnexpectedEof => write!(f, "unexpected end of binary package"),
            Self::TrailingBytes { extra } => {
                write!(f, "unexpected {extra} trailing byte(s) after package trailer")
            }
            Self::InvalidMagic { found } => {
                write!(f, "invalid package magic: expected WBP1, found {found:?}")
            }
            Self::UnsupportedRevision { found, expected } => {
                write!(f, "unsupported format revision {found} (expected {expected})")
            }
            Self::PayloadLengthMismatch { declared, consumed } => {
                write!(f, "payload length mismatch: header declared {declared} bytes, decoder consumed {consumed}")
            }
            Self::InvalidUtf8 => write!(f, "invalid UTF-8 string in binary payload"),
            Self::EmptyField { field } => write!(f, "required field '{field}' cannot be empty"),
            Self::FieldTooLong { field, actual, max } => {
                write!(f, "field '{field}' length {actual} exceeds maximum {max}")
            }
            Self::InvalidIdentifier { field, value } => {
                write!(f, "field '{field}' contains invalid identifier '{value}'")
            }
            Self::InvalidControlChar { field } => {
                write!(f, "field '{field}' contains disallowed control characters")
            }
            Self::CollectionTooLarge { field, actual, max } => {
                write!(f, "collection '{field}' count {actual} exceeds maximum {max}")
            }
            Self::InvalidEnumTag { field, tag } => {
                write!(f, "invalid discriminant {tag} for '{field}'")
            }
            Self::InvalidGuardRange { min_os_build, max_os_build } => {
                write!(f, "invalid OS build guard range: min ({min_os_build}) > max ({max_os_build})")
            }
            Self::DuplicateTweakId { tweak_id } => {
                write!(f, "duplicate tweak ID '{tweak_id}' in ordered sequence")
            }
            Self::NonCanonicalOverrideOrder => {
                write!(f, "parameter overrides are not in strictly ascending canonical key order")
            }
            Self::OrphanedOverride { tweak_id, param_name } => {
                write!(f, "override '{tweak_id}.{param_name}' references a tweak ID not present in preset")
            }
            Self::DigestMismatch => write!(f, "SHA-256 payload digest verification failed"),
            Self::MissingRequiredSignature => {
                write!(f, "signature policy requires an Ed25519 signature, but package is unsigned")
            }
            Self::InvalidPublicKey => write!(f, "invalid or malformed Ed25519 public key"),
            Self::InvalidSignatureEncoding => {
                write!(f, "non-canonical or malformed Ed25519 signature encoding")
            }
            Self::SignatureVerificationFailed => {
                write!(f, "Ed25519 signature verification failed")
            }
            Self::OsBuildIncompatible { current, min, max } => {
                write!(f, "OS build {current} does not satisfy preset requirement ({min:?}..={max:?})")
            }
            Self::GpuVendorIncompatible { required, detected } => {
                write!(f, "GPU vendor {detected} does not match required {required:?}")
            }
            Self::UnknownTweakId { tweak_id } => {
                write!(f, "tweak ID '{tweak_id}' does not exist in the loaded rules catalog")
            }
            Self::DependencyUnsatisfied { tweak_id, missing_dependency } => {
                write!(f, "tweak '{tweak_id}' requires prerequisite '{missing_dependency}' which is not scheduled earlier in preset")
            }
            Self::ConflictingTweaks { tweak_a, tweak_b } => {
                write!(f, "preset contains mutually exclusive conflicting tweaks: '{tweak_a}' and '{tweak_b}'")
            }
            Self::ExecutionFailed { tweak_id, reason } => {
                write!(f, "dry-run simulation failed on tweak '{tweak_id}': {reason}")
            }
        }
    }
}

impl std::error::Error for PresetError {}

// ============================================================================
// Core Package Data Model
// ============================================================================

/// High-level functional classification of a WinBlast0 preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PresetCategory {
    Gaming = 1,
    Latency = 2,
    Privacy = 3,
    PowerEfficiency = 4,
    Workstation = 5,
    Custom = 6,
}

impl PresetCategory {
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn from_u8(tag: u8) -> Result<Self, PresetError> {
        match tag {
            1 => Ok(Self::Gaming),
            2 => Ok(Self::Latency),
            3 => Ok(Self::Privacy),
            4 => Ok(Self::PowerEfficiency),
            5 => Ok(Self::Workstation),
            6 => Ok(Self::Custom),
            _ => Err(PresetError::InvalidEnumTag {
                field: "metadata.category",
                tag,
            }),
        }
    }
}

/// Supported GPU hardware vendors for preset compatibility guards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum GpuVendor {
    Nvidia = 1,
    Amd = 2,
    Intel = 3,
}

impl GpuVendor {
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn from_u8(tag: u8) -> Result<Self, PresetError> {
        match tag {
            1 => Ok(Self::Nvidia),
            2 => Ok(Self::Amd),
            3 => Ok(Self::Intel),
            _ => Err(PresetError::InvalidEnumTag {
                field: "guards.required_gpu_vendor",
                tag,
            }),
        }
    }
}

/// Descriptive metadata embedded in a canonical `WBP1` package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresetMetadata {
    pub title: String,
    pub author: String,
    pub description: String,
    /// UTC Unix timestamp in seconds.
    pub created_at: u64,
    pub category: PresetCategory,
}

/// Hardware and OS compatibility preconditions checked before application.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PresetGuards {
    pub min_os_build: Option<u32>,
    pub max_os_build: Option<u32>,
    pub required_gpu_vendor: Option<GpuVendor>,
}

/// Canonical composite key `(tweak_id, param_name)` for parameter overrides.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OverrideKey {
    pub tweak_id: String,
    pub param_name: String,
}

impl OverrideKey {
    pub fn new(tweak_id: impl Into<String>, param_name: impl Into<String>) -> Self {
        Self {
            tweak_id: tweak_id.into(),
            param_name: param_name.into(),
        }
    }
}

/// Strongly typed custom parameter override values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamValue {
    Bool(bool),
    I64(i64),
    U64(u64),
    Str(String),
    Bytes(Vec<u8>),
}

/// Signature enforcement policy applied during package decoding.
#[derive(Debug, Clone, Copy)]
pub enum SignaturePolicy<'a> {
    /// Reject unsigned packages and require a valid Ed25519 signature by `key`.
    Required(&'a VerifyingKey),
    /// Allow unsigned packages; if a signature is present, require validity under `key`.
    VerifyIfPresent(&'a VerifyingKey),
    /// Skip cryptographic signature verification (structural & SHA-256 checks still enforced).
    AllowUnsigned,
}

/// Canonical WinBlast0 preset package (`WBP1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresetPackage {
    pub format_revision: u32,
    pub metadata: PresetMetadata,
    pub guards: PresetGuards,
    /// Ordered, deduplicated sequence of tweak IDs to evaluate and apply.
    pub tweak_ids: Vec<String>,
    /// Canonical sorted map of `(tweak_id, param_name) -> value`.
    pub parameter_overrides: BTreeMap<OverrideKey, ParamValue>,
    /// SHA-256 digest of the canonical binary payload section.
    pub payload_digest: [u8; 32],
    /// Optional 64-byte RFC 8032 Ed25519 signature over the domain-separated transcript.
    pub signature: Option<[u8; 64]>,
}

impl PresetPackage {
    /// Constructs a validated unsigned `WBP1` package and computes its canonical SHA-256 digest.
    pub fn new(
        metadata: PresetMetadata,
        guards: PresetGuards,
        tweak_ids: Vec<String>,
        parameter_overrides: BTreeMap<OverrideKey, ParamValue>,
    ) -> Result<Self, PresetError> {
        let mut pkg = Self {
            format_revision: FORMAT_REVISION_V1,
            metadata,
            guards,
            tweak_ids,
            parameter_overrides,
            payload_digest: [0u8; 32],
            signature: None,
        };
        pkg.refresh_digest()?;
        Ok(pkg)
    }

    /// Validates all structural, charset, ordering, and relational invariants.
    pub fn validate(&self) -> Result<(), PresetError> {
        if self.format_revision != FORMAT_REVISION_V1 {
            return Err(PresetError::UnsupportedRevision {
                found: self.format_revision,
                expected: FORMAT_REVISION_V1,
            });
        }

        validate_text_field("metadata.title", &self.metadata.title, MAX_TITLE_LEN, false, false)?;
        validate_text_field("metadata.author", &self.metadata.author, MAX_AUTHOR_LEN, false, false)?;
        validate_text_field(
            "metadata.description",
            &self.metadata.description,
            MAX_DESCRIPTION_LEN,
            true,
            true,
        )?;

        if let (Some(min), Some(max)) = (self.guards.min_os_build, self.guards.max_os_build) {
            if min > max {
                return Err(PresetError::InvalidGuardRange {
                    min_os_build: min,
                    max_os_build: max,
                });
            }
        }

        if self.tweak_ids.len() > MAX_TWEAK_COUNT {
            return Err(PresetError::CollectionTooLarge {
                field: "tweak_ids",
                actual: self.tweak_ids.len(),
                max: MAX_TWEAK_COUNT,
            });
        }

        let mut seen_tweaks = BTreeSet::new();
        for tweak_id in &self.tweak_ids {
            validate_identifier("tweak_id", tweak_id, MAX_TWEAK_ID_LEN)?;
            if !seen_tweaks.insert(tweak_id.as_str()) {
                return Err(PresetError::DuplicateTweakId {
                    tweak_id: tweak_id.clone(),
                });
            }
        }

        if self.parameter_overrides.len() > MAX_OVERRIDE_COUNT {
            return Err(PresetError::CollectionTooLarge {
                field: "parameter_overrides",
                actual: self.parameter_overrides.len(),
                max: MAX_OVERRIDE_COUNT,
            });
        }

        for (key, value) in &self.parameter_overrides {
            validate_identifier("override.tweak_id", &key.tweak_id, MAX_TWEAK_ID_LEN)?;
            validate_identifier("override.param_name", &key.param_name, MAX_PARAM_KEY_LEN)?;

            if !seen_tweaks.contains(key.tweak_id.as_str()) {
                return Err(PresetError::OrphanedOverride {
                    tweak_id: key.tweak_id.clone(),
                    param_name: key.param_name.clone(),
                });
            }

            match value {
                ParamValue::Bool(_) | ParamValue::I64(_) | ParamValue::U64(_) => {}
                ParamValue::Str(s) => {
                    validate_text_field("override.value.str", s, MAX_PARAM_STRING_LEN, true, true)?;
                }
                ParamValue::Bytes(b) => {
                    if b.len() > MAX_PARAM_BYTES_LEN {
                        return Err(PresetError::FieldTooLong {
                            field: "override.value.bytes",
                            actual: b.len(),
                            max: MAX_PARAM_BYTES_LEN,
                        });
                    }
                }
            }
        }

        Ok(())
    }

    /// Recomputes and stores the canonical SHA-256 payload digest, clearing any stale signature.
    pub fn refresh_digest(&mut self) -> Result<[u8; 32], PresetError> {
        let payload = self.encode_canonical_payload()?;
        let digest = compute_sha256(&payload);
        self.payload_digest = digest;
        self.signature = None;
        Ok(digest)
    }

    /// Computes the canonical SHA-256 payload digest without mutating `self`.
    pub fn compute_payload_digest(&self) -> Result<[u8; 32], PresetError> {
        let payload = self.encode_canonical_payload()?;
        Ok(compute_sha256(&payload))
    }

    /// Recomputes the canonical payload digest and attaches an RFC 8032 Ed25519 signature.
    pub fn sign(&mut self, signing_key: &SigningKey) -> Result<[u8; 64], PresetError> {
        let digest = self.refresh_digest()?;
        let transcript = build_signing_transcript(self.format_revision, &digest);
        let sig = signing_key.sign(&transcript);
        let sig_bytes = sig.to_bytes();
        self.signature = Some(sig_bytes);
        Ok(sig_bytes)
    }

    /// Verifies the attached Ed25519 signature and payload digest against `public_key`.
    pub fn verify_signature(&self, public_key: &VerifyingKey) -> Result<(), PresetError> {
        let computed = self.compute_payload_digest()?;
        if computed != self.payload_digest {
            return Err(PresetError::DigestMismatch);
        }

        let sig_bytes = self.signature.as_ref().ok_or(PresetError::MissingRequiredSignature)?;
        let sig = Signature::from_bytes(sig_bytes);
        let transcript = build_signing_transcript(self.format_revision, &computed);
        public_key
            .verify(&transcript, &sig)
            .map_err(|_| PresetError::SignatureVerificationFailed)
    }

    /// Encodes the package into the canonical `WBP1` binary format.
    pub fn encode(&self) -> Result<Vec<u8>, PresetError> {
        let payload = self.encode_canonical_payload()?;
        let computed_digest = compute_sha256(&payload);
        if computed_digest != self.payload_digest {
            return Err(PresetError::DigestMismatch);
        }

        let payload_len_u32 = u32::try_from(payload.len()).map_err(|_| {
            PresetError::PackageTooLarge {
                actual: payload.len(),
                max: MAX_PACKAGE_BYTES,
            }
        })?;

        let sig_bytes = if self.signature.is_some() { SIGNATURE_LEN } else { 0 };
        let total_len = HEADER_LEN
            .checked_add(payload.len())
            .and_then(|v| v.checked_add(DIGEST_LEN))
            .and_then(|v| v.checked_add(1))
            .and_then(|v| v.checked_add(sig_bytes))
            .ok_or(PresetError::PackageTooLarge {
                actual: usize::MAX,
                max: MAX_PACKAGE_BYTES,
            })?;

        if total_len > MAX_PACKAGE_BYTES {
            return Err(PresetError::PackageTooLarge {
                actual: total_len,
                max: MAX_PACKAGE_BYTES,
            });
        }

        let mut out = Vec::with_capacity(total_len);
        out.extend_from_slice(&PACKAGE_MAGIC);
        out.extend_from_slice(&self.format_revision.to_le_bytes());
        out.extend_from_slice(&payload_len_u32.to_le_bytes());
        out.extend_from_slice(&payload);
        out.extend_from_slice(&computed_digest);

        match &self.signature {
            None => out.push(0),
            Some(sig) => {
                out.push(1);
                out.extend_from_slice(sig);
            }
        }

        Ok(out)
    }

    /// Decodes and validates a `WBP1` binary package with bounds checking,
    /// SHA-256 digest verification, and enforcement of `policy`.
    pub fn decode(bytes: &[u8], policy: SignaturePolicy<'_>) -> Result<Self, PresetError> {
        if bytes.len() > MAX_PACKAGE_BYTES {
            return Err(PresetError::PackageTooLarge {
                actual: bytes.len(),
                max: MAX_PACKAGE_BYTES,
            });
        }

        if bytes.len() < MIN_PACKAGE_LEN {
            return Err(PresetError::UnexpectedEof);
        }

        let mut hdr = BinaryReader::new(&bytes[..HEADER_LEN]);
        let magic = hdr.read_fixed::<4>()?;
        if magic != PACKAGE_MAGIC {
            return Err(PresetError::InvalidMagic { found: magic });
        }

        let format_revision = hdr.read_u32()?;
        if format_revision != FORMAT_REVISION_V1 {
            return Err(PresetError::UnsupportedRevision {
                found: format_revision,
                expected: FORMAT_REVISION_V1,
            });
        }

        let declared_payload_len = hdr.read_u32()? as usize;
        let payload_end = HEADER_LEN
            .checked_add(declared_payload_len)
            .ok_or(PresetError::UnexpectedEof)?;
        let digest_end = payload_end
            .checked_add(DIGEST_LEN)
            .ok_or(PresetError::UnexpectedEof)?;
        let flag_end = digest_end
            .checked_add(1)
            .ok_or(PresetError::UnexpectedEof)?;

        if flag_end > bytes.len() {
            return Err(PresetError::UnexpectedEof);
        }

        let payload_slice = &bytes[HEADER_LEN..payload_end];
        let mut expected_digest = [0u8; DIGEST_LEN];
        expected_digest.copy_from_slice(&bytes[payload_end..digest_end]);

        let actual_digest = compute_sha256(payload_slice);
        if actual_digest != expected_digest {
            return Err(PresetError::DigestMismatch);
        }

        let sig_flag = bytes[digest_end];
        let signature = match sig_flag {
            0 => {
                if bytes.len() > flag_end {
                    return Err(PresetError::TrailingBytes {
                        extra: bytes.len() - flag_end,
                    });
                }
                None
            }
            1 => {
                let sig_end = flag_end
                    .checked_add(SIGNATURE_LEN)
                    .ok_or(PresetError::UnexpectedEof)?;
                if bytes.len() < sig_end {
                    return Err(PresetError::UnexpectedEof);
                }
                if bytes.len() > sig_end {
                    return Err(PresetError::TrailingBytes {
                        extra: bytes.len() - sig_end,
                    });
                }
                let mut sig = [0u8; SIGNATURE_LEN];
                sig.copy_from_slice(&bytes[flag_end..sig_end]);
                Some(sig)
            }
            tag => {
                return Err(PresetError::InvalidEnumTag {
                    field: "signature_flag",
                    tag,
                });
            }
        };

        // Enforce signature policy before parsing untrusted payload contents.
        match (policy, &signature) {
            (SignaturePolicy::Required(pk), Some(sig_bytes))
            | (SignaturePolicy::VerifyIfPresent(pk), Some(sig_bytes)) => {
                let sig = Signature::from_bytes(sig_bytes);
                let transcript = build_signing_transcript(format_revision, &actual_digest);
                pk.verify(&transcript, &sig)
                    .map_err(|_| PresetError::SignatureVerificationFailed)?;
            }
            (SignaturePolicy::Required(_), None) => {
                return Err(PresetError::MissingRequiredSignature);
            }
            (SignaturePolicy::VerifyIfPresent(_), None) | (SignaturePolicy::AllowUnsigned, _) => {}
        }

        let mut reader = BinaryReader::new(payload_slice);

        // Metadata
        let title = reader.read_string("metadata.title", MAX_TITLE_LEN)?;
        let author = reader.read_string("metadata.author", MAX_AUTHOR_LEN)?;
        let description = reader.read_string("metadata.description", MAX_DESCRIPTION_LEN)?;
        let created_at = reader.read_u64()?;
        let category = PresetCategory::from_u8(reader.read_u8()?)?;
        let metadata = PresetMetadata {
            title,
            author,
            description,
            created_at,
            category,
        };

        // Guards
        let min_os_build = reader.read_opt_u32("guards.min_os_build")?;
        let max_os_build = reader.read_opt_u32("guards.max_os_build")?;
        let required_gpu_vendor = match reader.read_u8()? {
            0 => None,
            1 => Some(GpuVendor::from_u8(reader.read_u8()?)?),
            tag => {
                return Err(PresetError::InvalidEnumTag {
                    field: "guards.required_gpu_vendor.option",
                    tag,
                });
            }
        };
        let guards = PresetGuards {
            min_os_build,
            max_os_build,
            required_gpu_vendor,
        };

        // Ordered tweak IDs
        let tweak_count = reader.read_u32()? as usize;
        if tweak_count > MAX_TWEAK_COUNT {
            return Err(PresetError::CollectionTooLarge {
                field: "tweak_ids",
                actual: tweak_count,
                max: MAX_TWEAK_COUNT,
            });
        }

        let mut tweak_ids = Vec::with_capacity(tweak_count);
        for _ in 0..tweak_count {
            tweak_ids.push(reader.read_string("tweak_id", MAX_TWEAK_ID_LEN)?);
        }

        // Canonical sorted parameter overrides
        let override_count = reader.read_u32()? as usize;
        if override_count > MAX_OVERRIDE_COUNT {
            return Err(PresetError::CollectionTooLarge {
                field: "parameter_overrides",
                actual: override_count,
                max: MAX_OVERRIDE_COUNT,
            });
        }

        let mut parameter_overrides = BTreeMap::new();
        let mut prev_key: Option<OverrideKey> = None;

        for _ in 0..override_count {
            let tweak_id = reader.read_string("override.tweak_id", MAX_TWEAK_ID_LEN)?;
            let param_name = reader.read_string("override.param_name", MAX_PARAM_KEY_LEN)?;
            let key = OverrideKey { tweak_id, param_name };

            if let Some(ref prev) = prev_key {
                if key <= *prev {
                    return Err(PresetError::NonCanonicalOverrideOrder);
                }
            }

            let value = match reader.read_u8()? {
                1 => match reader.read_u8()? {
                    0 => ParamValue::Bool(false),
                    1 => ParamValue::Bool(true),
                    tag => {
                        return Err(PresetError::InvalidEnumTag {
                            field: "override.value.bool",
                            tag,
                        });
                    }
                },
                2 => ParamValue::I64(reader.read_i64()?),
                3 => ParamValue::U64(reader.read_u64()?),
                4 => ParamValue::Str(reader.read_string("override.value.str", MAX_PARAM_STRING_LEN)?),
                5 => ParamValue::Bytes(reader.read_bytes("override.value.bytes", MAX_PARAM_BYTES_LEN)?),
                tag => {
                    return Err(PresetError::InvalidEnumTag {
                        field: "override.value.tag",
                        tag,
                    });
                }
            };

            prev_key = Some(key.clone());
            parameter_overrides.insert(key, value);
        }

        if reader.remaining() != 0 {
            return Err(PresetError::PayloadLengthMismatch {
                declared: declared_payload_len,
                consumed: reader.pos,
            });
        }

        let pkg = Self {
            format_revision,
            metadata,
            guards,
            tweak_ids,
            parameter_overrides,
            payload_digest: actual_digest,
            signature,
        };

        pkg.validate()?;
        Ok(pkg)
    }

    fn encode_canonical_payload(&self) -> Result<Vec<u8>, PresetError> {
        self.validate()?;
        let mut w = BinaryWriter::new();

        // Metadata
        w.write_str(&self.metadata.title)?;
        w.write_str(&self.metadata.author)?;
        w.write_str(&self.metadata.description)?;
        w.write_u64(self.metadata.created_at);
        w.write_u8(self.metadata.category.to_u8());

        // Guards
        w.write_opt_u32(self.guards.min_os_build);
        w.write_opt_u32(self.guards.max_os_build);
        match self.guards.required_gpu_vendor {
            None => w.write_u8(0),
            Some(vendor) => {
                w.write_u8(1);
                w.write_u8(vendor.to_u8());
            }
        }

        // Ordered tweak IDs
        w.write_u32(self.tweak_ids.len() as u32);
        for tweak_id in &self.tweak_ids {
            w.write_str(tweak_id)?;
        }

        // Parameter overrides (iterating BTreeMap guarantees strictly ascending order)
        w.write_u32(self.parameter_overrides.len() as u32);
        for (key, value) in &self.parameter_overrides {
            w.write_str(&key.tweak_id)?;
            w.write_str(&key.param_name)?;
            match value {
                ParamValue::Bool(b) => {
                    w.write_u8(1);
                    w.write_u8(if *b { 1 } else { 0 });
                }
                ParamValue::I64(v) => {
                    w.write_u8(2);
                    w.write_i64(*v);
                }
                ParamValue::U64(v) => {
                    w.write_u8(3);
                    w.write_u64(*v);
                }
                ParamValue::Str(s) => {
                    w.write_u8(4);
                    w.write_str(s)?;
                }
                ParamValue::Bytes(b) => {
                    w.write_u8(5);
                    w.write_bytes(b)?;
                }
            }
        }

        Ok(w.into_bytes())
    }
}

// ============================================================================
// Dry-Run Simulation & Differential Review Model
// ============================================================================

/// Exact diff of a single registry DWORD/String mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryDiff {
    pub hive: String,
    pub path: String,
    pub name: String,
    pub original_dword: Option<u32>,
    pub proposed_dword: Option<u32>,
    pub original_string: Option<String>,
    pub proposed_string: Option<String>,
}

/// Exact diff of a service state mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceDiff {
    pub service_name: String,
    pub original_start_type: u32,
    pub proposed_start_type: u32,
}

/// Exact diff of a power scheme setting mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerDiff {
    pub subgroup_guid: String,
    pub setting_guid: String,
    pub original_ac: u32,
    pub proposed_ac: u32,
    pub original_dc: u32,
    pub proposed_dc: u32,
}

/// Differential review report computed by dry-running the preset in `SandboxEnv`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresetDifferentialReport {
    pub total_tweaks: usize,
    pub requires_reboot_count: usize,
    pub unresolvable_conflicts: Vec<String>,
    pub registry_diffs: Vec<RegistryDiff>,
    pub service_diffs: Vec<ServiceDiff>,
    pub power_diffs: Vec<PowerDiff>,
}

/// Evaluates `preset` against `catalog` in an isolated `SandboxEnv` without modifying system state.
pub fn simulate_preset_dry_run(
    preset: &PresetPackage,
    catalog: &[TweakDefinition],
    sandbox: &SandboxEnv,
) -> Result<PresetDifferentialReport, PresetError> {
    preset.validate()?;

    // 1. Check OS Build guards if detected
    let os_info = sandbox.detect_os().map_err(|e| PresetError::ExecutionFailed {
        tweak_id: "guards".to_string(),
        reason: format!("failed to detect OS: {e}"),
    })?;

    if let Some(min) = preset.guards.min_os_build {
        if os_info.build_number < min {
            return Err(PresetError::OsBuildIncompatible {
                current: os_info.build_number,
                min: Some(min),
                max: preset.guards.max_os_build,
            });
        }
    }
    if let Some(max) = preset.guards.max_os_build {
        if os_info.build_number > max {
            return Err(PresetError::OsBuildIncompatible {
                current: os_info.build_number,
                min: preset.guards.min_os_build,
                max: Some(max),
            });
        }
    }

    // Index catalog tweaks by ID for fast lookup
    let tweak_map: HashMap<&str, &TweakDefinition> =
        catalog.iter().map(|t| (t.id.as_str(), t)).collect();

    // 2. Verify all tweak IDs exist and evaluate dependencies & conflicts
    let mut applied_ids = BTreeSet::new();
    let mut requires_reboot_count = 0;
    let mut unresolvable_conflicts = Vec::new();

    for tweak_id in &preset.tweak_ids {
        let tweak = tweak_map.get(tweak_id.as_str()).ok_or_else(|| {
            PresetError::UnknownTweakId {
                tweak_id: tweak_id.clone(),
            }
        })?;

        // Check prerequisites
        for prereq in &tweak.prerequisites {
            if !applied_ids.contains(prereq.as_str()) {
                return Err(PresetError::DependencyUnsatisfied {
                    tweak_id: tweak_id.clone(),
                    missing_dependency: prereq.clone(),
                });
            }
        }

        // Check conflicts
        for conflict in &tweak.conflicts_with {
            if applied_ids.contains(conflict.as_str()) {
                let msg = format!("'{}' conflicts with '{}'", tweak_id, conflict);
                unresolvable_conflicts.push(msg);
            }
        }

        if tweak.valuation.reversibility == crate::core::risk::Reversibility::RequiresReboot {
            requires_reboot_count += 1;
        }

        applied_ids.insert(tweak_id.as_str());
    }

    if !unresolvable_conflicts.is_empty() {
        return Ok(PresetDifferentialReport {
            total_tweaks: preset.tweak_ids.len(),
            requires_reboot_count,
            unresolvable_conflicts,
            registry_diffs: Vec::new(),
            service_diffs: Vec::new(),
            power_diffs: Vec::new(),
        });
    }

    // 3. Capture baseline state and simulate execution in SandboxEnv
    let in_memory_ledger = StateLedger::in_memory().map_err(|e| PresetError::ExecutionFailed {
        tweak_id: "ledger".to_string(),
        reason: format!("failed to initialize memory ledger: {e}"),
    })?;

    let mut registry_diffs = Vec::new();
    let mut service_diffs = Vec::new();
    let mut power_diffs = Vec::new();

    for tweak_id in &preset.tweak_ids {
        let tweak = tweak_map[tweak_id.as_str()];

        // Record pre-mutation diffs
        for action in &tweak.actions {
            match action {
                TweakAction::RegistryDword { hive, path, name, optimized_value, .. } => {
                    let orig = sandbox.reg_get_dword(hive, path, name);
                    registry_diffs.push(RegistryDiff {
                        hive: hive.clone(),
                        path: path.clone(),
                        name: name.clone(),
                        original_dword: orig,
                        proposed_dword: Some(*optimized_value),
                        original_string: None,
                        proposed_string: None,
                    });
                }
                TweakAction::RegistryString { hive, path, name, optimized_value, .. } => {
                    let orig = sandbox.reg_get_string(hive, path, name, None).map(|s| s.value);
                    registry_diffs.push(RegistryDiff {
                        hive: hive.clone(),
                        path: path.clone(),
                        name: name.clone(),
                        original_dword: None,
                        proposed_dword: None,
                        original_string: orig,
                        proposed_string: Some(optimized_value.clone()),
                    });
                }
                TweakAction::ServiceState { service_name, target_start_type, default_start_type } => {
                    service_diffs.push(ServiceDiff {
                        service_name: service_name.clone(),
                        original_start_type: *default_start_type,
                        proposed_start_type: *target_start_type,
                    });
                }
                TweakAction::PowerCfgSetting { subgroup_guid, setting_guid, optimized_ac, optimized_dc, default_ac, default_dc, .. } => {
                    power_diffs.push(PowerDiff {
                        subgroup_guid: subgroup_guid.clone(),
                        setting_guid: setting_guid.clone(),
                        original_ac: *default_ac,
                        proposed_ac: *optimized_ac,
                        original_dc: *default_dc,
                        proposed_dc: *optimized_dc,
                    });
                }
                _ => {}
            }
        }

        // Apply tweak into sandbox environment
        apply_tweak_with(tweak, &in_memory_ledger, sandbox).map_err(|e| {
            PresetError::ExecutionFailed {
                tweak_id: tweak_id.clone(),
                reason: format!("simulation error: {e}"),
            }
        })?;
    }

    Ok(PresetDifferentialReport {
        total_tweaks: preset.tweak_ids.len(),
        requires_reboot_count,
        unresolvable_conflicts,
        registry_diffs,
        service_diffs,
        power_diffs,
    })
}

// ============================================================================
// Internal Helpers & Codec Primitives
// ============================================================================

pub fn compute_sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn build_signing_transcript(revision: u32, digest: &[u8; 32]) -> Vec<u8> {
    let mut transcript = Vec::with_capacity(SIGNATURE_DOMAIN_SEPARATOR.len() + 4 + 32);
    transcript.extend_from_slice(SIGNATURE_DOMAIN_SEPARATOR);
    transcript.extend_from_slice(&revision.to_le_bytes());
    transcript.extend_from_slice(digest);
    transcript
}

fn validate_text_field(
    field: &'static str,
    val: &str,
    max_len: usize,
    allow_empty: bool,
    allow_multiline: bool,
) -> Result<(), PresetError> {
    if !allow_empty && val.trim().is_empty() {
        return Err(PresetError::EmptyField { field });
    }
    if val.len() > max_len {
        return Err(PresetError::FieldTooLong {
            field,
            actual: val.len(),
            max: max_len,
        });
    }
    for ch in val.chars() {
        if ch.is_control() {
            if allow_multiline && (ch == '\n' || ch == '\r' || ch == '\t') {
                continue;
            }
            return Err(PresetError::InvalidControlChar { field });
        }
    }
    Ok(())
}

fn validate_identifier(
    field: &'static str,
    val: &str,
    max_len: usize,
) -> Result<(), PresetError> {
    if val.is_empty() {
        return Err(PresetError::EmptyField { field });
    }
    if val.len() > max_len {
        return Err(PresetError::FieldTooLong {
            field,
            actual: val.len(),
            max: max_len,
        });
    }
    for ch in val.chars() {
        if !ch.is_ascii_alphanumeric() && ch != '_' && ch != '.' && ch != '-' {
            return Err(PresetError::InvalidIdentifier {
                field,
                value: val.to_string(),
            });
        }
    }
    Ok(())
}

struct BinaryWriter {
    buf: Vec<u8>,
}

impl BinaryWriter {
    fn new() -> Self {
        Self { buf: Vec::with_capacity(256) }
    }

    fn write_u8(&mut self, val: u8) {
        self.buf.push(val);
    }

    fn write_u32(&mut self, val: u32) {
        self.buf.extend_from_slice(&val.to_le_bytes());
    }

    fn write_u64(&mut self, val: u64) {
        self.buf.extend_from_slice(&val.to_le_bytes());
    }

    fn write_i64(&mut self, val: i64) {
        self.buf.extend_from_slice(&val.to_le_bytes());
    }

    fn write_opt_u32(&mut self, val: Option<u32>) {
        match val {
            None => self.write_u8(0),
            Some(v) => {
                self.write_u8(1);
                self.write_u32(v);
            }
        }
    }

    fn write_str(&mut self, s: &str) -> Result<(), PresetError> {
        let len = u32::try_from(s.len()).map_err(|_| PresetError::PackageTooLarge {
            actual: s.len(),
            max: MAX_PACKAGE_BYTES,
        })?;
        self.write_u32(len);
        self.buf.extend_from_slice(s.as_bytes());
        Ok(())
    }

    fn write_bytes(&mut self, b: &[u8]) -> Result<(), PresetError> {
        let len = u32::try_from(b.len()).map_err(|_| PresetError::PackageTooLarge {
            actual: b.len(),
            max: MAX_PACKAGE_BYTES,
        })?;
        self.write_u32(len);
        self.buf.extend_from_slice(b);
        Ok(())
    }

    fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}

struct BinaryReader<'a> {
    slice: &'a [u8],
    pos: usize,
}

impl<'a> BinaryReader<'a> {
    fn new(slice: &'a [u8]) -> Self {
        Self { slice, pos: 0 }
    }

    fn remaining(&self) -> usize {
        self.slice.len().saturating_sub(self.pos)
    }

    fn read_fixed<const N: usize>(&mut self) -> Result<[u8; N], PresetError> {
        if self.remaining() < N {
            return Err(PresetError::UnexpectedEof);
        }
        let mut arr = [0u8; N];
        arr.copy_from_slice(&self.slice[self.pos..self.pos + N]);
        self.pos += N;
        Ok(arr)
    }

    fn read_u8(&mut self) -> Result<u8, PresetError> {
        if self.remaining() < 1 {
            return Err(PresetError::UnexpectedEof);
        }
        let b = self.slice[self.pos];
        self.pos += 1;
        Ok(b)
    }

    fn read_u32(&mut self) -> Result<u32, PresetError> {
        let b = self.read_fixed::<4>()?;
        Ok(u32::from_le_bytes(b))
    }

    fn read_u64(&mut self) -> Result<u64, PresetError> {
        let b = self.read_fixed::<8>()?;
        Ok(u64::from_le_bytes(b))
    }

    fn read_i64(&mut self) -> Result<i64, PresetError> {
        let b = self.read_fixed::<8>()?;
        Ok(i64::from_le_bytes(b))
    }

    fn read_opt_u32(&mut self, field: &'static str) -> Result<Option<u32>, PresetError> {
        match self.read_u8()? {
            0 => Ok(None),
            1 => Ok(Some(self.read_u32()?)),
            tag => Err(PresetError::InvalidEnumTag { field, tag }),
        }
    }

    fn read_string(&mut self, field: &'static str, max_len: usize) -> Result<String, PresetError> {
        let len = self.read_u32()? as usize;
        if len > max_len {
            return Err(PresetError::FieldTooLong {
                field,
                actual: len,
                max: max_len,
            });
        }
        if self.remaining() < len {
            return Err(PresetError::UnexpectedEof);
        }
        let bytes = &self.slice[self.pos..self.pos + len];
        self.pos += len;
        std::str::from_utf8(bytes)
            .map(|s| s.to_string())
            .map_err(|_| PresetError::InvalidUtf8)
    }

    fn read_bytes(&mut self, field: &'static str, max_len: usize) -> Result<Vec<u8>, PresetError> {
        let len = self.read_u32()? as usize;
        if len > max_len {
            return Err(PresetError::FieldTooLong {
                field,
                actual: len,
                max: max_len,
            });
        }
        if self.remaining() < len {
            return Err(PresetError::UnexpectedEof);
        }
        let b = self.slice[self.pos..self.pos + len].to_vec();
        self.pos += len;
        Ok(b)
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_metadata() -> PresetMetadata {
        PresetMetadata {
            title: "Ultra Low Latency Gaming".to_string(),
            author: "WinBlast Engineering".to_string(),
            description: "Disables DPC jitter, disables background telemetry, optimizes MMCSS".to_string(),
            created_at: 1732120000,
            category: PresetCategory::Latency,
        }
    }

    #[test]
    fn test_canonical_encode_decode_roundtrip_unsigned() {
        let metadata = sample_metadata();
        let guards = PresetGuards {
            min_os_build: Some(19041),
            max_os_build: Some(26100),
            required_gpu_vendor: Some(GpuVendor::Nvidia),
        };
        let tweak_ids = vec![
            "disable_telemetry".to_string(),
            "win32_priority_separation".to_string(),
        ];
        let mut overrides = BTreeMap::new();
        overrides.insert(
            OverrideKey::new("win32_priority_separation", "priority_value"),
            ParamValue::U64(0x28),
        );

        let pkg = PresetPackage::new(metadata, guards, tweak_ids, overrides).unwrap();
        let encoded = pkg.encode().unwrap();

        let decoded = PresetPackage::decode(&encoded, SignaturePolicy::AllowUnsigned).unwrap();
        assert_eq!(pkg, decoded);
    }

    #[test]
    fn test_ed25519_sign_and_verify_policy() {
        let metadata = sample_metadata();
        let guards = PresetGuards::default();
        let tweak_ids = vec!["disable_copilot".to_string()];
        let overrides = BTreeMap::new();

        let mut pkg = PresetPackage::new(metadata, guards, tweak_ids, overrides).unwrap();

        // Generate test signing key
        let mut seed = [0x42u8; 32];
        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();

        pkg.sign(&signing_key).unwrap();
        assert!(pkg.signature.is_some());

        let encoded = pkg.encode().unwrap();

        // Decode with Required policy
        let decoded = PresetPackage::decode(&encoded, SignaturePolicy::Required(&verifying_key)).unwrap();
        assert_eq!(pkg, decoded);

        // Verify with wrong key fails
        seed[0] ^= 0xFF;
        let wrong_signing_key = SigningKey::from_bytes(&seed);
        let wrong_verifying_key = wrong_signing_key.verifying_key();

        let fail_res = PresetPackage::decode(&encoded, SignaturePolicy::Required(&wrong_verifying_key));
        assert_eq!(fail_res.unwrap_err(), PresetError::SignatureVerificationFailed);
    }

    #[test]
    fn test_corrupted_magic_rejection() {
        let mut pkg_bytes = b"NOPE\x01\x00\x00\x00\x00\x00\x00\x00".to_vec();
        pkg_bytes.extend_from_slice(&[0u8; 40]);
        let res = PresetPackage::decode(&pkg_bytes, SignaturePolicy::AllowUnsigned);
        assert!(matches!(res.unwrap_err(), PresetError::InvalidMagic { .. }));
    }

    #[test]
    fn test_digest_mismatch_tampering_detection() {
        let metadata = sample_metadata();
        let pkg = PresetPackage::new(metadata, PresetGuards::default(), vec!["tweak_a".to_string()], BTreeMap::new()).unwrap();
        let mut encoded = pkg.encode().unwrap();

        // Tamper with a payload byte
        encoded[14] ^= 0x01;

        let res = PresetPackage::decode(&encoded, SignaturePolicy::AllowUnsigned);
        assert_eq!(res.unwrap_err(), PresetError::DigestMismatch);
    }

    #[test]
    fn test_duplicate_tweak_id_rejection() {
        let metadata = sample_metadata();
        let tweak_ids = vec!["tweak_a".to_string(), "tweak_a".to_string()];
        let res = PresetPackage::new(metadata, PresetGuards::default(), tweak_ids, BTreeMap::new());
        assert!(matches!(res.unwrap_err(), PresetError::DuplicateTweakId { .. }));
    }

    #[test]
    fn test_orphaned_override_rejection() {
        let metadata = sample_metadata();
        let tweak_ids = vec!["tweak_a".to_string()];
        let mut overrides = BTreeMap::new();
        overrides.insert(OverrideKey::new("tweak_b", "param"), ParamValue::Bool(true));

        let res = PresetPackage::new(metadata, PresetGuards::default(), tweak_ids, overrides);
        assert!(matches!(res.unwrap_err(), PresetError::OrphanedOverride { .. }));
    }

    #[test]
    fn test_simulate_preset_dry_run_with_catalog() {
        use crate::core::risk::load_embedded_rules;

        let catalog = load_embedded_rules();
        let metadata = sample_metadata();
        let guards = PresetGuards {
            min_os_build: Some(19041),
            max_os_build: None,
            required_gpu_vendor: None,
        };

        // Pick a tweak with registry actions
        let reg_tweak = catalog
            .iter()
            .find(|t| {
                t.actions
                    .iter()
                    .any(|a| matches!(a, TweakAction::RegistryDword { .. }))
            })
            .expect("expected at least one registry tweak");

        let tweak_ids = vec![reg_tweak.id.clone()];

        let pkg = PresetPackage::new(metadata, guards, tweak_ids, BTreeMap::new()).unwrap();
        let sandbox = SandboxEnv::new();

        let report = simulate_preset_dry_run(&pkg, &catalog, &sandbox).unwrap();
        assert_eq!(report.total_tweaks, 1);
        assert!(report.unresolvable_conflicts.is_empty());
        assert!(!report.registry_diffs.is_empty());
    }
}
