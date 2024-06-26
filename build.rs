//! Build script of smve. Currently sets the windows icon resource.

use {
    std::{env, io},
    winres::WindowsResource,
};

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("images/icon.ico")
            .compile()?;
    }
    Ok(())
}
