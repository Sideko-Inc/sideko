use crate::styles::{fmt_green, fmt_red, fmt_yellow};
use log::{error, info, log_enabled, warn};
use spinoff::*;
use std::borrow::Cow;

/// Custom Sideko frames for the spinner
pub struct SidekoSpinner;

impl SidekoSpinner {
    pub fn frames() -> spinners::SpinnerFrames {
        spinner!(
            [
                "⬡⟡⬢", // Gradient progression
                "⟡⬢⟡", // Shifting right
                "⬢⟡⬡", // Continuing shift
                "⟡⬡⟡"  // Reset
            ],
            150
        )
    }
}

/// Wrapper around spinoff::Spinner to handle only
/// showing if log level is INFO
pub struct Spinner {
    sp: Option<spinoff::Spinner>,
}

impl Spinner {
    pub fn new<M: Into<Cow<'static, str>>>(msg: M) -> Self {
        let sp = if log_enabled!(log::Level::Debug) || !log_enabled!(log::Level::Info) {
            // level debug or quiet mode
            info!("{}...", msg.into());
            None
        } else {
            Some(spinoff::Spinner::new(
                SidekoSpinner::frames(),
                msg,
                Color::Green,
            ))
        };
        Self { sp }
    }

    pub fn update_text<M: Into<Cow<'static, str>>>(&mut self, msg: M) {
        if let Some(sp) = self.sp.as_mut() {
            sp.update_text(msg);
        } else {
            info!("{}...", msg.into())
        }
    }

    pub fn stop_success<M: Into<Cow<'static, str>>>(&mut self, msg: M) {
        let symbol = fmt_green("✔");
        if let Some(sp) = self.sp.as_mut() {
            sp.stop_and_persist(&symbol, &msg.into());
        } else {
            info!("{symbol} {}", msg.into());
        }
    }

    pub fn stop_warn<M: Into<Cow<'static, str>>>(&mut self, msg: M) {
        let symbol = fmt_yellow("ø");
        if let Some(sp) = self.sp.as_mut() {
            sp.stop_and_persist(&symbol, &msg.into());
        } else {
            warn!("{symbol} {}", msg.into());
        }
    }

    pub fn stop_error<M: Into<Cow<'static, str>>>(&mut self, msg: M) {
        let symbol = fmt_red("✘");
        if let Some(sp) = self.sp.as_mut() {
            sp.stop_and_persist(&symbol, &msg.into());
        } else {
            error!("{symbol} {}", msg.into());
        }
    }
}
