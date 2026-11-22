use crate::ui_sync::bitops::{contains_ascii_icase, BitSet256, MAX_TWEAKS};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmationRequirement {
    /// No protected tweaks are currently selected.
    NotRequired,
    /// Protected tweaks are selected and require explicit user confirmation.
    Required {
        protected_count: u32,
        first_protected_index: usize,
        confirmation_token: u64,
    },
    /// The exact active protected subset has been explicitly confirmed.
    Confirmed { protected_count: u32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SelectionMetrics {
    pub total_selected: u32,
    pub visible_selected: u32,
    pub visible_total: u32,
    pub all_visible_selected: bool,
    pub protected_selected: u32,
    pub confirmation: ConfirmationRequirement,
}

/// Static filterable metadata borrowed during query evaluation (zero allocation).
pub struct FilterableEntry<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub category: &'a str,
}

pub struct FilterStrictSelection {
    total_count: usize,
    selected: BitSet256,
    visible: BitSet256,
    protected: BitSet256,
    selectable: BitSet256,
    confirmed_protected: BitSet256,
}

impl FilterStrictSelection {
    pub fn new(total_count: usize, protected: BitSet256, selectable: BitSet256) -> Self {
        let clamped = total_count.min(MAX_TWEAKS);
        let mut all_visible = BitSet256::EMPTY;
        for i in 0..clamped {
            all_visible.set(i);
        }
        Self {
            total_count: clamped,
            selected: BitSet256::EMPTY,
            visible: all_visible,
            protected: protected.and(all_visible),
            selectable: selectable.and(all_visible),
            confirmed_protected: BitSet256::EMPTY,
        }
    }

    /// Recomputes `visible` in-place against a search query and optional category filter.
    /// Returns `true` if the visibility mask changed.
    pub fn apply_filter<'a, I>(
        &mut self,
        query: &str,
        category_filter: Option<&str>,
        entries: I,
    ) -> bool
    where
        I: IntoIterator<Item = FilterableEntry<'a>>,
    {
        let trimmed = query.trim();
        let mut next_visible = BitSet256::EMPTY;

        for (idx, entry) in entries.into_iter().take(self.total_count).enumerate() {
            let cat_ok = match category_filter {
                Some(cat) if !cat.is_empty() => entry.category.eq_ignore_ascii_case(cat),
                _ => true,
            };
            if !cat_ok {
                continue;
            }
            let query_ok = trimmed.is_empty()
                || contains_ascii_icase(entry.title, trimmed)
                || contains_ascii_icase(entry.id, trimmed)
                || contains_ascii_icase(entry.category, trimmed);
            if query_ok {
                next_visible.set(idx);
            }
        }

        let changed = next_visible != self.visible;
        self.visible = next_visible;
        changed
    }

    /// Toggles a single row if it is both visible and selectable.
    /// Returns the bitmask of rows whose selection state changed (at most 1 bit).
    pub fn toggle_row(&mut self, idx: usize) -> BitSet256 {
        if !self.visible.contains(idx) || !self.selectable.contains(idx) {
            return BitSet256::EMPTY;
        }
        let prev = self.selected;
        self.selected.toggle(idx);
        self.prune_confirmation();
        prev.xor(self.selected)
    }

    /// Filter-strict Select All / Deselect All:
    /// - If all selectable visible rows are already selected, deselects only the visible rows.
    /// - Otherwise, sets selection strictly to `visible & selectable` (never selects hidden rows
    ///   and clears stale out-of-filter selections to prevent unintended execution).
    ///
    /// Returns the exact delta bitmask for differential `VecModel::set_row_data` updates.
    pub fn toggle_select_all_filtered(&mut self) -> BitSet256 {
        let scope = self.visible.and(self.selectable);
        if scope.is_empty() {
            return BitSet256::EMPTY;
        }

        let prev = self.selected;
        if scope.is_subset_of(self.selected) {
            self.selected = self.selected.and_not(scope);
        } else {
            self.selected = scope;
        }

        self.prune_confirmation();
        prev.xor(self.selected)
    }

    /// Explicitly clears all selections (visible and hidden). Returns delta mask.
    pub fn clear_all(&mut self) -> BitSet256 {
        let prev = self.selected;
        self.selected = BitSet256::EMPTY;
        self.confirmed_protected = BitSet256::EMPTY;
        prev
    }

    /// Confirms the currently selected protected subset if `token` matches the active mask.
    pub fn confirm_protected(&mut self, token: u64) -> bool {
        let active_protected = self.selected.and(self.protected);
        if active_protected.is_empty() || active_protected.fingerprint() != token {
            return false;
        }
        self.confirmed_protected = active_protected;
        true
    }

    #[inline]
    pub fn visible_mask(&self) -> BitSet256 {
        self.visible
    }

    #[inline]
    pub fn selected_mask(&self) -> BitSet256 {
        self.selected
    }

    #[inline]
    pub fn is_selected(&self, idx: usize) -> bool {
        self.selected.contains(idx)
    }

    #[inline]
    pub fn is_protected(&self, idx: usize) -> bool {
        self.protected.contains(idx)
    }

    #[inline]
    pub fn confirmation_requirement(&self) -> ConfirmationRequirement {
        let active_protected = self.selected.and(self.protected);
        let count = active_protected.count();
        if count == 0 {
            return ConfirmationRequirement::NotRequired;
        }
        if active_protected.is_subset_of(self.confirmed_protected) {
            ConfirmationRequirement::Confirmed {
                protected_count: count,
            }
        } else {
            ConfirmationRequirement::Required {
                protected_count: count,
                first_protected_index: active_protected.first_one().unwrap_or(0),
                confirmation_token: active_protected.fingerprint(),
            }
        }
    }

    pub fn metrics(&self) -> SelectionMetrics {
        let scope = self.visible.and(self.selectable);
        let vis_sel = self.selected.and(self.visible);
        let active_protected = self.selected.and(self.protected);
        SelectionMetrics {
            total_selected: self.selected.count(),
            visible_selected: vis_sel.count(),
            visible_total: self.visible.count(),
            all_visible_selected: !scope.is_empty() && scope.is_subset_of(self.selected),
            protected_selected: active_protected.count(),
            confirmation: self.confirmation_requirement(),
        }
    }

    #[inline]
    fn prune_confirmation(&mut self) {
        // Shrink confirmed mask whenever protected items are unchecked so re-checking
        // them later forces a fresh confirmation dialog.
        self.confirmed_protected = self
            .confirmed_protected
            .and(self.selected.and(self.protected));
    }
}
