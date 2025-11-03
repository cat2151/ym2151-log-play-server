// Performance monitoring utilities for identifying audio stuttering causes
//
// This module provides lightweight performance instrumentation to measure
// timing of critical audio rendering operations.

use crate::resampler::{OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE};
use std::time::{Duration, Instant};

/// Performance statistics for a specific operation
#[derive(Debug, Clone)]
pub struct PerfStats {
    /// Operation name
    pub name: String,
    /// Number of samples collected
    pub count: u64,
    /// Total time spent
    pub total_time: Duration,
    /// Minimum time observed
    pub min_time: Duration,
    /// Maximum time observed
    pub max_time: Duration,
    /// Number of times the operation exceeded the threshold
    pub threshold_violations: u64,
}

impl PerfStats {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            count: 0,
            total_time: Duration::ZERO,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
            threshold_violations: 0,
        }
    }

    /// Record a measurement
    pub fn record(&mut self, duration: Duration, threshold: Duration) {
        self.count += 1;
        self.total_time += duration;
        self.min_time = self.min_time.min(duration);
        self.max_time = self.max_time.max(duration);

        if duration > threshold {
            self.threshold_violations += 1;
        }
    }

    /// Get average time
    pub fn avg_time(&self) -> Duration {
        if self.count > 0 {
            // Use checked_div to safely divide by count as u32
            // This avoids overflow when count is large
            self.total_time
                .checked_div(self.count as u32)
                .unwrap_or(Duration::ZERO)
        } else {
            Duration::ZERO
        }
    }

    /// Get violation percentage
    pub fn violation_percentage(&self) -> f64 {
        if self.count > 0 {
            (self.threshold_violations as f64 / self.count as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Format stats as human-readable string
    pub fn format(&self) -> String {
        format!(
            "{}: avg={:.2}ms, min={:.2}ms, max={:.2}ms, violations={}/{} ({:.1}%)",
            self.name,
            self.avg_time().as_secs_f64() * 1000.0,
            self.min_time.as_secs_f64() * 1000.0,
            self.max_time.as_secs_f64() * 1000.0,
            self.threshold_violations,
            self.count,
            self.violation_percentage()
        )
    }
}

/// Performance monitor for audio generation pipeline
pub struct PerfMonitor {
    /// Statistics for OPM sample generation
    pub opm_generation: PerfStats,
    /// Statistics for resampling
    pub resampling: PerfStats,
    /// Statistics for WAV buffer capture
    pub wav_capture: PerfStats,
    /// Statistics for format conversion (i16 to f32)
    pub format_conversion: PerfStats,
    /// Statistics for total iteration time
    pub total_iteration: PerfStats,
    /// Time threshold for warnings (e.g., 10ms for 10ms buffers)
    pub threshold: Duration,
    /// Start time of monitoring
    start_time: Instant,
    /// Actual audio device buffer size (in samples, stereo interleaved)
    audio_buffer_size: Option<usize>,
    /// Generation buffer size (in stereo frames)
    generation_buffer_size: usize,
}

impl PerfMonitor {
    /// Create a new performance monitor
    ///
    /// # Parameters
    /// - `threshold_ms`: Threshold in milliseconds for flagging slow operations
    /// - `audio_buffer_size`: Actual audio device buffer size (in samples, stereo interleaved)
    /// - `generation_buffer_size`: Generation buffer size (in stereo frames)
    pub fn new(
        threshold_ms: u64,
        audio_buffer_size: Option<usize>,
        generation_buffer_size: usize,
    ) -> Self {
        let threshold = Duration::from_millis(threshold_ms);
        Self {
            opm_generation: PerfStats::new("OPM Generation"),
            resampling: PerfStats::new("Resampling"),
            wav_capture: PerfStats::new("WAV Capture"),
            format_conversion: PerfStats::new("Format Conversion"),
            total_iteration: PerfStats::new("Total Iteration"),
            threshold,
            start_time: Instant::now(),
            audio_buffer_size,
            generation_buffer_size,
        }
    }

    /// Print performance report
    pub fn report(&self) {
        let elapsed = self.start_time.elapsed();

        println!("\n=== Performance Report ===");
        println!("Total monitoring time: {:.2}s", elapsed.as_secs_f64());

        // Print buffer configuration
        println!("\n=== Buffer Configuration ===");
        if let Some(buffer_size) = self.audio_buffer_size {
            let buffer_frames = buffer_size / 2;
            let buffer_duration_ms = (buffer_frames as f64 / OUTPUT_SAMPLE_RATE as f64) * 1000.0;
            println!(
                "Audio device buffer: {} samples ({} stereo frames)",
                buffer_size, buffer_frames
            );
            println!(
                "Audio buffer duration: {:.2}ms at {} Hz",
                buffer_duration_ms, OUTPUT_SAMPLE_RATE
            );
        } else {
            println!("Audio device buffer: Unknown (fallback mode)");
        }
        println!(
            "Generation buffer: {} samples ({} stereo frames) at {} Hz",
            self.generation_buffer_size * 2,
            self.generation_buffer_size,
            OPM_SAMPLE_RATE
        );
        let gen_duration_ms =
            (self.generation_buffer_size as f64 / OPM_SAMPLE_RATE as f64) * 1000.0;
        println!("Generation buffer duration: {:.2}ms", gen_duration_ms);

        println!("\n=== Performance Threshold ===");
        println!("Threshold: {:.2}ms", self.threshold.as_secs_f64() * 1000.0);
        if self.audio_buffer_size.is_some() {
            println!("(Based on actual audio device buffer size)");
        } else {
            println!("(Based on generation buffer size - fallback)");
        }
        println!("Rationale: Processing must complete within this time to avoid audio underruns");
        println!();

        // Print stats for each operation
        println!("{}", self.opm_generation.format());
        println!("{}", self.resampling.format());
        println!("{}", self.wav_capture.format());
        println!("{}", self.format_conversion.format());
        println!("{}", self.total_iteration.format());

        println!();

        // Analyze bottlenecks
        let total_avg = self.total_iteration.avg_time().as_secs_f64() * 1000.0;
        if total_avg > 0.0 {
            let opm_pct =
                (self.opm_generation.avg_time().as_secs_f64() * 1000.0 / total_avg) * 100.0;
            let resample_pct =
                (self.resampling.avg_time().as_secs_f64() * 1000.0 / total_avg) * 100.0;
            let wav_pct = (self.wav_capture.avg_time().as_secs_f64() * 1000.0 / total_avg) * 100.0;
            let conv_pct =
                (self.format_conversion.avg_time().as_secs_f64() * 1000.0 / total_avg) * 100.0;

            println!("=== Time Breakdown ===");
            println!("OPM Generation:    {:.1}%", opm_pct);
            println!("Resampling:        {:.1}%", resample_pct);
            println!("WAV Capture:       {:.1}%", wav_pct);
            println!("Format Conversion: {:.1}%", conv_pct);
            println!();
        }

        // Warning if performance requirement not met
        if self.total_iteration.violation_percentage() > 1.0 {
            println!("⚠️  WARNING: Performance requirement NOT met!");
            println!(
                "   {:.1}% of iterations exceeded {:.2}ms threshold",
                self.total_iteration.violation_percentage(),
                self.threshold.as_secs_f64() * 1000.0
            );
            println!("   This likely causes audio stuttering.");
        } else {
            println!("✅ Performance requirement met!");
            println!("   Audio should play smoothly without stuttering.");
        }

        println!("==========================\n");
    }
}

/// Scoped timer for automatic duration measurement
pub struct ScopedTimer<'a> {
    stats: &'a mut PerfStats,
    threshold: Duration,
    start: Instant,
}

impl<'a> ScopedTimer<'a> {
    pub fn new(stats: &'a mut PerfStats, threshold: Duration) -> Self {
        Self {
            stats,
            threshold,
            start: Instant::now(),
        }
    }
}

impl<'a> Drop for ScopedTimer<'a> {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        self.stats.record(duration, self.threshold);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_perf_stats_new() {
        let stats = PerfStats::new("Test Operation");
        assert_eq!(stats.name, "Test Operation");
        assert_eq!(stats.count, 0);
        assert_eq!(stats.threshold_violations, 0);
    }

    #[test]
    fn test_perf_stats_record() {
        let mut stats = PerfStats::new("Test");
        let threshold = Duration::from_millis(10);

        stats.record(Duration::from_millis(5), threshold);
        assert_eq!(stats.count, 1);
        assert_eq!(stats.threshold_violations, 0);

        stats.record(Duration::from_millis(15), threshold);
        assert_eq!(stats.count, 2);
        assert_eq!(stats.threshold_violations, 1);
    }

    #[test]
    fn test_perf_stats_avg_time() {
        let mut stats = PerfStats::new("Test");
        let threshold = Duration::from_millis(10);

        stats.record(Duration::from_millis(5), threshold);
        stats.record(Duration::from_millis(15), threshold);

        let avg = stats.avg_time();
        assert_eq!(avg, Duration::from_millis(10));
    }

    #[test]
    fn test_perf_stats_violation_percentage() {
        let mut stats = PerfStats::new("Test");
        let threshold = Duration::from_millis(10);

        stats.record(Duration::from_millis(5), threshold);
        stats.record(Duration::from_millis(15), threshold);
        stats.record(Duration::from_millis(15), threshold);
        stats.record(Duration::from_millis(5), threshold);

        assert_eq!(stats.violation_percentage(), 50.0);
    }

    #[test]
    fn test_scoped_timer() {
        let mut stats = PerfStats::new("Test");
        let threshold = Duration::from_millis(10);

        {
            let _timer = ScopedTimer::new(&mut stats, threshold);
            thread::sleep(Duration::from_millis(5));
        }

        assert_eq!(stats.count, 1);
        assert!(stats.avg_time() >= Duration::from_millis(4)); // Allow some variance
    }

    #[test]
    fn test_perf_monitor_creation() {
        let monitor = PerfMonitor::new(10, Some(4096), 2048);
        assert_eq!(monitor.threshold, Duration::from_millis(10));
        assert_eq!(monitor.opm_generation.count, 0);
        assert_eq!(monitor.audio_buffer_size, Some(4096));
        assert_eq!(monitor.generation_buffer_size, 2048);
    }
}
