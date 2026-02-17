#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console in release

mod config;
mod telemetry;
mod tray;
mod audio_feedback;

use anyhow::{anyhow, Context, Result};
use colored::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleFormat;
use hound::{WavSpec, WavWriter};
use rdev::{listen, Event, EventType, Key};
use single_instance::SingleInstance;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const WHISPER_SAMPLE_RATE: u32 = 16000;

fn main() -> Result<()> {
    // 1. Single Instance Check
    let instance = SingleInstance::new("fluidvoice_mvp_instance").unwrap();
    if !instance.is_single() {
        eprintln!("FluidVoice is already running.");
        return Ok(());
    }

    // 2. Load Config & Telemetry
    let config = config::load_config().unwrap_or_default();
    telemetry::load_stats();

    if let Some(key) = &config.openai_api_key {
        std::env::set_var("OPENAI_API_KEY", key);
    } else {
        dotenv::dotenv().ok();
    }

    // 3. Setup Tray & Audio
    // Note: tray-item relies on the main thread loop on Windows usually?
    // Actually tray-item spins its own loop in `new` in some versions or hooks into message pump.
    // Let's keep it alive.
    let _tray_system = tray::SystemTray::new().ok(); // Ignore tray error for now if icon missing

    let audio = Arc::new(audio_feedback::AudioFeedback::new());
    let audio_clone = audio.clone();

    // 4. Setup Hotkey Listener
    let (tx, rx) = mpsc::channel::<()>();
    let modifiers = Arc::new(Mutex::new(Modifiers::default()));
    let modifiers_clone = modifiers.clone();

    thread::spawn(move || {
        let callback = move |event: Event| {
            if let Ok(mut mods) = modifiers_clone.lock() {
                mods.update(&event);
            }
            if let EventType::KeyPress(Key::KeyV) = event.event_type {
                if let Ok(mods) = modifiers_clone.lock() {
                    if mods.ctrl && mods.shift {
                        let _ = tx.send(());
                    }
                }
            }
        };
        if let Err(error) = listen(callback) {
            eprintln!("Error: could not listen to events: {:?}", error);
        }
    });

    println!("{}", "🎤 FluidVoice MVP — Phase 2".bright_green().bold());
    println!("Background Mode Active. Check System Tray.");

    // 5. Main Loop
    loop {
        // We just wait for hotkey trigger
        match rx.recv() {
            Ok(_) => {
                println!("\n{} Hotkey detected!", "⚡".yellow());
                audio_clone.play_start();
                
                let config_duration = config.record_duration_ms;
                let ac = audio_clone.clone();
                // We run pipeline in THIS thread (it blocks tray interaction? maybe)
                // If tray provides "Quit", that runs in a separate callback thread usually.
                // So blocking main thread here is mostly fine for MVP.
                
                match run_pipeline(config_duration) {
                    Ok(_) => ac.play_stop(),
                    Err(e) => {
                        eprintln!("\n{} Pipeline error: {}", "✗".red(), e);
                        ac.play_error();
                        telemetry::record_error();
                    }
                }
                telemetry::save_stats();
            }
            Err(_) => break,
        }
    }
    
    Ok(())
}

// ... Modifiers, run_pipeline helpers ...

#[derive(Default)]
struct Modifiers {
    ctrl: bool,
    shift: bool,
}

impl Modifiers {
    fn update(&mut self, event: &Event) {
        match event.event_type {
            EventType::KeyPress(key) => match key {
                Key::ControlLeft | Key::ControlRight => self.ctrl = true,
                Key::ShiftLeft | Key::ShiftRight => self.shift = true,
                _ => {}
            },
            EventType::KeyRelease(key) => match key {
                Key::ControlLeft | Key::ControlRight => self.ctrl = false,
                Key::ShiftLeft | Key::ShiftRight => self.shift = false,
                _ => {}
            },
            _ => {}
        }
    }
}

fn run_pipeline(duration_ms: u64) -> Result<()> {
    println!("{} Recording...", "⏺".red());

    let (samples, device_sample_rate, device_channels) = record_audio(duration_ms)?;
    
    // Telemetry: Audio collected
    telemetry::record_usage(duration_ms as f64 / 1000.0);

    let mono_samples = to_mono(&samples, device_channels as usize);
    let resampled = if device_sample_rate != WHISPER_SAMPLE_RATE {
        resample(&mono_samples, device_sample_rate, WHISPER_SAMPLE_RATE)
    } else {
        mono_samples
    };

    let wav_path = save_to_wav(&resampled, WHISPER_SAMPLE_RATE)?;
    
    println!("{} Transcribing...", "↻".yellow());
    let transcript = transcribe_openai(&wav_path)?;
    
    println!("{} Typing...", "⌨".green());
    type_text(&transcript)?;
    
    println!("{} Done!", "✓".green());

    Ok(())
}

