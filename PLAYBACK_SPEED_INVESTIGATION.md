# リアルタイム再生速度問題の調査レポート

## 問題の概要

リアルタイム再生が想定外のハイスピードで動作している:
- **期待値**: 2.5秒の再生時間
- **実際**: 1.3秒で再生完了
- **加速率**: 約1.92倍 (2.5 / 1.3 ≈ 1.92)

## 調査アプローチ

### 1. アーキテクチャの分析

現在の実装は以下の構造:

```
[生成スレッド] ---(SyncChannel)---> [オーディオコールバック]
     ↓
  55930 Hz
  2048 frames
     ↓
 [リサンプラー]
     ↓
  48000 Hz
  ~1758 frames
```

#### 生成スレッド (`generate_samples_thread`)
- 2048ステレオフレーム (4096サンプル) を55930 Hzで生成
- 各生成サイクルの時間: 36.62 ms
- リサンプリング後: 約1758ステレオフレーム (約3515サンプル) at 48000 Hz
- リサンプリング後の時間: 36.62 ms (時間は保持される)

#### オーディオコールバック
- 設定: `cpal::BufferSize::Fixed(4224)`
- コールバック関数が受け取る `data: &mut [f32]` の長さを確認する必要あり
- `try_recv()` で channel から**1つのバッファ**を受信

### 2. 問題の特定

現在の実装 (`src/audio.rs` 行153-177):

```rust
if let Ok(samples) = sample_rx.try_recv() {
    let len = data.len().min(samples.len());
    data[..len].copy_from_slice(&samples[..len]);
    
    // 残りは無音で埋める
    if len < data.len() {
        data[len..].fill(0.0);
    }
}
```

**重要な問題点**:
- 各コールバックは**1つの完全なバッファ**のみを受信
- バッファサイズが不足していれば、残りを無音で埋める
- 次のコールバックは**新しいバッファ**を取得（継続せず）

## 根本原因の仮説

### 仮説A: バッファサイズ不一致によるサンプルスキップ

`cpal::BufferSize::Fixed(4224)` の解釈により2つのシナリオ:

#### シナリオ1: 4224 = フレーム数
- `data.len()` = 8448サンプル (4224フレーム × 2チャンネル)
- 生成は約3515サンプルを提供
- 使用率: 3515 / 8448 = 41.6%
- **理論的な加速率: 8448 / 3515 = 2.40倍**

#### シナリオ2: 4224 = サンプル数
- `data.len()` = 4224サンプル
- 生成は約3515サンプルを提供
- 使用率: 3515 / 4224 = 83.2%
- **理論的な加速率: 4224 / 3515 = 1.20倍**

**シナリオ1が報告された1.92倍に近い値を示す**

### 仮説B: サンプル消費のタイミング

オーディオデバイスは固定レートでコールバックを呼び出す:
- デバイスバッファ4224フレーム (8448サンプル) の場合: 88ms毎
- デバイスバッファ2112フレーム (4224サンプル) の場合: 44ms毎

各コールバックで:
1. 約3515サンプルを受信
2. デバイスバッファを埋める必要がある
3. 不足分を無音で埋める
4. **結果**: オーディオコンテンツが本来より短い時間で再生される

## 診断機能の実装

`src/audio.rs` に以下の診断カウンターを追加:

1. **callback_count**: 総コールバック回数
2. **samples_received_total**: 生成から受信したサンプル総数
3. **samples_used_total**: 実際に使用されたサンプル数
4. **samples_silenced_total**: 無音で埋められたサンプル数

再生完了後、以下の情報を出力:

```
=== Audio Playback Diagnostics ===
Total audio callbacks: N
Audio callback buffer size: X samples (Y stereo frames)
Audio callback buffer duration: Z.ZZ ms

Sample statistics:
  Samples received from generation: NNNN
  Samples actually used: NNNN
  Samples filled with silence: NNNN
  Usage percentage: XX.X%

Timing analysis:
  Audio content played: X.XX seconds
  Total callback time: Y.YY seconds
  *** Speed-up factor: Z.ZZx ***
```

## 予想される診断結果

実際のオーディオデバイスでテストした場合、以下のような結果が予想される:

### ケース1: バッファサイズ = 8448サンプル
```
Audio callback buffer size: 8448 samples (4224 stereo frames)
Samples received: ~288000
Samples used: ~288000
Samples silenced: ~403000
Usage percentage: 41.6%
Speed-up factor: ~2.40x
```

### ケース2: バッファサイズ = 4224サンプル
```
Audio callback buffer size: 4224 samples (2112 stereo frames)
Samples received: ~288000
Samples used: ~288000
Samples silenced: ~58000
Usage percentage: 83.2%
Speed-up factor: ~1.20x
```

## 次のステップ

1. **実環境でのテスト**
   - 実際のオーディオデバイスで診断ログを確認
   - `data.len()` の実際の値を確認
   - 加速係数を測定

2. **根本原因の確認**
   - 診断結果から実際のバッファサイズを特定
   - サンプル使用率と加速係数の相関を確認

3. **修正案の検討**
   - オーディオコールバックでの複数バッファ消費
   - バッファサイズの調整
   - リサンプリング出力サイズの最適化

## 参考: 正しい実装案

オーディオコールバックで複数のバッファを消費するように修正:

```rust
let mut offset = 0;
while offset < data.len() {
    if let Ok(samples) = sample_rx.try_recv() {
        let remaining = data.len() - offset;
        let len = remaining.min(samples.len());
        data[offset..offset+len].copy_from_slice(&samples[..len]);
        offset += len;
        
        // バッファに残りがある場合は次のイテレーションで使用
        // (部分的に消費されたバッファの管理が必要)
    } else {
        // サンプルがない場合のみ無音で埋める
        data[offset..].fill(0.0);
        break;
    }
}
```

この修正により、オーディオデバイスバッファが常に利用可能なオーディオデータで完全に埋められ、
無音による時間の「水増し」が防止されます。

## 結論

診断機能により、以下が可視化されます:
- 実際のオーディオバッファサイズ
- サンプルの使用率と廃棄率
- 正確な加速係数
- 無音サンプルの割合

これらの情報から、ハイスピード再生の正確な原因を特定し、適切な修正を実装できます。
