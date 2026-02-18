use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
use anyhow::{Result, Context};
use std::path::Path;
use colored::*;

pub struct LocalTranscriber {
    ctx: WhisperContext,
}

impl LocalTranscriber {
    pub fn new(model_path: &Path) -> Result<Self> {
        println!("{} Loading Whisper model...", "⏳".yellow());
        
        let ctx = WhisperContext::new_with_params(
            model_path.to_str().unwrap(), 
            WhisperContextParameters::default()
        ).context("Failed to load Whisper model")?;

        println!("{} Model loaded!", "✓".green());
        Ok(Self { ctx })
    }

    pub fn transcribe(&self, audio_samples: &[f32]) -> Result<String> {
        // Create a state
        let mut state = self.ctx.create_state().context("Failed to create state")?;

        // Configure parameters
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_n_threads(4);
        params.set_language(Some("en"));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        // Run the inference
        state.full(params, audio_samples).context("Failed to run full inference")?;

        // Retrieve the text
        let num_segments = state.full_n_segments();
        let mut text = String::new();

        for i in 0..num_segments {
            // New API in 0.15.1: get_segment returns Option<WhisperSegment>
            // We use expect because i is bounded by num_segments
            let segment = state.get_segment(i).expect("Segment index out of bounds");
            let segment_text = segment.to_str().context("Failed to decode segment text")?;
            text.push_str(segment_text);
            text.push(' ');
        }

        Ok(text.trim().to_string())
    }
}
