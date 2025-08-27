	/// デフォルトの設定ファイルパスを返す
	pub fn default_path() -> std::path::PathBuf {
		use std::env;
		let config_dir = env::var_os("XDG_CONFIG_HOME")
			.map(PathBuf::from)
			.or_else(dirs::config_dir);
		let mut path = config_dir.unwrap_or_else(|| {
			// fallback: ~/.config
			let mut home = dirs::home_dir().expect("Home directory not found");
			home.push(".config");
			home
		});
		path.push("linux-lx-dos");
		if let Err(e) = std::fs::create_dir_all(&path) {
			eprintln!("Failed to create config dir: {}", e);
		}
		path.push("settings.conf");
		path
	}
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct SettingsData {
	pub key: String,
	pub description: String,
	pub value: String,
}
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Settings {
	map: HashMap<String, String>,
	path: Option<PathBuf>,
}

impl Settings {
	/// 設定の全キー一覧を返す
	pub fn keys(&self) -> Vec<SettingsData> {
		// descriptionはここではkeyと同じにしておく（将来拡張可）
		self.map.iter().map(|(k, v)| SettingsData {
			key: k.clone(),
			description: k.clone(),
			value: v.clone(),
		}).collect()
	}
	/// 新規インスタンス。ファイルパスを指定すると自動でロード
	pub fn new(path: Option<PathBuf>) -> Self {
		let mut s = Settings {
			map: HashMap::new(),
			path,
		};
		if let Some(ref _p) = s.path {
			let _ = s.load();
		}
		s
	}

	/// 設定値を取得
	pub fn get(&self, key: &str) -> Option<&str> {
		self.map.get(key).map(|s| s.as_str())
	}

	/// 設定値を追加・変更
	pub fn set(&mut self, key: &str, value: &str) {
		self.map.insert(key.to_string(), value.to_string());
	}

	/// 設定値を削除
	pub fn remove(&mut self, key: &str) {
		self.map.remove(key);
	}

	/// 設定をファイルに保存
	pub fn save(&self) -> io::Result<()> {
		if let Some(ref path) = self.path {
			let mut file = fs::File::create(path)?;
			for (k, v) in &self.map {
				writeln!(file, "{}={}", k, v)?;
			}
		}
		Ok(())
	}

	/// ファイルから設定を読み込み
	pub fn load(&mut self) -> io::Result<()> {
		if let Some(ref path) = self.path {
			if let Ok(content) = fs::read_to_string(path) {
				self.map.clear();
				for line in content.lines() {
					if let Some((k, v)) = line.split_once('=') {
						self.map.insert(k.trim().to_string(), v.trim().to_string());
					}
				}
			}
		}
		Ok(())
	}
}
