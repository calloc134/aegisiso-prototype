use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum BuildMode {
    Iso,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust 版 mkarchiso")]
pub struct Cli {
    /// 設定ファイルパス
    #[arg(short, long, default_value = "archiso.toml")]
    pub config: String,

    /// ビルドモード (現状 iso 固定)
    #[arg(value_enum, default_value_t = BuildMode::Iso)]
    pub mode: BuildMode,
}
