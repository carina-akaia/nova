#!/bin/env bash

echo "Hello, World!" |\
./infrastructure/ml/transformers/tts/piper/piper \
--model "./.config/en_US-libritts-high.onnx" \
--output-file "./_tmp/output.wav" \
\
&& \
\
ffmpeg -y -i "./_tmp/output.wav" -c:a libopus -b:a 96k "./_tmp/output.ogg"
