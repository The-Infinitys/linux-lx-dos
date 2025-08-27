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
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsData {
	pub key: String,
	pub description: String,
	pub value: String,
}
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
	pub map: HashMap<String, String>,
	#[serde(skip)]
	pub path: Option<PathBuf>,
}

impl Settings {
	/// デフォルトの設定ファイルパス (~/.local/share/lx-dos/settings.yaml)
	pub fn default_path() -> PathBuf {
		let mut path = dirs::home_dir().expect("Home directory not found");
		path.push(".local/share/lx-dos");
		if let Err(e) = std::fs::create_dir_all(&path) {
			eprintln!("Failed to create config dir: {}", e);
		}
		path.push("settings.yaml");
		path
	}

	/// 規定値で初期化
	pub fn default() -> Self {
		let mut map = HashMap::new();
		// 必要な規定値をここで追加
		map.insert("theme".to_string(), "light".to_string());
		map.insert("language".to_string(), "ja".to_string());
		Settings {
			map,
			path: Some(Self::default_path()),
		}
	}

	/// ファイルから読み込み。なければdefault()
	pub fn open() -> Self {
		let path = Self::default_path();
		if let Ok(content) = std::fs::read_to_string(&path) {
			if let Ok(map) = serde_yaml::from_str::<HashMap<String, String>>(&content) {
				return Settings { map, path: Some(path) };
			}
		}
		Self::default()
	}
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

	/// 設定をファイルに保存（YAML形式）
	pub fn save(&self) -> io::Result<()> {
		if let Some(ref path) = self.path {
			let s = serde_yaml::to_string(&self.map).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
			let mut file = fs::File::create(path)?;
			file.write_all(s.as_bytes())?;
		}
		Ok(())
	}

	/// ファイルから設定を読み込み（YAML形式）
	pub fn load(&mut self) -> io::Result<()> {
		if let Some(ref path) = self.path {
			if let Ok(content) = fs::read_to_string(path) {
				if let Ok(map) = serde_yaml::from_str::<HashMap<String, String>>(&content) {
					self.map = map;
				}
			}
		}
		Ok(())
	}
}
