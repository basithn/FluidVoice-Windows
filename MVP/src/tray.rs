use tray_item::{IconSource, TrayItem};
use std::sync::{Arc, Mutex};

pub struct SystemTray {
    pub tray: TrayItem,
}

impl SystemTray {
    pub fn new() -> anyhow::Result<Self> {
        let mut tray = TrayItem::new("FluidVoice", IconSource::Resource("main-icon"))
            .map_err(|e| anyhow::anyhow!("Failed to create tray: {}", e))?;

        tray.add_menu_item("Quit", || {
            std::process::exit(0);
        }).map_err(|e| anyhow::anyhow!("Failed to add menu item: {}", e))?;

        // Note: On Windows, IconSource::Resource expects an embedded .ico resource in the EXE.
        // For MVP, if we don't have a build.rs adding resources, this might panic or show no icon.
        // We will try to add a build script next if this fails, or use a default if possible.
        // But tray-item might NOT support IconSource::Data on Windows easily without external crate features.
        
        // Actually, we should check if we can use a simpler approach or if we need a build.rs.
        // For now, let's try. If it breaks, we add build.rs.

        Ok(Self { tray })
    }
}
