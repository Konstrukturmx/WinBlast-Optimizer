//! CPU vendor, topology, and hybrid core detection.
//!
//! Uses Win32 `GetLogicalProcessorInformationEx(RelationProcessorCore, ...)`
//! (`EfficiencyClass`) alongside `raw_cpuid` and `sysinfo` to detect processor capabilities.

use raw_cpuid::CpuId;
use sysinfo::System;

/// Win32 hardware core topology parsed from `RelationProcessorCore`.
#[derive(Debug, Clone)]
pub(crate) struct Win32CoreTopology {
    pub physical_cores: usize,
    pub has_hybrid: bool,
    pub p_core_count: usize,
    pub e_core_count: usize,
    /// Per-logical-processor label (`"P-Core"`, `"E-Core"`, or `"Core"`).
    pub logical_core_types: Vec<&'static str>,
}

/// Pure helper mapping `(efficiency_class, logical_indices)` records into `Win32CoreTopology`.
fn classify_efficiency_records(
    records: &[(u8, Vec<usize>)],
    total_logical_hint: usize,
) -> Option<Win32CoreTopology> {
    if records.is_empty() {
        return None;
    }

    let min_eff = records.iter().map(|(e, _)| *e).min().unwrap_or(0);
    let max_eff = records.iter().map(|(e, _)| *e).max().unwrap_or(0);
    let has_hybrid = max_eff > min_eff;

    let mut max_logical_idx = total_logical_hint;
    for (_, indices) in records {
        if let Some(&m) = indices.iter().max() {
            max_logical_idx = max_logical_idx.max(m + 1);
        }
    }

    let mut logical_core_types = vec!["Core"; max_logical_idx];
    let mut p_core_count = 0usize;
    let mut e_core_count = 0usize;

    for (eff, indices) in records {
        let label = if has_hybrid {
            if *eff == max_eff {
                p_core_count += 1;
                "P-Core"
            } else {
                e_core_count += 1;
                "E-Core"
            }
        } else {
            "Core"
        };
        for &idx in indices {
            if idx < logical_core_types.len() {
                logical_core_types[idx] = label;
            }
        }
    }

    Some(Win32CoreTopology {
        physical_cores: records.len(),
        has_hybrid,
        p_core_count,
        e_core_count,
        logical_core_types,
    })
}

/// Query exact per-core `EfficiencyClass` topology via Win32 `GetLogicalProcessorInformationEx`.
pub(crate) fn query_win32_core_topology(total_logical_hint: usize) -> Option<Win32CoreTopology> {
    use windows::Win32::System::SystemInformation::{
        GetLogicalProcessorInformationEx, RelationProcessorCore, GROUP_AFFINITY,
        PROCESSOR_RELATIONSHIP, SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX,
    };
    use windows::Win32::System::Threading::{
        GetActiveProcessorCount, GetActiveProcessorGroupCount,
    };

    let mut len = 0u32;
    let _ = unsafe { GetLogicalProcessorInformationEx(RelationProcessorCore, None, &mut len) };
    if len == 0 {
        return None;
    }

    let mut buf = vec![0u8; len as usize];
    if unsafe {
        GetLogicalProcessorInformationEx(
            RelationProcessorCore,
            Some(buf.as_mut_ptr().cast::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX>()),
            &mut len,
        )
    }
    .is_err()
    {
        return None;
    }

    let group_count = unsafe { GetActiveProcessorGroupCount() } as usize;
    let mut group_bases = Vec::with_capacity(group_count.max(1));
    let mut running_base = 0usize;
    for g in 0..group_count {
        group_bases.push(running_base);
        running_base += unsafe { GetActiveProcessorCount(g as u16) } as usize;
    }

    let valid_len = (len as usize).min(buf.len());
    let mut offset = 0usize;
    let mut records: Vec<(u8, Vec<usize>)> = Vec::new();
    let processor_offset = std::mem::offset_of!(SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, Anonymous);
    let min_proc_record_size = processor_offset + std::mem::size_of::<PROCESSOR_RELATIONSHIP>();

    while offset + 8 <= valid_len {
        let ptr = unsafe { buf.as_ptr().add(offset) as *const SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX };
        let rel = unsafe { (*ptr).Relationship };
        let size = unsafe { (*ptr).Size } as usize;
        if size < 8 || offset + size > valid_len {
            break;
        }
        if rel == RelationProcessorCore && size >= min_proc_record_size {
            let proc_rel = unsafe { &(*ptr).Anonymous.Processor };
            let g_cnt = proc_rel.GroupCount as usize;
            let needed_size = min_proc_record_size
                + g_cnt.saturating_sub(1) * std::mem::size_of::<GROUP_AFFINITY>();
            if size >= needed_size && g_cnt > 0 {
                let masks = unsafe { std::slice::from_raw_parts(proc_rel.GroupMask.as_ptr(), g_cnt) };
                let mut logical_indices = Vec::new();
                for ga in masks {
                    let base = group_bases.get(ga.Group as usize).copied().unwrap_or(0);
                    let mut mask = ga.Mask;
                    while mask != 0 {
                        let bit = mask.trailing_zeros() as usize;
                        logical_indices.push(base + bit);
                        mask &= mask - 1;
                    }
                }
                records.push((proc_rel.EfficiencyClass, logical_indices));
            }
        }
        offset += size;
    }

    classify_efficiency_records(&records, total_logical_hint.max(running_base))
}

