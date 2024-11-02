#!/bin/env bash

echo "You can actually have some fun with an NPU! If you make it work..." |\
./infrastructure/ml/transformers/tts/piper/piper \
--model "./.config/en_US-libritts-high.onnx" \
--output-file "./_tmp/output.wav" && \
ffmpeg -y -i "./_tmp/output.wav" -c:a libopus -b:a 96k "./_tmp/output.ogg"
