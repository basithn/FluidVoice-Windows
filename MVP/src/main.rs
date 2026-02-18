#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console in release

mod config;
mod telemetry;
mod tray;
mod audio_feedback;
#[cfg(feature = "local")]
mod model;       
#[cfg(feature = "local")]
mod transcriber;

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
// use std::time::{Duration, Instant}; // Instant unused
use std::time::Duration;

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

    // 3. Setup AI Engine
    #[cfg(feature = "local")]
    let transcriber = {
        println!("{}", "🧠 Phase 3: Initializing Local AI...".cyan());
        let model_path = model::ensure_model_exists().context("Model initialization failed")?;
        Arc::new(transcriber::LocalTranscriber::new(&model_path)?)
    };

    #[cfg(feature = "openai")]
    println!("{}", "☁️ Initializing OpenAI Cloud Mode...".cyan());
    
    // 4. Setup Tray & Audio
    let _tray_system = tray::SystemTray::new().ok(); 
    let audio = Arc::new(audio_feedback::AudioFeedback::new());
    let audio_clone = audio.clone();

    // 5. Setup Hotkey Listener
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

    println!("{}", "🎤 FluidVoice MVP".bright_green().bold());
    #[cfg(feature = "local")]
    println!("Mode: LOCAL Whispher");
    #[cfg(feature = "openai")]
    println!("Mode: CLOUD OpenAI");

    println!("Background Mode Active. Check System Tray.");

    // 6. Main Loop
    loop {
        match rx.recv() {
            Ok(_) => {
                println!("\n{} Hotkey detected!", "⚡".yellow());
                audio_clone.play_start();
                
                let config_duration = config.record_duration_ms;
                let ac = audio_clone.clone();

                #[cfg(feature = "local")]
                let t_clone = transcriber.clone();

                // Build conditional arguments for pipeline
                let pipeline_result = {
                    #[cfg(feature = "local")]
                    { run_pipeline(config_duration, t_clone) }
                    #[cfg(feature = "openai")]
                    { run_pipeline(config_duration, config.openai_api_key.clone()) }
                };

                match pipeline_result {
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

#[derive(Default)]
struct Modifiers {
    ctrl: bool,
    shift: bool,
}

impl Modifiers {
    fn update(&mut self, event: &Event) {
        match event.event_type {
            EventType::KeyPress(Key::ControlLeft) | EventType::KeyPress(Key::ControlRight) => self.ctrl = true,
            EventType::KeyRelease(Key::ControlLeft) | EventType::KeyRelease(Key::ControlRight) => self.ctrl = false,
            EventType::KeyPress(Key::ShiftLeft) | EventType::KeyPress(Key::ShiftRight) => self.shift = true,
            EventType::KeyRelease(Key::ShiftLeft) | EventType::KeyRelease(Key::ShiftRight) => self.shift = false,
            _ => {}
        }
    }
}

// Conditional Pipeline Logic
#[cfg(feature = "local")]
fn run_pipeline(duration_ms: u64, transcriber: Arc<transcriber::LocalTranscriber>) -> Result<()> {
    println!("{} Recording...", "⏺".red());
    let (samples, device_sample_rate, device_channels) = record_audio(duration_ms)?;
    telemetry::record_usage(duration_ms as f64 / 1000.0);

    // Resample for Whisper (16kHz)
    let mono_samples = to_mono(&samples, device_channels as usize);
    let resampled = if device_sample_rate != WHISPER_SAMPLE_RATE {
        resample(&mono_samples, device_sample_rate, WHISPER_SAMPLE_RATE)
    } else {
        mono_samples
    };

    // Debug save
    let _ = save_to_wav(&resampled, WHISPER_SAMPLE_RATE)?;
    
    println!("{} Transcribing (Local)...", "🧠".magenta());
    let transcript = transcriber.transcribe(&resampled)?;
    
    println!("{} Typing...", "⌨".green());
    type_text(&transcript)?;
    println!("{} Done!", "✓".green());
    Ok(())
}

#[cfg(feature = "openai")]
fn run_pipeline(duration_ms: u64, api_key: Option<String>) -> Result<()> {
    println!("{} Recording...", "⏺".red());
    let (samples, device_sample_rate, device_channels) = record_audio(duration_ms)?;
    telemetry::record_usage(duration_ms as f64 / 1000.0);

    // For OpenAI, we need a WAV file.
    // We can use the raw capture rate or resample. 
    // Let's us raw capture rate to save processing (OpenAI handles it).
    // Or stick to 16kHz for consistency. Let's use 16kHz.
    let mono_samples = to_mono(&samples, device_channels as usize);
    let resampled = if device_sample_rate != WHISPER_SAMPLE_RATE {
        resample(&mono_samples, device_sample_rate, WHISPER_SAMPLE_RATE)
    } else {
        mono_samples
    };

    let wav_path = save_to_wav(&resampled, WHISPER_SAMPLE_RATE)?;
    
    println!("{} Transcribing (Cloud)...", "☁️".blue());
    let key = api_key.ok_or_else(|| anyhow!("OpenAI API Key not found in config or env"))?;
    let transcript = transcribe_openai(&wav_path, &key)?;

    println!("{} Typing...", "⌨".green());
    type_text(&transcript)?;
    println!("{} Done!", "✓".green());
    Ok(())
}

#[cfg(feature = "openai")]
fn transcribe_openai(file_path: &str, api_key: &str) -> Result<String> {
    use reqwest::blocking::multipart;
    
    let client = reqwest::blocking::Client::new();
    let form = multipart::Form::new()
        .file("file", file_path)?
        .text("model", "whisper-1");

    let res = client.post("https://api.openai.com/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .context("Failed to connect to OpenAI")?;

    if !res.status().is_success() {
        let error_text = res.text().unwrap_or_default();
        return Err(anyhow!("OpenAI API Error: {}", error_text));
    }

    let json: serde_json::Value = res.json()?;
    let text = json["text"].as_str().unwrap_or("").to_string();
    Ok(text)
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
