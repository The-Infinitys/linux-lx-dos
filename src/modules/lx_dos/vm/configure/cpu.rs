use std::fmt;
use std::str::FromStr;

/// QEMU CPUの設定を表す構造体
pub struct QemuCpu {
    pub cores: QemuCpuCores, // CPUコアの設定
    pub model: String,       // CPUモデル名 (例: "host")
}

/// QEMU CPUコアの数を相対値または絶対値で表す構造体
pub struct QemuCpuCores {
    relative: Option<f64>,   // 相対値 (例: 0.5 for 50%)
    absolute: Option<usize>, // 絶対値 (例: 4 cores)
}

impl Default for QemuCpu {
    /// QemuCpuのデフォルト値を定義します。
    /// デフォルトは50%のCPUコアを使用します。
    fn default() -> Self {
        QemuCpu::from_str("50%").unwrap()
    }
}

impl FromStr for QemuCpu {
    type Err = String;

    /// 文字列からQemuCpuインスタンスをパースします。
    /// "50%" のような相対値、または "4" のような絶対値をサポートします。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let s = s.replace(" ", ""); // スペースを削除

        if let Some(relative_str) = s.strip_suffix("%") {
            // xx% の場合は、relative (相対値)として処理する。
            if let Ok(relative) = f64::from_str(relative_str) {
                if (0.0..=100.0).contains(&relative) {
                    let relative = relative / 100.0;
                    return Ok(Self {
                        cores: QemuCpuCores {
                            relative: Some(relative),
                            absolute: None,
                        },
                        model: "host".to_string(),
                    });
                } else {
                    return Err(format!(
                        "{} is an invalid percentage. It must be between 0 and 100.",
                        relative
                    ));
                }
            } else {
                return Err(format!("Couldn't convert '{}' as a percentage.", s));
            }
        } else if let Ok(value) = usize::from_str(s.as_str()) {
            // 絶対値 (例: "4" コア) として処理する。
            let absolute = value;
            return Ok(Self {
                cores: QemuCpuCores {
                    relative: None,
                    absolute: Some(absolute),
                },
                model: "host".to_string(),
            });
        } else {
            return Err(format!(
                "Couldn't convert '{}' as an absolute cpu cores value.",
                s
            ));
        }
    }
}

impl fmt::Display for QemuCpu {
    /// QemuCpuの表示形式を定義します。
    /// 例: "cores=50%" または "cores=4"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        // coresが指定されていない、または0の場合は表示しない
        // QemuCpuCoresがusizeと比較できるようにPartialOrd<usize>を実装する必要がある
        if self.cores > 0 {
            parts.push(format!("cores={}", self.cores));
        }
        // socketsが1の場合は省略 (このフィールドは現在QemuCpuに存在しないが、コメントとして残す)
        // partsが空の場合 (例: cores=0 の場合など)
        if parts.is_empty() {
            write!(f, "cores=0") // coresが0でも最低限表示
        } else {
            write!(f, "{}", parts.join(","))
        }
    }
}

impl fmt::Debug for QemuCpu {
    /// QemuCpuのデバッグ表示形式を定義します。Displayと同じ形式を使用します。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// QemuCpuCoresの表示形式を定義します。
// 相対値の場合はパーセンテージ、絶対値の場合はそのままの数を表示します。
impl fmt::Display for QemuCpuCores {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(relative) = self.relative {
            // 相対値の場合はパーセンテージとして表示 (例: 50%)
            write!(f, "{}%", (relative * 100.0) as u64)
        } else if let Some(absolute) = self.absolute {
            // 絶対値の場合はそのまま表示 (例: 4)
            write!(f, "{}", absolute)
        } else {
            // どちらもNoneの場合は、予期せぬ状態としてエラーまたはデフォルト値を表示
            // FromStrの実装により、この状態にはならないはずですが、念のため記述
            write!(f, "invalid_cores_state")
        }
    }
}

// QemuCpuCoresとusizeの等価性を定義します。
// 主に `self.cores > 0` のような比較のために使用されます。
impl PartialEq<usize> for QemuCpuCores {
    fn eq(&self, other: &usize) -> bool {
        if *other == 0 {
            // 0 との比較の場合
            if let Some(absolute) = self.absolute {
                absolute == 0
            } else if let Some(relative) = self.relative {
                // 相対値が0.0%の場合のみ0と等しいとみなす
                relative == 0.0
            } else {
                false // この状態はFromStrの実装により発生しないはず
            }
        } else {
            // 0 以外のusizeとの比較は、現状のQemuCpuCoresの設計では曖昧なため、等しくないと判断
            false
        }
    }
}

// QemuCpuCoresとusizeの順序比較を定義します。
// 主に `self.cores > 0` のような比較のために使用されます。
impl PartialOrd<usize> for QemuCpuCores {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        if *other == 0 {
            // 0 との比較の場合
            if let Some(absolute) = self.absolute {
                Some(absolute.cmp(other))
            } else if let Some(relative) = self.relative {
                // 相対値が0.0%より大きい場合は0より大きい
                if relative > 0.0 {
                    Some(std::cmp::Ordering::Greater)
                } else if relative == 0.0 {
                    Some(std::cmp::Ordering::Equal)
                } else {
                    // 相対値が負になることはFromStrの実装によりないはず
                    Some(std::cmp::Ordering::Less)
                }
            } else {
                None // この状態はFromStrの実装により発生しないはず
            }
        } else {
            // 0 以外のusizeとの比較は、現状のQemuCpuCoresの設計では曖昧なため、比較不能と判断
            None
        }
    }
}

// QemuCpuCoresの等価性を定義します (QemuCpuCores同士の比較)。
// これにより、QemuCpuCores == QemuCpuCores の比較が可能になります。
impl PartialEq for QemuCpuCores {
    fn eq(&self, other: &Self) -> bool {
        match (self.absolute, self.relative, other.absolute, other.relative) {
            (Some(self_abs), None, Some(other_abs), None) => self_abs == other_abs, // 両方絶対値
            (None, Some(self_rel), None, Some(other_rel)) => {
                (self_rel - other_rel).abs() < f64::EPSILON
            } // 両方相対値 (浮動小数点数の比較)
            _ => false, // 異なる型の比較は等しくないとみなす
        }
    }
}

// QemuCpuCoresの順序比較を定義します (QemuCpuCores同士の比較)。
// これにより、QemuCpuCores < QemuCpuCores の比較などが可能になります。
impl PartialOrd for QemuCpuCores {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.absolute, self.relative, other.absolute, other.relative) {
            (Some(self_abs), None, Some(other_abs), None) => Some(self_abs.cmp(&other_abs)), // 両方絶対値
            (None, Some(self_rel), None, Some(other_rel)) => self_rel.partial_cmp(&other_rel), // 両方相対値
            _ => None, // 異なる型の比較は順序付け不能とみなす
        }
    }
}

#[derive(Debug)]
pub enum Architecture {
    X86_64,
    Arm64,
}

impl Default for Architecture {
    fn default() -> Self {
        match std::env::consts::ARCH {
            "x86_64" => Self::X86_64,
            "arm64" => Self::Arm64,
            // fall back
            _ => Self::X86_64,
        }
    }
}