/// Detected CPU information.
#[derive(Debug, Clone)]
pub struct CpuInfo {
    /// CPU vendor string (e.g., "GenuineIntel", "AuthenticAMD").
    pub vendor: String,
    /// Full model name (e.g., "13th Gen Intel(R) Core(TM) i9-13900K").
    pub model_name: String,
    /// Number of physical cores.
    pub physical_cores: usize,
    /// Number of logical processors (includes hyper-threads).
    pub logical_cores: usize,
    /// Whether the CPU has a hybrid topology (P-cores + E-cores).
    pub has_hybrid_topology: bool,
    /// Number of performance cores (0 if not hybrid or unknown).
    pub p_core_count: usize,
    /// Number of efficiency cores (0 if not hybrid or unknown).
    pub e_core_count: usize,
}

impl CpuInfo {
    /// Detect the CPU in this system.
    pub fn detect() -> Self {
        let cpuid = CpuId::new();

        let vendor = cpuid
            .get_vendor_info()
            .map(|v| v.as_str().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let model_name = cpuid
            .get_processor_brand_string()
            .map(|b| b.as_str().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());

        let mut sys = System::new();
        sys.refresh_cpu_all();

        let logical_cores = sys.cpus().len();
        let fallback_physical = sys.physical_core_count().unwrap_or(logical_cores);

        let (physical_cores, has_hybrid, p_cores, e_cores) =
            if let Some(topo) = query_win32_core_topology(logical_cores) {
                (
                    topo.physical_cores.max(1),
                    topo.has_hybrid,
                    topo.p_core_count,
                    topo.e_core_count,
                )
            } else {
                // Safe fallback only when logical > physical and asymmetric HT reveals P vs E
                // (never compute p_cores = 0 when logical == physical on non-HT hybrid CPUs).
                let can_infer_ht_split = vendor == "GenuineIntel"
                    && fallback_physical > 0
                    && logical_cores > fallback_physical
                    && logical_cores != fallback_physical * 2;
                if can_infer_ht_split {
                    let p = logical_cores.saturating_sub(fallback_physical);
                    let e = (2 * fallback_physical).saturating_sub(logical_cores);
                    (fallback_physical, p > 0 && e > 0, p, e)
                } else {
                    (fallback_physical, false, 0, 0)
                }
            };

        Self {
            vendor,
            model_name,
            physical_cores,
            logical_cores,
            has_hybrid_topology: has_hybrid,
            p_core_count: p_cores,
            e_core_count: e_cores,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_efficiency_records_hybrid_and_non_ht_and_uniform() {
        // Intel Core Ultra Series 2 (e.g. 8 P-cores without HT + 16 E-cores = 24 logical == 24 physical)
        let mut records = Vec::new();
        for i in 0..8 {
            records.push((1u8, vec![i]));
        }
        for i in 8..24 {
            records.push((0u8, vec![i]));
        }
        let ultra = classify_efficiency_records(&records, 24).unwrap();
        assert!(ultra.has_hybrid);
        assert_eq!(ultra.physical_cores, 24);
        assert_eq!(ultra.p_core_count, 8);
        assert_eq!(ultra.e_core_count, 16);
        assert_eq!(ultra.logical_core_types[0], "P-Core");
        assert_eq!(ultra.logical_core_types[8], "E-Core");

        // Uniform non-hybrid CPU (single EfficiencyClass = 0)
        let uniform_recs: Vec<(u8, Vec<usize>)> = (0..8).map(|i| (0u8, vec![i * 2, i * 2 + 1])).collect();
        let ryzen = classify_efficiency_records(&uniform_recs, 16).unwrap();
        assert!(!ryzen.has_hybrid);
        assert_eq!(ryzen.physical_cores, 8);
        assert_eq!(ryzen.p_core_count, 0);
        assert_eq!(ryzen.e_core_count, 0);
        assert!(ryzen.logical_core_types.iter().all(|&l| l == "Core"));
    }

    #[test]
    fn test_cpu_detect_host_safe() {
        let info = CpuInfo::detect();
        assert!(info.logical_cores >= 1);
        assert!(info.physical_cores >= 1);
        if info.has_hybrid_topology {
            assert!(info.p_core_count > 0);
        }
    }
}
