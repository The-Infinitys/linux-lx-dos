// src/modules/lx_dos/vm/device/video.rs

use super::super::QemuArgs;

#[derive(Debug)]
pub struct QemuVideo {
    display: VideoDisplay,
}

#[derive(Debug)]
pub enum VideoDisplay {
    SDL,
    GTK,
    VNC { port: u16 },
}

impl QemuVideo {
    pub fn new(display: VideoDisplay) -> Self {
        Self { display }
    }
}

impl QemuArgs for QemuVideo {
    fn to_qemu_args(&self) -> Vec<String> {
        match &self.display {
            VideoDisplay::SDL => vec!["-display".to_string(), "sdl".to_string()],
            VideoDisplay::GTK => vec!["-display".to_string(), "gtk".to_string()],
            VideoDisplay::VNC { port } => vec!["-vnc".to_string(), format!(":{}", port - 5900)],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_sdl() {
        let video = QemuVideo::new(VideoDisplay::SDL);
        assert_eq!(video.to_qemu_args(), vec!["-display", "sdl"]);
    }

    #[test]
    fn test_video_gtk() {
        let video = QemuVideo::new(VideoDisplay::GTK);
        assert_eq!(video.to_qemu_args(), vec!["-display", "gtk"]);
    }

    #[test]
    fn test_video_vnc() {
        let video = QemuVideo::new(VideoDisplay::VNC { port: 5901 });
        assert_eq!(video.to_qemu_args(), vec!["-vnc", ":1"]);
    }
}
