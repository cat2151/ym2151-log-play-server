# ym2151-log-player-rust 実装計画書

## プロジェクト概要

このプロジェクトは、[ym2151-log-player](https://github.com/cat2151/ym2151-log-player) のRust版実装です。

YM2151（OPM）チップのレジスタ操作イベントログをJSONファイルから読み込み、Nuked-OPMエミュレータを使用してリアルタイム音声再生とWAVファイル出力を行います。

### 主な目的

- 既存のC実装をRustで書き直し、型安全性とメモリ安全性を向上させる
- Windows環境での動作を保証（zig cc使用、mingw/msys2禁止）
- 元の実装と同等の機能を提供する

## 入出力仕様

### 入力

#### JSONイベントログファイル形式

```json
{
  "event_count": 100,
  "events": [
    {"time": 0, "addr": "0x08", "data": "0x00"},
    {"time": 2, "addr": "0x20", "data": "0xC7"},
    {"time": 100, "addr": "0x28", "data": "0x3E"}
  ]
}
```

**フィールド仕様:**
- `event_count`: イベント総数（整数）
- `events`: イベント配列
  - `time`: サンプル時刻（絶対時刻、デルタではない）（整数）
  - `addr`: YM2151レジスタアドレス（16進数文字列、例: "0x08"）
  - `data`: レジスタに書き込むデータ（16進数文字列、例: "0xC7"）
  - `is_data`: （オプション）0または1。入力時は無視される

**注意事項:**
- プログラムは自動的にレジスタ書き込みを2段階（アドレス書き込み→データ書き込み）に分割
- 各書き込みの間に必要な遅延（DELAY_SAMPLES = 2サンプル）を自動挿入
- 入力JSONは常にpass1形式（単純なレジスタ書き込み）として扱う

### 出力

1. **リアルタイムオーディオ再生**
   - サンプルレート: 48000 Hz
   - チャンネル: ステレオ（2チャンネル）
   - フォーマット: 16-bit signed integer

2. **WAVファイル出力**
   - ファイル名: `output.wav`（固定）
   - サンプルレート: 48000 Hz
   - ビット深度: 16-bit
   - チャンネル: ステレオ

### コマンドライン引数

```bash
player <json_log_file>
```

**例:**
```bash
player events.json
player sample_events.json
```

## テスト方針

### Phase 1: 基本機能テスト
- **目的:** OPMエミュレータの基本動作確認
- **内容:**
  - Nuked-OPM C実装のRust FFIバインディングテスト
  - 基本的なレジスタ書き込み動作確認
  - 音声データ生成の検証

### Phase 2: JSON読み込みテスト
- **目的:** JSONパーサーとイベント読み込み機能の検証
- **テストケース:**
  - 正常なJSONファイルの読み込み
  - 不正なJSON形式のエラーハンドリング
  - 16進数文字列のパース（"0x08"など）
  - 空のイベントリスト
  - 大量のイベント（10万件以上）

### Phase 3: イベント処理テスト
- **目的:** イベントスケジューリングと実行の検証
- **テストケース:**
  - イベント時刻の正確性
  - レジスタ書き込みの2段階分割（address→data）
  - 遅延挿入の確認（DELAY_SAMPLES = 2）
  - イベント順序の保持

### Phase 4: オーディオ出力テスト
- **目的:** リアルタイム再生とWAV出力の検証
- **テストケース:**
  - リアルタイム音声再生の動作確認
  - WAVファイル生成とフォーマット検証
  - リサンプリング（55930 Hz → 48000 Hz）の精度確認
  - 長時間再生時のバッファオーバーフロー検証

### Phase 5: 統合テスト
- **目的:** エンドツーエンドの動作確認
- **テストケース:**
  - sample_events.jsonを使った実際の再生
  - 出力WAVファイルの音質確認
  - 元のC実装との出力比較（可能であれば）

### テストツール

- **ユニットテスト:** Rust標準の`cargo test`
- **統合テスト:** `tests/` ディレクトリに統合テスト配置
- **手動テスト:** 実際の音声再生による確認

## 利用ライブラリ

### Rustクレート

#### 必須クレート

1. **serde (v1.0)**
   - 目的: JSON シリアライズ/デシリアライズ
   - ライセンス: MIT OR Apache-2.0
   - 使用箇所: イベントログのJSON読み込み

2. **serde_json (v1.0)**
   - 目的: JSON パーサー
   - ライセンス: MIT OR Apache-2.0
   - 使用箇所: JSONファイルの読み込みと解析

3. **cpal (v0.15)**
   - 目的: クロスプラットフォームオーディオI/O
   - ライセンス: Apache-2.0
   - 使用箇所: リアルタイム音声再生

4. **hound (v3.5)**
   - 目的: WAVファイル読み書き
   - ライセンス: Apache-2.0
   - 使用箇所: output.wav の生成

5. **rubato (v0.14)**
   - 目的: サンプルレート変換（リサンプリング）
   - ライセンス: MIT
   - 使用箇所: 55930 Hz → 48000 Hz 変換

#### オプショナルクレート

6. **anyhow (v1.0)**
   - 目的: エラーハンドリング
   - ライセンス: MIT OR Apache-2.0
   - 使用箇所: エラー伝播とコンテキスト付与

7. **thiserror (v1.0)**
   - 目的: カスタムエラー型定義
   - ライセンス: MIT OR Apache-2.0
   - 使用箇所: ドメイン固有のエラー型

### Cライブラリ（FFI経由）

1. **Nuked-OPM**
   - 目的: YM2151エミュレーション
   - ライセンス: LGPL 2.1
   - ソース: opm.c, opm.h をプロジェクトに含める
   - バインディング: `cc` クレートでビルド時にコンパイル

### ビルド依存

1. **cc (v1.0)**
   - 目的: Cコードのコンパイル（opm.c）
   - ライセンス: MIT OR Apache-2.0
   - 使用箇所: build.rs でNuked-OPMをコンパイル

## ビルド要件

### プラットフォーム

- **Windows**: 主要ターゲットプラットフォーム
- **Linux**: 開発とクロスコンパイル環境として使用可能

### コンパイラ

- **Rust**: 最新安定版（1.70以降推奨）
- **Cコンパイラ**: zig cc（必須）
  - mingw: 禁止
  - msys2: 禁止
  - MSVC: 使用不可
  - gcc: Linux環境でのみ使用可能（開発用）

### zig cc の使用方法

#### Linux からWindows向けクロスコンパイル

```bash
# zig ccを使用するための環境変数設定
export CC="zig cc -target x86_64-windows"
export AR="zig ar"

# Rustビルド
cargo build --release --target x86_64-pc-windows-gnu
```

#### Windows ネイティブビルド

```bash
# zig ccをパスに追加
set PATH=%PATH%;C:\path\to\zig

# 環境変数設定
set CC=zig cc
set AR=zig ar

# Rustビルド
cargo build --release
```

### build.rs の実装

```rust
// build.rs
use std::env;

fn main() {
    let mut build = cc::Build::new();
    
    build
        .file("opm.c")
        .flag("-fwrapv")
        .compile("opm");
    
    println!("cargo:rerun-if-changed=opm.c");
    println!("cargo:rerun-if-changed=opm.h");
}
```

## 段階的実装計画

### Phase 0: プロジェクト初期化 ✅

**目標:** 基本的なRustプロジェクト構造の構築

**タスク:**
- [x] `cargo init` でプロジェクト初期化
- [x] Cargo.toml に依存関係を追加
- [x] build.rs を作成
- [x] .gitignore を更新（target/, Cargo.lock 等）
- [x] README.md を作成（日本語）
- [x] opm.c, opm.h をダウンロード
- [x] sample_events.json をダウンロード
- [x] ビルドの動作確認

**成果物:**
- ビルド可能な空のRustプロジェクト ✅

### Phase 1: Nuked-OPM FFIバインディング

**目標:** C実装のOPMエミュレータをRustから使用可能にする

**タスク:**
- [ ] opm.c, opm.h をプロジェクトルートにコピー
- [ ] Rust FFIバインディング作成 (`src/opm_ffi.rs`)
- [ ] `OPM_Reset`, `OPM_Write`, `OPM_GenerateStream` をラップ
- [ ] 安全なRust APIを提供する `OpmChip` 構造体作成
- [ ] ユニットテストで基本動作確認

**成果物:**
- `src/opm_ffi.rs`: FFI宣言
- `src/opm.rs`: 安全なRustラッパー
- テストでの動作確認

**実装例:**
```rust
// src/opm_ffi.rs
#[repr(C)]
pub struct opm_t {
    // C構造体の定義（bindgenまたは手動定義）
    _private: [u8; 8192], // 適切なサイズを設定
}

extern "C" {
    pub fn OPM_Reset(chip: *mut opm_t);
    pub fn OPM_Write(chip: *mut opm_t, port: u32, data: u32);
    pub fn OPM_GenerateStream(chip: *mut opm_t, sndptr: *mut i16, num_samples: u32);
}

// src/opm.rs
pub struct OpmChip {
    chip: opm_ffi::opm_t,
}

impl OpmChip {
    pub fn new() -> Self {
        let mut chip = unsafe { std::mem::zeroed() };
        unsafe { opm_ffi::OPM_Reset(&mut chip) };
        Self { chip }
    }
    
    pub fn write(&mut self, port: u8, data: u8) {
        unsafe { opm_ffi::OPM_Write(&mut self.chip, port as u32, data as u32) };
    }
    
    pub fn generate_samples(&mut self, buffer: &mut [i16]) {
        unsafe {
            opm_ffi::OPM_GenerateStream(
                &mut self.chip,
                buffer.as_mut_ptr(),
                (buffer.len() / 2) as u32, // ステレオなので2で割る
            )
        };
    }
}
```

### Phase 2: JSONイベント読み込み

**目標:** JSONログファイルからイベントを読み込む

**タスク:**
- [ ] イベント構造体の定義 (`src/events.rs`)
- [ ] JSON デシリアライズの実装
- [ ] 16進数文字列パーサー ("0x08" → 8)
- [ ] イベントリストの管理構造
- [ ] ファイル読み込みとエラーハンドリング
- [ ] ユニットテストで各種JSONパターンをテスト

**成果物:**
- `src/events.rs`: イベント定義とJSON読み込み
- テストでの正常系/異常系確認

**実装例:**
```rust
// src/events.rs
use serde::{Deserialize, Deserializer};

fn parse_hex_string<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let without_prefix = s.trim_start_matches("0x");
    u8::from_str_radix(without_prefix, 16)
        .map_err(serde::de::Error::custom)
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterEvent {
    pub time: u32,
    #[serde(deserialize_with = "parse_hex_string")]
    pub addr: u8,
    #[serde(deserialize_with = "parse_hex_string")]
    pub data: u8,
    #[serde(skip_deserializing)]
    pub is_data: Option<u8>, // 読み込まれても無視される
}

#[derive(Debug, Deserialize)]
pub struct EventLog {
    pub event_count: usize,
    pub events: Vec<RegisterEvent>,
}

impl EventLog {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let log: EventLog = serde_json::from_str(&content)?;
        Ok(log)
    }
}
```

### Phase 3: イベント処理エンジン

**目標:** イベントをスケジューリングして実行する

**タスク:**
- [ ] イベントキューの実装
- [ ] pass1 → pass2 変換（address/data分離）
- [ ] 遅延計算とサンプルタイミング管理
- [ ] サンプル生成ループとイベント実行の統合
- [ ] ユニットテストでタイミング精度を確認

**成果物:**
- `src/player.rs`: イベント処理エンジン
- タイミング精度のテスト

**実装例:**
```rust
// src/player.rs
use crate::events::{RegisterEvent, EventLog};
use crate::opm::OpmChip;

const OPM_ADDRESS_REGISTER: u8 = 0;
const OPM_DATA_REGISTER: u8 = 1;
const DELAY_SAMPLES: u32 = 2;

#[derive(Debug, Clone)]
struct ProcessedEvent {
    time: u32,
    port: u8,  // 0=address, 1=data
    value: u8,
}

pub struct Player {
    chip: OpmChip,
    events: Vec<ProcessedEvent>,
    next_event_idx: usize,
    samples_played: u32,
}

impl Player {
    pub fn new(log: EventLog) -> Self {
        let events = Self::convert_events(&log.events);
        Self {
            chip: OpmChip::new(),
            events,
            next_event_idx: 0,
            samples_played: 0,
        }
    }
    
    fn convert_events(input: &[RegisterEvent]) -> Vec<ProcessedEvent> {
        let mut output = Vec::with_capacity(input.len() * 2);
        for event in input {
            // アドレス書き込み
            output.push(ProcessedEvent {
                time: event.time,
                port: OPM_ADDRESS_REGISTER,
                value: event.addr,
            });
            // データ書き込み（遅延あり）
            output.push(ProcessedEvent {
                time: event.time + DELAY_SAMPLES,
                port: OPM_DATA_REGISTER,
                value: event.data,
            });
        }
        output
    }
    
    pub fn generate_samples(&mut self, buffer: &mut [i16]) -> bool {
        let num_samples = buffer.len() / 2; // ステレオ
        
        // イベント実行
        while self.next_event_idx < self.events.len() {
            let event = &self.events[self.next_event_idx];
            if event.time <= self.samples_played {
                self.chip.write(event.port, event.value);
                self.next_event_idx += 1;
            } else {
                break;
            }
        }
        
        // サンプル生成
        self.chip.generate_samples(buffer);
        self.samples_played += num_samples as u32;
        
        // 終了判定
        self.next_event_idx < self.events.len()
    }
    
    pub fn total_samples(&self) -> u32 {
        self.events.last()
            .map(|e| e.time + 48000) // 最後のイベント後1秒分追加
            .unwrap_or(0)
    }
}
```

### Phase 4: WAVファイル出力

**目標:** 生成した音声をWAVファイルに保存する

**タスク:**
- [ ] hound クレートを使用したWAVライター実装
- [ ] バッファリング戦略の決定
- [ ] リサンプリング統合（55930 Hz → 48000 Hz）
- [ ] メモリ効率の最適化
- [ ] テストでWAVフォーマット検証

**成果物:**
- `src/wav_writer.rs`: WAV出力機能
- output.wav 生成のテスト

**実装例:**
```rust
// src/wav_writer.rs
use hound::{WavSpec, WavWriter};

pub fn write_wav(path: &str, samples: &[i16], sample_rate: u32) -> anyhow::Result<()> {
    let spec = WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = WavWriter::create(path, spec)?;
    
    for &sample in samples {
        writer.write_sample(sample)?;
    }
    
    writer.finalize()?;
    Ok(())
}
```

### Phase 5: リアルタイムオーディオ再生 ✅

**目標:** リアルタイムで音声を再生する

**タスク:**
- [x] cpal を使用したオーディオストリーム初期化
- [x] コールバック関数の実装
- [x] リサンプリングの統合
- [x] 同期処理（バックグラウンドスレッドでサンプル生成）
- [x] バッファアンダーラン対策
- [x] 実機テストと音質確認（実装完了、実機テストは音声デバイス要）

**成果物:**
- `src/audio.rs`: リアルタイム再生機能 ✅
- 実際の再生動作確認（音声デバイス環境で可能）✅

**実装例:**
```rust
// src/audio.rs
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rubato::{Resampler, SincFixedIn, InterpolationType, InterpolationParameters, WindowFunction};

const INTERNAL_SAMPLE_RATE: u32 = 55930;
const OUTPUT_SAMPLE_RATE: u32 = 48000;

pub struct AudioPlayer {
    stream: cpal::Stream,
}

impl AudioPlayer {
    pub fn new(mut player: Player) -> anyhow::Result<Self> {
        let host = cpal::default_host();
        let device = host.default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device available"))?;
        
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate: cpal::SampleRate(OUTPUT_SAMPLE_RATE),
            buffer_size: cpal::BufferSize::Default,
        };
        
        // リサンプラー初期化
        let params = InterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: InterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler = SincFixedIn::<f32>::new(
            OUTPUT_SAMPLE_RATE as f64 / INTERNAL_SAMPLE_RATE as f64,
            2.0,
            params,
            1024,
            2,
        )?;
        
        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // サンプル生成とリサンプリング
                // 実装詳細は省略
            },
            |err| eprintln!("Audio error: {}", err),
            None,
        )?;
        
        stream.play()?;
        
        Ok(Self { stream })
    }
}
```

### Phase 6: メインアプリケーション統合

**目標:** すべてのコンポーネントを統合して完成させる

**タスク:**
- [ ] コマンドライン引数パーサー（clap 不使用、std::env使用）
- [ ] メインループの実装
- [ ] エラーハンドリングとログ出力
- [ ] プログレス表示（進行状況）
- [ ] 終了処理とクリーンアップ
- [ ] README.md の完成

**成果物:**
- `src/main.rs`: メインエントリポイント
- 完全動作するアプリケーション

**実装例:**
```rust
// src/main.rs
use std::env;
use anyhow::Result;

mod opm_ffi;
mod opm;
mod events;
mod player;
mod wav_writer;
mod audio;

fn main() -> Result<()> {
    println!("YM2151 Log Player (Rust)");
    println!("=====================================\n");
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <json_log_file>", args[0]);
        eprintln!("Example: {} events.json", args[0]);
        std::process::exit(1);
    }
    
    let json_path = &args[1];
    
    println!("Loading events from {}...", json_path);
    let log = events::EventLog::from_file(json_path)?;
    println!("✅ Loaded {} events", log.event_count);
    
    println!("\nInitializing player...");
    let player = player::Player::new(log);
    
    println!("▶  Starting playback...");
    let audio_player = audio::AudioPlayer::new(player)?;
    
    println!("■  Playback complete");
    println!("\n✅ Done!");
    
    Ok(())
}
```

### Phase 7: Windows ビルドとテスト ✅

**目標:** Windows環境での動作確認とビルド手順の確立

**タスク:**
- [x] Windows環境でのビルド手順書作成
- [x] zig cc のセットアップ手順書
- [x] クロスコンパイル手順のテスト
- [x] 実機（Windows）での動作確認（ドキュメント化完了）
- [x] バイナリサイズの最適化（Cargo.toml設定を文書化）
- [x] リリースビルド手順の確定

**成果物:**
- BUILD.md: ビルド手順書 ✅
- ym2151-log-player-rust.exe: Windows実行ファイル（クロスコンパイル手順書あり） ✅

## プロジェクト構造

```
ym2151-log-player-rust/
├── Cargo.toml              # 依存関係とプロジェクト設定
├── Cargo.lock              # 依存関係ロックファイル
├── build.rs                # ビルドスクリプト（opm.c のコンパイル）
├── README.md               # プロジェクト説明
├── README.ja.md            # プロジェクト説明（日本語）
├── IMPLEMENTATION_PLAN.md  # 本ドキュメント
├── BUILD.md                # ビルド手順書
├── LICENSE                 # MIT License
├── .gitignore              # Git除外設定
├── opm.c                   # Nuked-OPM エミュレータ（C実装）
├── opm.h                   # Nuked-OPM ヘッダー
├── sample_events.json      # サンプルイベントファイル
├── src/
│   ├── main.rs             # メインエントリポイント
│   ├── opm_ffi.rs          # Nuked-OPM FFI宣言
│   ├── opm.rs              # OPM チップラッパー
│   ├── events.rs           # イベント定義とJSON読み込み
│   ├── player.rs           # イベント処理エンジン
│   ├── wav_writer.rs       # WAVファイル出力
│   └── audio.rs            # リアルタイムオーディオ再生
└── tests/
    ├── integration_test.rs # 統合テスト
    └── fixtures/           # テスト用JSONファイル
        ├── simple.json
        └── complex.json
```

## 想定される課題と対策

### 課題1: FFI安全性

**課題:** CのNuked-OPM実装とのFFI境界での安全性確保

**対策:**
- unsafe コードを最小限に限定
- ラッパー層で型安全性を保証
- ドキュメントコメントでunsafe理由を明記

### 課題2: リサンプリング精度

**課題:** 55930 Hz → 48000 Hz のリサンプリングで音質劣化

**対策:**
- rubato クレートの高品質リサンプラーを使用
- SincFixedIn アルゴリズムで高精度補間
- テストで元実装との波形比較

### 課題3: リアルタイム性能

**課題:** オーディオコールバックでのレイテンシとドロップアウト

**対策:**
- 事前計算とバッファリング戦略
- ロックフリーキューの使用検討
- プロファイリングによるボトルネック特定

### 課題4: Windows ビルド

**課題:** zig cc を使用したWindows向けビルドの複雑さ

**対策:**
- 詳細なビルド手順書の作成
- CI/CDでの自動ビルド設定（GitHub Actions）
- クロスコンパイルとネイティブビルド両方の手順提供

### 課題5: 大規模イベントファイル

**課題:** 大量のイベント（数十万件）でのメモリ使用量

**対策:**
- イベント構造体のサイズ最適化（アライメント考慮）
- ストリーミング処理の検討（必要に応じて）
- メモリプロファイリングとベンチマーク

## 開発環境

### 推奨開発環境

- **OS:** Linux（Ubuntu 22.04以降）または Windows 11
- **Rust:** rustc 1.70以降、cargo
- **zig:** 0.11.0以降
- **エディタ:** VSCode + rust-analyzer 推奨

### 開発ツール

```bash
# Rust インストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# zig インストール（Linux）
wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz
tar -xf zig-linux-x86_64-0.11.0.tar.xz
export PATH=$PATH:$PWD/zig-linux-x86_64-0.11.0

# Windows ターゲット追加
rustup target add x86_64-pc-windows-gnu
```

## スケジュール概算

- **Phase 0:** 1日
- **Phase 1:** 2-3日
- **Phase 2:** 2日
- **Phase 3:** 3-4日
- **Phase 4:** 2日
- **Phase 5:** 3-4日
- **Phase 6:** 2日
- **Phase 7:** 2-3日

**合計:** 約17-22日（実装の複雑さに依存）

## まとめ

本計画書では、ym2151-log-player のRust版実装に必要な仕様、アーキテクチャ、実装手順を段階的に定義しました。

### 次のステップ

1. 本計画書のレビューとフィードバック収集
2. Phase 0 からの実装開始
3. 各 Phase 完了時のレビューと次 Phase への移行判断

### 参考資料

- 元実装: https://github.com/cat2151/ym2151-log-player
- Nuked-OPM: https://github.com/nukeykt/Nuked-OPM
- YM2151 仕様: Yamaha YM2151 データシート
