use serde::{Deserialize, Serialize};
use std::{
    io::{self, BufReader, BufWriter, Write, Read},
    process::{Child, Command, Stdio},
};
// bincode v2のトレイトをインポート
use bincode::{Encode, Decode};

/// プロセス間通信で送受信されるメッセージの例
/// 任意のenumに置き換え可能です
#[derive(Debug, Serialize, Deserialize, PartialEq, Encode, Decode)] // EncodeとDecodeを追加
pub enum MyMessage {
    Request(String),
    Response(String),
    Quit,
}

/// InstanceServer: 親プロセスが利用し、子プロセスとの通信を管理します。
pub struct InstanceServer {
    child: Child,
    writer: BufWriter<std::process::ChildStdin>,
    reader: BufReader<std::process::ChildStdout>,
}

impl InstanceServer {
    /// 新しいInstanceServerを作成し、子プロセスを起動します。
    /// `child_path` は子プロセスの実行可能ファイルのパスです。
    pub fn new(child_path: &str) -> io::Result<Self> {
        let mut child = Command::new(child_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take().ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Failed to get child stdin pipe")
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Failed to get child stdout pipe")
        })?;

        let writer = BufWriter::new(stdin);
        let reader = BufReader::new(stdout);

        Ok(Self {
            child,
            writer,
            reader,
        })
    }

    /// 子プロセスにメッセージを送信します。
    /// bincode v2でシリアライズされたメッセージの前に、その長さをu64で送信します。
    pub fn send_message<T: Encode>(&mut self, message: &T) -> io::Result<()> {
        let encoded = bincode::encode_to_vec(message, bincode::config::standard())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Bincode encode error: {}", e)))?;
        
        // メッセージの長さを送信
        let len = encoded.len() as u64;
        self.writer.write_all(&len.to_le_bytes())?; // リトルエンディアンでバイト列に変換
        
        // メッセージデータを送信
        self.writer.write_all(&encoded)?;
        self.writer.flush()?;
        Ok(())
    }

    /// 子プロセスからメッセージを受信します。
    /// 最初にu64のメッセージ長を読み込み、その後にメッセージデータを読み込みます。
    // Tにbincode::Decodeトレイト境界を追加し、ジェネリック引数`()`を追加 (ライフタイムは不要)
    pub fn receive_message<T: Decode<()>>(&mut self) -> io::Result<T> {
        // メッセージ長を読み込み
        let mut len_bytes = [0u8; 8]; // u64は8バイト
        self.reader.read_exact(&mut len_bytes)?; 
        let len = u64::from_le_bytes(len_bytes) as usize;

        // メッセージデータを読み込み
        let mut buffer = vec![0u8; len];
        self.reader.read_exact(&mut buffer)?;
        
        // bincode v2のデコード
        let (decoded, _bytes_read) = bincode::decode_from_slice::<T, _>(&buffer, bincode::config::standard())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Bincode decode error: {}", e)))?;
        Ok(decoded)
    }

    /// 子プロセスが終了するのを待ちます。
    pub fn wait_for_child(&mut self) -> io::Result<std::process::ExitStatus> {
        self.child.wait()
    }
}

/// InstanceClient: 子プロセスが利用し、親プロセスとの通信を確立します。
pub struct InstanceClient {
    writer: BufWriter<io::Stdout>,
    reader: BufReader<io::Stdin>,
}

impl InstanceClient {
    /// 新しいInstanceClientを作成し、親プロセスとの通信を確立します。
    pub fn new() -> Self {
        let stdin = io::stdin();
        let stdout = io::stdout();

        let writer = BufWriter::new(stdout);
        let reader = BufReader::new(stdin);

        Self { writer, reader }
    }

    /// 親プロセスにメッセージを送信します。
    /// bincode v2でシリアライズされたメッセージの前に、その長さをu64で送信します。
    pub fn send_message<T: Encode>(&mut self, message: &T) -> io::Result<()> {
        let encoded = bincode::encode_to_vec(message, bincode::config::standard())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Bincode encode error: {}", e)))?;

        // メッセージの長さを送信
        let len = encoded.len() as u64;
        self.writer.write_all(&len.to_le_bytes())?; // リトルエンディアンでバイト列に変換

        // メッセージデータを送信
        self.writer.write_all(&encoded)?;
        self.writer.flush()?;
        Ok(())
    }

    /// 親プロセスからメッセージを受信します。
    /// 最初にu64のメッセージ長を読み込み、その後にメッセージデータを読み込みます。
    // Tにbincode::Decodeトレイト境界を追加し、ジェネリック引数`()`を追加 (ライフタイムは不要)
    pub fn receive_message<T: Decode<()>>(&mut self) -> io::Result<T> {
        // メッセージ長を読み込み
        let mut len_bytes = [0u8; 8]; // u64は8バイト
        self.reader.read_exact(&mut len_bytes)?;
        let len = u64::from_le_bytes(len_bytes) as usize;

        // メッセージデータを読み込み
        let mut buffer = vec![0u8; len];
        self.reader.read_exact(&mut buffer)?;

        // bincode v2のデコード
        let (decoded, _bytes_read) = bincode::decode_from_slice::<T, _>(&buffer, bincode::config::standard())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Bincode decode error: {}", e)))?;
        Ok(decoded)
    }
}