//! Audio system configuration constants
//!
//! This module contains all audio-related buffer sizes, timing offsets,
//! and other configuration parameters that affect audio latency and performance.

/// Audio buffer configuration
pub mod buffer {
    /// Size of audio generation buffer (samples, stereo)
    /// Affects latency vs stability tradeoff
    // pub const GENERATION_BUFFER_SIZE: usize = 2048;
    pub const GENERATION_BUFFER_SIZE: usize = 1024; // 1024は時折ザッという音で途切れた。512は常時途切れた。

    /// Number of chunks in sync channel between audio thread and generation thread
    /// Higher values = more buffering = higher latency but more stability
    // pub const SYNC_CHANNEL_CAPACITY: usize = 8;
    pub const SYNC_CHANNEL_CAPACITY: usize = 1; // 途切れは確認されず

    /// Resampling chunk size for high-quality mode
    // pub const RESAMPLING_CHUNK_SIZE: usize = 1024;
    pub const RESAMPLING_CHUNK_SIZE: usize = 1; // 0でフリーズ。1はOK。1以上であればどんな値でもふるまいは変化なし。後続処理input_frames_next()で上書きされる。少しでも遅延時間が増えるリスク（の検討時間）を減らすため、最小値にしておく。

    /// CPAL buffer size configuration
    /// Using Fixed size for precise latency control
    /// Lower values = lower latency but higher risk of audio dropouts
    // pub const CPAL_BUFFER_SIZE: cpal::BufferSize = cpal::BufferSize::Default;
    pub const CPAL_BUFFER_SIZE: cpal::BufferSize = cpal::BufferSize::Fixed(1); // 1に設定しても実際は2112 samplesが使われた。4096にしたら8192 samplesが使われたので、2112 samplesが下限キャップであると判断する。少しでも遅延時間が増えるリスク（の検討時間）を減らすため、最小値にしておく。
}

/// Timing and scheduling configuration
pub mod timing {
    /// Future scheduling offset for interactive mode (seconds)
    /// Must be larger than total buffer latency to prevent audio dropouts
    pub const FUTURE_SCHEDULING_OFFSET_SEC: f64 = 0.030; // 上記のバッファ数値をagentが実装した段階では400ms必要だったが、削ったら30msでもOKになった。20msは遅延発生（この場合の遅延とはverboseログで遅延と表示されて音が崩れる現象のこと）

    /// Audio system stabilization wait time (milliseconds)
    pub const AUDIO_STABILIZATION_WAIT_MS: u64 = 1;
}
