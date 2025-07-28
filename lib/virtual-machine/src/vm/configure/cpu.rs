use super::VmArgs;
use std::fmt;
use std::str::FromStr;

/// QEMU CPUの設定を表す構造体
pub struct VmCpu {
    pub cores: VmCpuCores, // CPUコアの設定
    pub model: String,       // CPUモデル名 (例: "host")
}

/// QEMU CPUコアの数を相対値または絶対値で表す構造体
pub struct VmCpuCores {
    relative: Option<f64>,   // 相対値 (例: 0.5 for 50%)
    absolute: Option<usize>, // 絶対値 (例: 4 cores)
}

impl VmCpuCores {
    /// QEMUの-smpオプションで使用するコアの絶対数を文字列で返します。
    /// 相対値の場合は、ホストのCPUコア数に基づいて計算します。
    pub fn get(&self) -> usize {
        // TODO: 実際のアプリケーションでは、ホストのCPUコア数を動的に取得する必要があります。
        // 例: num_cpus::get().
        // ここでは仮に8コアとします。
        let host_cores_count = std::thread::available_parallelism()
            .expect("Error while getting the number of available threads")
            .get(); // 仮のホストCPUコア数

        if let Some(absolute) = self.absolute {
            absolute
        } else if let Some(relative) = self.relative {
            // ホストのコア数に対する相対値を計算し、最も近い整数に丸めます。
            let calculated_cores = (relative * host_cores_count as f64).round() as usize;
            // 少なくとも1コアは確保するようにする (QEMUの-smpは0を許容しない場合があるため)
            calculated_cores.max(1)
        } else {
            // どちらも設定されていない場合は0コアとして扱うか、エラーを返す
            // FromStrの実装により、この状態にはならないはずですが、念のため
            0
        }
    }
}

impl Default for VmCpu {
    /// VmCpuのデフォルト値を定義します。
    /// デフォルトは50%のCPUコアを使用します。
    fn default() -> Self {
        VmCpu::from_str("50%").unwrap()
    }
}

impl VmArgs for VmCpu {
    fn to_vm_args(&self) -> Vec<String> {
        vec![
            "-cpu".to_string(),
            self.model.to_string(),
            "-smp".to_string(), // "-smp" も String に変換
            self.cores.get().to_string(),
        ]
    }
}

impl FromStr for VmCpu {
    type Err = String;

    /// 文字列からVmCpuインスタンスをパースします。
    /// "50%" のような相対値、または "4" のような絶対値をサポートします。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let s = s.replace(" ", ""); // スペースを削除

        if let Some(relative_str) = s.strip_suffix("%") {
            // xx% の場合は、relative (相対値)として処理する。
            if let Ok(relative) = f64::from_str(relative_str) {
                if (0.0..=100.0).contains(&relative) {
                    let relative = relative / 100.0;
                    Ok(Self {
                        cores: VmCpuCores {
                            relative: Some(relative),
                            absolute: None,
                        },
                        model: "host".to_string(),
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
        } else if let Ok(value) = usize::from_str(s.as_str()) {
            // 絶対値 (例: "4" コア) として処理する。
            let absolute = value;
            return Ok(Self {
                cores: VmCpuCores {
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

impl fmt::Display for VmCpu {
    /// VmCpuの表示形式を定義します。
    /// 例: "cores=50%" または "cores=4"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        // coresが指定されていない、または0の場合は表示しない
        // VmCpuCoresがusizeと比較できるようにPartialOrd<usize>を実装する必要がある
        if self.cores > 0 {
            parts.push(format!("cores={}", self.cores));
        }
        // socketsが1の場合は省略 (このフィールドは現在VmCpuに存在しないが、コメントとして残す)
        // partsが空の場合 (例: cores=0 の場合など)
        if parts.is_empty() {
            write!(f, "cores=0") // coresが0でも最低限表示
        } else {
            write!(f, "{}", parts.join(","))
        }
    }
}

impl fmt::Debug for VmCpu {
    /// VmCpuのデバッグ表示形式を定義します。Displayと同じ形式を使用します。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// VmCpuCoresの表示形式を定義します。
// 相対値の場合はパーセンテージ、絶対値の場合はそのままの数を表示します。
impl fmt::Display for VmCpuCores {
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

// VmCpuCoresとusizeの等価性を定義します。
// 主に `self.cores > 0` のような比較のために使用されます。
impl PartialEq<usize> for VmCpuCores {
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
            // 0 以外のusizeとの比較は、現状のVmCpuCoresの設計では曖昧なため、等しくないと判断
            false
        }
    }
}

// VmCpuCoresとusizeの順序比較を定義します。
// 主に `self.cores > 0` のような比較のために使用されます。
impl PartialOrd<usize> for VmCpuCores {
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
            // 0 以外のusizeとの比較は、現状のVmCpuCoresの設計では曖昧なため、比較不能と判断
            None
        }
    }
}

// VmCpuCoresの等価性を定義します (VmCpuCores同士の比較)。
// これにより、VmCpuCores == VmCpuCores の比較が可能になります。
impl PartialEq for VmCpuCores {
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

// VmCpuCoresの順序比較を定義します (VmCpuCores同士の比較)。
// これにより、VmCpuCores < VmCpuCores の比較などが可能になります。
impl PartialOrd for VmCpuCores {
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
impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X86_64 => "x86_64",
                Self::Arm64 => "arm64",
            }
        )
    }
}
impl VmArgs for Architecture {
    fn to_vm_args(&self) -> Vec<String> {
        vec![format!("vm-system-{}", self)]
    }
}
