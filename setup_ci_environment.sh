#!/bin/bash
# Setup script for CI/headless environments
# This script configures ALSA to enable audio output without physical audio devices

set -e

echo "=========================================="
echo "CI Environment Setup for ym2151-log-player"
echo "=========================================="
echo ""

# Create ALSA configuration
echo "Creating ALSA configuration file (~/.asoundrc)..."
cat <<'EOF' > ~/.asoundrc
pcm.!default {
  type file
  slave.pcm "null"
  file "/tmp/alsa_capture.wav"
  format "wav"
}
EOF

echo "✅ ALSA configuration created"
echo ""
echo "Configuration details:"
cat ~/.asoundrc
echo ""
echo "=========================================="
echo "Setup complete!"
echo "=========================================="
echo ""
echo "The program can now run normally:"
echo "  cargo run --release sample_events.json"
echo ""
echo "Audio output will be saved to:"
echo "  - output.wav (direct WAV output)"
echo "  - /tmp/alsa_capture.wav (ALSA capture)"
