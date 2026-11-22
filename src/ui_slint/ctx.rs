//! Shared handles passed to every view binder.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::app::AppState;
use crate::core::i18n::Language;

use super::MainWindow;

/// Cheap-to-clone bundle of the window handle, the application state and UI-side caches.
#[derive(Clone)]
pub struct Ctx {
    pub window: slint::Weak<MainWindow>,
    pub state: Rc<RefCell<AppState>>,
    /// Language used by the `Tr` lookup. Kept outside the AppState RefCell so string lookups
    /// performed during Slint binding evaluation can never collide with an active borrow.
    pub lang: Rc<Cell<Language>>,
    /// Set by callbacks; the UI tick performs a full sync when true.
    pub dirty: Rc<Cell<bool>>,
}

impl Ctx {
    pub fn new(window: &MainWindow, state: Rc<RefCell<AppState>>) -> Self {
        let lang = state.borrow().language;
        Self {
            window: slint::ComponentHandle::as_weak(window),
            state,
            lang: Rc::new(Cell::new(lang)),
            dirty: Rc::new(Cell::new(true)),
        }
    }

    /// Run a state mutation and mark the UI for re-sync. Returns None if the state is
    /// already borrowed (re-entrant call), in which case the action is skipped and logged.
    pub fn act<R>(&self, f: impl FnOnce(&mut AppState) -> R) -> Option<R> {
        let out = match self.state.try_borrow_mut() {
            Ok(mut s) => {
                s.record_user_interaction();
                Some(f(&mut s))
            }
            Err(_) => {
                tracing::warn!("UI action skipped: AppState already borrowed (re-entrant callback)");
                None
            }
        };
        self.dirty.set(true);
        out
    }
}
