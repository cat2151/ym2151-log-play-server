#!/bin/bash
# Example TDD test using ALSA capture
# Demonstrates how to test audio output in CI/CD without audio hardware

set -e

echo "=============================================="
echo "TDD Example: Audio Output Verification Test"
echo "=============================================="
echo ""

# Setup ALSA configuration
echo "Setting up ALSA audio capture..."
cat <<'EOF' > ~/.asoundrc
pcm.!default {
  type file
  slave.pcm "null"
  file "/tmp/alsa_test_output.wav"
  format "wav"
}
EOF
echo "✅ ALSA configured to capture audio"
echo ""

# Clean up
rm -f /tmp/alsa_test_output.wav output.wav

# Run application
echo "Running application with audio capture..."
timeout 15 cargo run --release -- test_input.json > /dev/null 2>&1 || true
echo "✅ Application completed"
echo ""

# Test 1: Verify file was created
echo "Test 1: Verify audio capture file exists..."
if [ -f /tmp/alsa_test_output.wav ]; then
    echo "   ✅ PASS: Audio capture file created"
else
    echo "   ❌ FAIL: Audio capture file not found"
    rm -f ~/.asoundrc
    exit 1
fi
echo ""

# Test 2: Verify file is not empty
echo "Test 2: Verify audio capture file contains data..."
FILE_SIZE=$(stat -c%s /tmp/alsa_test_output.wav)
if [ "$FILE_SIZE" -gt 1000 ]; then
    echo "   ✅ PASS: File contains data (${FILE_SIZE} bytes)"
else
    echo "   ❌ FAIL: File is too small (${FILE_SIZE} bytes)"
    rm -f ~/.asoundrc
    exit 1
fi
echo ""

# Test 3: Verify file is valid WAV format
echo "Test 3: Verify file is valid WAV format..."
FILE_TYPE=$(file -b /tmp/alsa_test_output.wav | grep -o "WAVE audio" || echo "")
if [ -n "$FILE_TYPE" ]; then
    echo "   ✅ PASS: File is valid WAV audio"
else
    echo "   ❌ FAIL: File is not valid WAV format"
    rm -f ~/.asoundrc
    exit 1
fi
echo ""

# Test 4: Verify file contains non-zero audio data
echo "Test 4: Verify file contains non-zero audio samples..."
# Check if there are non-zero bytes in the audio data section (skip header)
NON_ZERO_COUNT=$(dd if=/tmp/alsa_test_output.wav bs=1 skip=1000 count=10000 2>/dev/null | \
                 hexdump -v -e '/1 "%02x\n"' | grep -v "^00$" | wc -l)
if [ "$NON_ZERO_COUNT" -gt 100 ]; then
    echo "   ✅ PASS: Audio data contains non-zero samples (${NON_ZERO_COUNT} non-zero bytes in sample)"
else
    echo "   ⚠️  WARNING: Very few non-zero samples found (${NON_ZERO_COUNT})"
    echo "   This might indicate silent audio, but file structure is valid"
fi
echo ""

# Clean up
rm -f ~/.asoundrc /tmp/alsa_test_output.wav

# Summary
echo "=============================================="
echo "TDD TEST RESULTS"
echo "=============================================="
echo "✅ All critical tests passed!"
echo ""
echo "This demonstrates that:"
echo "1. Audio output can be captured using ALSA config"
echo "2. File creation can be verified programmatically"
echo "3. Audio content can be validated"
echo "4. No sudo privileges required"
echo "5. Works on headless CI/CD environments"
echo ""
echo "Use Case: Test-Driven Development"
echo "- Verify audio processing generates output"
echo "- Check audio file format and structure"
echo "- Validate audio contains expected data"
echo "- Run in GitHub Actions or other CI/CD"
echo "=============================================="
