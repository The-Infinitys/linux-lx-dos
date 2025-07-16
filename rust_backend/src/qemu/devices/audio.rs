use crate::qemu::devices::QemuDevice;

pub struct AudioDevice {
    driver: String, // e.g., "pa", "alsa", "sdl"
    model: Option<String>, // e.g., "ac97", "hda"
}

impl AudioDevice {
    pub fn new(driver: String, model: Option<String>) -> Self {
        AudioDevice { driver, model }
    }
}

impl QemuDevice for AudioDevice {
    fn to_qemu_args(&self) -> Vec<String> {
        let mut args = vec![format!("-audiodev {},id=audio0", self.driver)];
        if let Some(model) = &self.model {
            args.push(format!("-device {},audiodev=audio0", model));
        }
        args
    }
}
