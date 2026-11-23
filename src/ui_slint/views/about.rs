//! Support & About binder (repository links; the style studio reuses AppStore.theme-styles).

use slint::ComponentHandle;

use super::super::ctx::Ctx;
use super::super::{AboutStore, MainWindow};
use super::diagnostics::open_external;

pub const REPO_NAME: &str = "Konstrukturmx / WinBlast-Optimizer";
pub const REPO_URL: &str = "https://github.com/Konstrukturmx/WinBlast-Optimizer";

pub struct AboutView;

impl AboutView {
    pub fn new(window: &MainWindow) -> Self {
        let a = window.global::<AboutStore>();
        a.set_repo_name(REPO_NAME.into());
        a.set_repo_url(REPO_URL.into());
        Self
    }

    pub fn bind(&self, window: &MainWindow, _ctx: &Ctx) {
        window.global::<AboutStore>().on_open_github(|| open_external(REPO_URL));
    }
}
