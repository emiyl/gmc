mod android;
mod html5;
mod ios;
mod linux;
mod mac;
mod main;
mod operagx;
mod reddit;
mod tvos;
mod windows;

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Options {
    android: android::AndroidOptions,
    html5: html5::Html5Options,
    ios: ios::IosOptions,
    linux: linux::LinuxOptions,
    mac: mac::MacOptions,
    main: main::MainOptions,
    operagx: operagx::OperaGXOptions,
    reddit: reddit::RedditOptions,
    tvos: tvos::TvOSOptions,
    windows: windows::WindowsOptions,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            android: android::AndroidOptions::default(),
            html5: html5::Html5Options::default(),
            ios: ios::IosOptions::default(),
            linux: linux::LinuxOptions::default(),
            mac: mac::MacOptions::default(),
            main: main::MainOptions::default(),
            operagx: operagx::OperaGXOptions::default(),
            reddit: reddit::RedditOptions::default(),
            tvos: tvos::TvOSOptions::default(),
            windows: windows::WindowsOptions::default(),
        }
    }
}

impl Options {
    pub fn new(name: &str) -> Self {
        let options = Self {
            android: android::AndroidOptions::default(),
            html5: html5::Html5Options::default(),
            ios: ios::IosOptions::new(name),
            linux: linux::LinuxOptions::new(name),
            mac: mac::MacOptions::new(name),
            main: main::MainOptions::new(name),
            operagx: operagx::OperaGXOptions::default(),
            reddit: reddit::RedditOptions::default(),
            tvos: tvos::TvOSOptions::new(name),
            windows: windows::WindowsOptions::new(name),
        };
        options
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        fn get_platform_path(options_dir: &Path, platform: &str) -> std::path::PathBuf {
            let platform_dir = options_dir.join(platform);
            platform_dir.join(format!("options_{}.yy", platform))
        }

        let android = android::AndroidOptions::load(get_platform_path(path.as_ref(), "android"))?;
        let html5 = html5::Html5Options::load(get_platform_path(path.as_ref(), "html5"))?;
        let ios = ios::IosOptions::load(get_platform_path(path.as_ref(), "ios"))?;
        let linux = linux::LinuxOptions::load(get_platform_path(path.as_ref(), "linux"))?;
        let mac = mac::MacOptions::load(get_platform_path(path.as_ref(), "mac"))?;
        let main = main::MainOptions::load(get_platform_path(path.as_ref(), "main"))?;
        let operagx = operagx::OperaGXOptions::load(get_platform_path(path.as_ref(), "operagx"))?;
        let reddit = reddit::RedditOptions::load(get_platform_path(path.as_ref(), "reddit"))?;
        let tvos = tvos::TvOSOptions::load(get_platform_path(path.as_ref(), "tvos"))?;
        let windows = windows::WindowsOptions::load(get_platform_path(path.as_ref(), "windows"))?;

        Ok(Self {
            android,
            html5,
            ios,
            linux,
            mac,
            main,
            operagx,
            reddit,
            tvos,
            windows,
        })
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let options_dir = path.as_ref();

        if !options_dir.exists() {
            std::fs::create_dir_all(options_dir)?;
        }

        // should be saved to options/<platform>/options_<platform>.yy)
        fn get_platform_path(options_dir: &Path, platform: &str) -> std::path::PathBuf {
            let platform_dir = options_dir.join(platform);
            if !platform_dir.exists() {
                std::fs::create_dir_all(&platform_dir).unwrap();
            }
            platform_dir.join(format!("options_{}.yy", platform))
        }

        self.android
            .save(get_platform_path(options_dir, "android"))?;
        self.html5.save(get_platform_path(options_dir, "html5"))?;
        self.ios.save(get_platform_path(options_dir, "ios"))?;
        self.linux.save(get_platform_path(options_dir, "linux"))?;
        self.mac.save(get_platform_path(options_dir, "mac"))?;
        self.main.save(get_platform_path(options_dir, "main"))?;
        self.operagx
            .save(get_platform_path(options_dir, "operagx"))?;
        self.reddit.save(get_platform_path(options_dir, "reddit"))?;
        self.tvos.save(get_platform_path(options_dir, "tvos"))?;
        self.windows
            .save(get_platform_path(options_dir, "windows"))?;
        Ok(())
    }
}
