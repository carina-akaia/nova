#!/bin/env bash

echo "A "system" is a set of interconnected components working together toward a common goal or purpose. Systems can be found in technology, biology, organization, and even social structures, and they often have inputs, processes, outputs, and feedback mechanisms.

In computing, an "Operating System" (OS) is a type of system software that manages computer hardware and software resources, providing common services for computer programs. It acts as an intermediary between users and the computer hardware.

As for "Operating Subsystem," while it's not a standard term, it could refer to a subset or component of a larger operating system designed to handle specific tasks or functions. In a complex system, subsystems can exist independently with defined roles, often interacting with other subsystems to contribute to the overall functionality of the main system. This concept is common in areas like aerospace, where larger systems are composed of various subsystems dedicated to propulsion, navigation, etc.

In the context of operating systems, similar concepts might be seen in modular OS designs where different parts (or subsystems) handle elements like memory management, process scheduling, etc. So, you could conceivably have an "Operating Subsystem" as part of a larger, complex system." |\
./infrastructure/ml/transformers/tts/piper/piper \
--model "./.config/en_US-libritts-high.onnx" \
--output-file "./_tmp/output.wav" && \
ffmpeg -y -i "./_tmp/output.wav" -c:a libopus -b:a 96k "./_tmp/output.ogg"
