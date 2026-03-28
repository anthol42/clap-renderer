use std::fmt;
use std::sync::Arc;

use crate::builder::{Command, StyledStr};

/// Trait for customizing clap's automatically generated output pages.
///
/// Both methods return `Option<StyledStr>` — returning `None` means "use
/// clap's built-in output for this page", so you can override only what you
/// need. An empty impl (all `None`) is valid and has no effect.
///
/// Pass an instance to [`Command::with_renderer`] or
/// [`Parser::parse_with_renderer`][crate::Parser::parse_with_renderer].
///
/// # Pages
///
/// | Method | Triggered by |
/// |---|---|
/// | [`render_help`][Renderer::render_help] | `-h` / `--help`, or missing required arg/subcommand |
/// | [`render_version`][Renderer::render_version] | `-V` / `--version` |
///
/// # Example — custom help only
///
/// ```rust
/// # use clap_builder as clap;
/// use clap::{Command, Renderer, builder::StyledStr};
///
/// struct MyRenderer;
///
/// impl Renderer for MyRenderer {
///     fn render_help(&self, cmd: &Command, _use_long: bool) -> Option<StyledStr> {
///         let mut out = StyledStr::new();
///         out.push_str(&format!("Custom help for `{}`\n", cmd.get_name()));
///         Some(out)
///     }
/// }
///
/// Command::new("my-app")
///     .with_renderer(MyRenderer)
///     .get_matches_from(["my-app", "--help"]);
/// ```
///
/// # Example — custom version only
///
/// ```rust
/// # use clap_builder as clap;
/// use clap::{Command, Renderer, builder::StyledStr};
///
/// struct MyRenderer;
///
/// impl Renderer for MyRenderer {
///     fn render_version(&self, cmd: &Command, _use_long: bool) -> Option<StyledStr> {
///         let mut out = StyledStr::new();
///         out.push_str(&format!("v{}\n", cmd.get_version().unwrap_or("unknown")));
///         Some(out)
///     }
/// }
///
/// Command::new("my-app")
///     .version("1.2.3")
///     .with_renderer(MyRenderer)
///     .get_matches_from(["my-app", "--version"]);
/// ```
pub trait Renderer: Send + Sync + 'static {
    /// Render the help page for `cmd`.
    ///
    /// `use_long` is `true` for `--help` and `false` for `-h`, mirroring the
    /// `long_about` / `about` distinction on [`Command`].
    ///
    /// Return `None` to fall back to clap's built-in [`AutoHelp`] /
    /// [`HelpTemplate`] output.
    ///
    /// [`AutoHelp`]: crate::output::AutoHelp
    /// [`HelpTemplate`]: crate::output::HelpTemplate
    fn render_help(&self, _cmd: &Command, _use_long: bool) -> Option<StyledStr> {
        None
    }

    /// Render the version page for `cmd`.
    ///
    /// `use_long` is `true` for `--version` and `false` for `-V`, mirroring
    /// the `long_version` / `version` distinction on [`Command`].
    ///
    /// Return `None` to fall back to clap's built-in `"{name} {version}\n"` format.
    fn render_version(&self, _cmd: &Command, _use_long: bool) -> Option<StyledStr> {
        None
    }
}

/// A clonable, `Debug`-safe wrapper around `Arc<dyn Renderer>`.
///
/// Used as the field type inside [`Command`] so that `Command` can still
/// derive `Clone` and `Debug` without requiring those bounds on `Renderer`.
#[derive(Clone)]
pub(crate) struct ArcRenderer(pub(crate) Arc<dyn Renderer>);

impl ArcRenderer {
    pub(crate) fn new(r: impl Renderer) -> Self {
        ArcRenderer(Arc::new(r))
    }
}

impl fmt::Debug for ArcRenderer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Renderer")
    }
}

impl std::ops::Deref for ArcRenderer {
    type Target = dyn Renderer;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
