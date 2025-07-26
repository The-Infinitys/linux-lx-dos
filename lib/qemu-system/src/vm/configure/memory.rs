// src/modules/lx_dos/vm/configure/memory.rs
use std::fmt;
use std::str::FromStr;

use crate::vm::QemuArgs;
use sysinfo::System; // ホストのメモリ情報を取得するために追加 // QemuArgsトレイトを使うためにインポート

pub struct QemuMemory {
    absolute: Option<usize>,
    relative: Option<f64>,
}

// Debug トレイトの実装: デバッグ時に構造体の内容を人間が読める形式で表示
impl fmt::Debug for QemuMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QemuMemory")
            .field("absolute", &self.absolute)
            .field("relative", &self.relative)
            .finish()
    }
}

// Display トレイトの実装: ユーザー向けの表示形式を定義
impl fmt::Display for QemuMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(absolute) = self.absolute {
            // 絶対値の場合、読みやすい単位で表示する
            let giga = QemuMemory::get_giga();
            let mega = QemuMemory::get_mega();
            let kilo = QemuMemory::get_kilo();

            if absolute >= giga && absolute % giga == 0 {
                write!(f, "{}G", absolute / giga)
            } else if absolute >= mega && absolute % mega == 0 {
                write!(f, "{}M", absolute / mega)
            } else if absolute >= kilo && absolute % kilo == 0 {
                write!(f, "{}K", absolute / kilo)
            } else {
                write!(f, "{}B", absolute) // 単位なしの場合はB (バイト)
            }
        } else if let Some(relative) = self.relative {
            // 相対値の場合、パーセンテージで表示
            write!(f, "{}%", relative * 100.0)
        } else {
            // どちらもNoneの場合はエラーまたはデフォルト表示
            write!(f, "Invalid QemuMemory state")
        }
    }
}

impl FromStr for QemuMemory {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let s = s.replace(" ", ""); // スペースを削除

        if let Some(relative_str) = s.strip_suffix("%") {
            // xx% の場合は、relative (相対値)として処理する。
            if let Ok(relative) = f64::from_str(relative_str) {
                if (0.0..=100.0).contains(&relative) {
                    let relative = relative / 100.0;
                    Ok(Self {
                        absolute: None,
                        relative: Some(relative),
                    })
                } else {
                    Err(format!(
                        "{} is an invalid percentage. It must be between 0 and 100.",
                        relative
                    ))
                }
            } else {
                Err(format!("Couldn't convert '{}' as a percentage.", s))
            }
        } else {
            // それ以外の場合は、絶対値として処理する。xxG, xxM, xxK, xxB を処理するようにする。
            let s_lower = s.to_lowercase(); // 単位の判定のために小文字に変換

            let (value_str, multiplier) = if let Some(stripped) = s_lower.strip_suffix("g") {
                (stripped, QemuMemory::get_giga())
            } else if let Some(stripped) = s_lower.strip_suffix("m") {
                (stripped, QemuMemory::get_mega())
            } else if let Some(stripped) = s_lower.strip_suffix("k") {
                (stripped, QemuMemory::get_kilo())
            } else if let Some(stripped) = s_lower.strip_suffix("b") {
                (stripped, 1) // バイト単位
            } else {
                // 単位指定がない場合は数値のみとみなし、バイトとして処理する
                if s_lower.chars().all(|c| c.is_ascii_digit()) {
                    (s_lower.as_str(), 1)
                } else {
                    return Err(format!("Couldn't parse '{}' as a valid memory value.", s));
                }
            };

            if let Ok(value) = usize::from_str(value_str) {
                let absolute = value
                    .checked_mul(multiplier) // 修正: オーバーフローチェックを追加
                    .ok_or_else(|| {
                        format!(
                            "Value {} {} causes an overflow.",
                            value_str,
                            s.chars().last().unwrap_or(' ')
                        )
                    })?;
                Ok(Self {
                    absolute: Some(absolute),
                    relative: None,
                })
            } else {
                Err(format!(
                    "Couldn't convert '{}' as an absolute memory value.",
                    s
                ))
            }
        }
    }
}

impl QemuMemory {
    /// 1キロバイト (KB) をバイト単位で返します。
    fn get_kilo() -> usize {
        1024
    }

    /// 1メガバイト (MB) をバイト単位で返します。
    fn get_mega() -> usize {
        1024 * QemuMemory::get_kilo()
    }

    /// 1ギガバイト (GB) をバイト単位で返します。
    fn get_giga() -> usize {
        1024 * QemuMemory::get_mega()
    }

