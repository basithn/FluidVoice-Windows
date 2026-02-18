# FluidVoice MVP (Windows)

Instant voice-to-text dictation for any Windows application. Now runs in the background!

## 🚀 Quick Start

1.  **Run** `fluidvoice-mvp.exe`.
2.  **Check** your System Tray (near the clock). You should see the app running (Note: Icon might be generic/invisible in MVP).
3.  **Press** `Ctrl + Shift + V` anywhere to start recording.
4.  **Hear** a "Beep" sound (Recording Started).
5.  **Speak** for up to 5 seconds.
6.  **Hear** a "Beep-Beep" sound (Recording Stopped).
7.  **Text appears** in your active window!

## 📋 System Requirements
- **Windows 10/11**
- **Visual Studio 2022 C++ Build Tools** (Required for compilation)
- **CMake** (Required for build)
- **Internet** (First run only, to download model)

## 🛑 How to Quit

Since the app runs in the background:
1.  Find the **FluidVoice** icon in the System Tray (near the clock).
2.  Right-click the icon.
3.  Select **Quit**.

## ⚙️ Configuration

The first time you run the app, it creates a `config.toml` file. You can edit this file to change settings:

```toml
hotkey = "Ctrl+Shift+V"   # Change the global hotkey
record_duration_ms = 5000 # Duration in milliseconds (e.g. 10000 for 10s)
# openai_api_key = "sk-..." # Optional: set your API key here
```

## 🔑 API Key (Optional)

FluidVoice MVP now uses **Local Whisper** by default (Offline).
- On first run, it downloads the model (~140MB).
- **No API Key required!**

(Legacy: OpenAI support is currently disabled in code, but can be re-enabled for higher accuracy).

## 📊 Statistics

Your usage stats are saved to `stats.json`. This helps you track how much you've dictated.

## ❓ Troubleshooting

- **"No input device found"**: Check your microphone settings in Windows.
- **"OpenAI error: 401"**: Check your API key.
- **"Icon is missing"**: In this MVP version, the tray icon might be a blank square or hidden. The app is likely still running if you hear beeps!
- **Text types in the wrong place**: Make sure your target window (Notepad, etc.) is focused *before* the transcription finishes.
