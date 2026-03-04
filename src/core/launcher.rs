use std::error::Error;

/// Trait for launching URLs in a browser or handler.
///
/// This trait enables backend-agnosticism by abstracting the mechanism
/// for opening URLs. Different implementations can:
/// - Use system default browser (default)
/// - Use custom browser handlers
/// - Launch custom applications
/// - Integrate with specific desktop environments
///
/// # Examples
///
/// ```ignore
/// struct DefaultLauncher;
///
/// impl Launcher for DefaultLauncher {
///     fn launch(&self, url: &str) -> Result<(), Box<dyn Error>> {
///         Command::new("xdg-open").arg(url).spawn()?;
///         Ok(())
///     }
/// }
/// ```
pub trait Launcher {
    /// Launch the given URL.
    ///
    /// # Arguments
    /// * `url` - The URL to launch
    ///
    /// # Returns
    /// * `Ok(())` if launch was successful
    /// * `Err(...)` if launch failed
    fn launch(&self, url: &str) -> Result<(), Box<dyn Error>>;
}

/// Default launcher using omarchy-launch-webapp command.
///
/// This is the default implementation that uses the omarchy desktop
/// environment's custom URL launcher. It's designed to work seamlessly
/// with the Hyprland window manager and related desktop utilities.
pub struct OmarchyLauncher;

impl Launcher for OmarchyLauncher {
    fn launch(&self, url: &str) -> Result<(), Box<dyn Error>> {
        use std::process::Command;
        Command::new("omarchy-launch-webapp").arg(url).spawn()?;
        Ok(())
    }
}
