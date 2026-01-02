use clap::{Parser, Subcommand};

/// Motion Forge - モーションを変換・管理ツール
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// ファイルを変換する
    Convert {
        /// 入力ファイル
        input: String,

        /// 出力ファイル
        #[arg(short, long)]
        out: Option<String>,
    },

    /// 自己テスト（バイナリやツールのチェック）
    Doctor,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { input, out } => {
            println!("変換開始: {}", input);
            if let Some(out) = out {
                println!("→ 出力: {}", out);
            } else {
                println!("→ 出力: (自動命名)");
            }

            // TODO: ここに “forge” ロジックを入れていく
        }

        Commands::Doctor => {
            println!("環境チェック中…");
            // TODO: 依存ツール確認など
        }
    }

    Ok(())
}
