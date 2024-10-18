#!/usr/bin/env python3

import sys
import os

sys.path.append(os.path.abspath(os.path.join(
                os.path.dirname(__file__), '../../../collection/transformers'))
)  # nopep8
from bark import SAMPLE_RATE, generate_audio, preload_models

from IPython.display import Audio
import json
import subprocess
from scipy.io.wavfile import write as write_wav


os.environ.setdefault("XDG_CACHE_HOME", os.path.abspath(os.path.join(
    os.path.dirname(__file__), "../../../.cache")))

with open(os.path.join(
    os.path.dirname(__file__), "../../config/suno-bark.config.json")
) as config_file:
    config = json.load(config_file)


preload_models()


def say(
    phrase=config["default_prompts"]["text"],
    voice=config["default_prompts"]["voice"],
    temperature=config["default_temperatures"]["text"]
):
    print(f"Saying {phrase} ...")

    audio_array = generate_audio(phrase, voice, temperature)

    write_wav("output.wav", SAMPLE_RATE, audio_array)
    subprocess.run(["mplayer", "output.wav"])

    # Converting to Telegram's native voice message format
    subprocess.run(["ffmpeg", "-y", "-i", "output.wav", "-c:a",
                   "libopus", "-b:a", "96k", "output.ogg"])


if len(sys.argv) > 1:
    say(sys.argv[1])
else:
    say()
