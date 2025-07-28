// src/modules/lx_dos/vm/device/video.rs

use super::super::VmArgs;

#[derive(Debug)]
pub struct VmVideo {
    display: VideoDisplay,
}

#[derive(Debug)]
pub enum VideoDisplay {
    Sdl,
    Gtk,
    Vnc { port: u16 },
}

impl VmVideo {
    pub fn new(display: VideoDisplay) -> Self {
        Self { display }
    }
}

impl VmArgs for VmVideo {
    fn to_vm_args(&self) -> Vec<String> {
        match &self.display {
            VideoDisplay::Sdl => vec!["-display".to_string(), "sdl".to_string()],
            VideoDisplay::Gtk => vec!["-display".to_string(), "gtk".to_string()],
            VideoDisplay::Vnc { port } => vec!["-vnc".to_string(), format!(":{}", port - 5900)],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_sdl() {
        let video = VmVideo::new(VideoDisplay::Sdl);
        assert_eq!(video.to_vm_args(), vec!["-display", "sdl"]);
    }

    #[test]
    fn test_video_gtk() {
        let video = VmVideo::new(VideoDisplay::Gtk);
        assert_eq!(video.to_vm_args(), vec!["-display", "gtk"]);
    }

    #[test]
    fn test_video_vnc() {
        let video = VmVideo::new(VideoDisplay::Vnc { port: 5901 });
        assert_eq!(video.to_vm_args(), vec!["-vnc", ":1"]);
    }
}