fn type_text(text: &str) -> Result<()> {
    use enigo::{Enigo, Key, KeyboardControllable};
    let mut enigo = Enigo::new();
    for ch in text.chars() {
        match ch {
            '\n' => enigo.key_click(Key::Return),
            '\t' => enigo.key_click(Key::Tab),
            _ => enigo.key_sequence(&ch.to_string()),
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}

fn record_audio(duration_ms: u64) -> Result<(Vec<f32>, u32, u16)> {
    let host = cpal::default_host();
    let device = host.default_input_device().ok_or_else(|| anyhow!("No input device found."))?;
    let config = device.default_input_config()?;
    let sample_rate = config.sample_rate().0;
    let channels = config.channels();

    let samples = Arc::new(Mutex::new(Vec::new()));
    let samples_writer = Arc::clone(&samples);
    let err_fn = |err: cpal::StreamError| eprintln!("Stream error: {}", err);

    let stream = match config.sample_format() {
        SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _| samples_writer.lock().unwrap().extend_from_slice(data),
            err_fn, None)?,
        SampleFormat::I16 => {
            let sw = Arc::clone(&samples_writer);
            device.build_input_stream(
                &config.into(),
                move |data: &[i16], _| {
                    let mut guard = sw.lock().unwrap();
                    for &s in data { guard.push(s as f32 / i16::MAX as f32); }
                }, err_fn, None)?
        },
        SampleFormat::U16 => {
            let sw = Arc::clone(&samples_writer);
            device.build_input_stream(
                &config.into(),
                move |data: &[u16], _| {
                    let mut guard = sw.lock().unwrap();
                    for &s in data { guard.push((s as f32 / u16::MAX as f32) * 2.0 - 1.0); }
                }, err_fn, None)?
        },
        fmt => return Err(anyhow!("Unsupported sample format: {:?}", fmt)),
    };

    stream.play().context("Failed to start audio stream")?;
    std::thread::sleep(Duration::from_millis(duration_ms));
    drop(stream);

    let result = Arc::try_unwrap(samples).map_err(|_| anyhow!("Failed to extract samples"))?.into_inner()?;
    if result.is_empty() { return Err(anyhow!("No audio captured.")); }
    Ok((result, sample_rate, channels))
}

fn to_mono(samples: &[f32], channels: usize) -> Vec<f32> {
    if channels == 1 { return samples.to_vec(); }
    samples.chunks(channels).map(|frame| frame.iter().sum::<f32>() / channels as f32).collect()
}

fn resample(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    if from_rate == to_rate { return samples.to_vec(); }
    let ratio = from_rate as f64 / to_rate as f64;
    let output_len = (samples.len() as f64 / ratio).ceil() as usize;
    let mut output = Vec::with_capacity(output_len);
    for i in 0..output_len {
        let src_idx = i as f64 * ratio;
        let idx = src_idx as usize;
        let frac = src_idx - idx as f64;
        let s0 = samples[idx.min(samples.len() - 1)];
        let s1 = samples[(idx + 1).min(samples.len() - 1)];
        output.push(s0 + (s1 - s0) * frac as f32);
    }
    output
}

fn save_to_wav(samples: &[f32], sample_rate: u32) -> Result<String> {
    let path = "recording.wav";
    let spec = WavSpec { channels: 1, sample_rate, bits_per_sample: 16, sample_format: hound::SampleFormat::Int };
    let mut writer = WavWriter::create(path, spec)?;
    for &sample in samples {
        let clamped = sample.clamp(-1.0, 1.0);
        writer.write_sample((clamped * i16::MAX as f32) as i16)?;
    }
    writer.finalize()?;
    Ok(path.to_string())
}

fn transcribe_openai(wav_path: &str) -> Result<String> {
    let api_key = std::env::var("OPENAI_API_KEY").context("OPENAI_API_KEY missing")?;
    let client = reqwest::blocking::Client::builder().timeout(Duration::from_secs(30)).build()?;
    let form = reqwest::blocking::multipart::Form::new()
        .file("file", wav_path)?
        .text("model", "whisper-1")
        .text("response_format", "json");
    let response = client.post("https://api.openai.com/v1/audio/transcriptions")
        .bearer_auth(&api_key).multipart(form).send()?;
    if !response.status().is_success() { return Err(anyhow!("OpenAI error: {}", response.status())); }
    let json: serde_json::Value = response.json()?;
    json["text"].as_str().map(|s| s.to_string()).ok_or_else(|| anyhow!("No 'text' field"))
}
