// src/modules/lx_dos/vm/device/audio.rs

use super::super::VmArgs;

#[derive(Debug)]
pub struct VmAudio {
    model: AudioModel,
}

#[derive(Debug)]
pub enum AudioModel {
    AC97,
    SB16,
    Hda,
}

impl VmAudio {
    pub fn new(model: AudioModel) -> Self {
        Self { model }
    }
}

impl VmArgs for VmAudio {
    fn to_vm_args(&self) -> Vec<String> {
        match self.model {
            AudioModel::AC97 => vec!["-soundhw".to_string(), "ac97".to_string()],
            AudioModel::SB16 => vec!["-soundhw".to_string(), "sb16".to_string()],
            AudioModel::Hda => vec!["-soundhw".to_string(), "hda".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_ac97() {
        let audio = VmAudio::new(AudioModel::AC97);
        assert_eq!(audio.to_vm_args(), vec!["-soundhw", "ac97"]);
    }

    #[test]
    fn test_audio_sb16() {
        let audio = VmAudio::new(AudioModel::SB16);
        assert_eq!(audio.to_vm_args(), vec!["-soundhw", "sb16"]);
    }

    #[test]
    fn test_audio_hda() {
        let audio = VmAudio::new(AudioModel::Hda);
        assert_eq!(audio.to_vm_args(), vec!["-soundhw", "hda"]);
    }
}