    /// ホストマシンの総メモリを取得します（バイト単位）。
    fn get_host_total_memory() -> Result<usize, String> {
        let mut system = System::new_all();
        system.refresh_memory();
        let total_memory = system.total_memory();
        if total_memory > 0 {
            // sysinfoはキロバイト単位で返すので、バイトに変換
            Ok((total_memory as usize) * 1024)
        } else {
            Err("Failed to retrieve host total memory.".to_string())
        }
    }

    /// メモリの絶対値を取得します。相対値の場合はホストの総メモリに基づいて計算します。
    pub fn get_absolute_value(&self) -> Result<usize, String> {
        if let Some(abs) = self.absolute {
            Ok(abs)
        } else if let Some(rel) = self.relative {
            // ホストの総メモリを取得して相対値を計算
            let max_memory_bytes = Self::get_host_total_memory()?;
            Ok((max_memory_bytes as f64 * rel).round() as usize)
        } else {
            Err(
                "QemuMemory is in an invalid state (both absolute and relative are None)."
                    .to_string(),
            )
        }
    }
}

impl Default for QemuMemory {
    fn default() -> Self {
        // デフォルトは50%とする
        Self::from_str("50%").unwrap()
    }
}

impl QemuArgs for QemuMemory {
    fn to_qemu_args(&self) -> Vec<String> {
        // メモリ値をQEMUの-mオプションに適した形式（メガバイト単位）で生成
        let memory_bytes = self
            .get_absolute_value()
            .expect("Failed to calculate memory value");
        // QEMUはメガバイト単位を期待するので、バイトをメガバイトに変換
        let memory_mb = memory_bytes / QemuMemory::get_mega();
        vec!["-m".to_string(), memory_mb.to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_percentage() {
        assert_eq!(QemuMemory::from_str("50%").unwrap().relative, Some(0.5));
        assert_eq!(QemuMemory::from_str("100%").unwrap().relative, Some(1.0));
        assert_eq!(QemuMemory::from_str("0%").unwrap().relative, Some(0.0));
        assert!(QemuMemory::from_str("101%").is_err());
        assert!(QemuMemory::from_str("-10%").is_err());
        assert!(QemuMemory::from_str("abc%").is_err());
    }
    #[test]
    fn test_default() {
        let default_mem = QemuMemory::default();
        assert_eq!(default_mem.relative, Some(0.5));
        assert_eq!(default_mem.absolute, None);
    }

    #[test]
    fn test_get_absolute_value_relative() {
        let mem = QemuMemory::from_str("25%").unwrap();
        let host_memory = QemuMemory::get_host_total_memory().unwrap_or(4096 * 1024); // テスト用にデフォルト値を設定
        assert_eq!(
            mem.get_absolute_value().unwrap(),
            (host_memory as f64 * 0.25).round() as usize
        );
    }

    #[test]
    fn test_get_absolute_value_absolute() {
        let mem = QemuMemory::from_str("1G").unwrap();
        assert_eq!(mem.get_absolute_value().unwrap(), 1024 * 1024 * 1024); // max_memory_bytesは関係ない
    }

    #[test]
    fn test_get_absolute_value_zero_max_memory() {
        let mem = QemuMemory::from_str("50%").unwrap();
        // ホストメモリ取得が失敗する場合のテストは環境依存のためスキップ可能
        if let Ok(_host_memory) = QemuMemory::get_host_total_memory() {
            assert!(mem.get_absolute_value().is_ok());
        }
    }

    #[test]
    fn test_debug_output() {
        let mem_abs = QemuMemory::from_str("1G").unwrap();
        assert_eq!(
            format!("{:?}", mem_abs),
            "QemuMemory { absolute: Some(1073741824), relative: None }"
        );

        let mem_rel = QemuMemory::from_str("50%").unwrap();
        assert_eq!(
            format!("{:?}", mem_rel),
            "QemuMemory { absolute: None, relative: Some(0.5) }"
        );
    }

    #[test]
    fn test_display_output() {
        let mem_abs_g = QemuMemory::from_str("2G").unwrap();
        assert_eq!(format!("{}", mem_abs_g), "2G");

        let mem_abs_m = QemuMemory::from_str("512M").unwrap();
        assert_eq!(format!("{}", mem_abs_m), "512M");

        let mem_abs_k = QemuMemory::from_str("256K").unwrap();
        assert_eq!(format!("{}", mem_abs_k), "256K");

        let mem_abs_b = QemuMemory::from_str("1024B").unwrap();
        assert_eq!(format!("{}", mem_abs_b), "1K");

        let mem_abs_no_unit = QemuMemory::from_str("100").unwrap();
        assert_eq!(format!("{}", mem_abs_no_unit), "100B");

        let mem_rel = QemuMemory::from_str("75%").unwrap();
        assert_eq!(format!("{}", mem_rel), "75%");
    }
}
