#![allow(dead_code)]

use anyhow::Result;
use hound::{WavReader, WavSpec, WavWriter};
use realfft::RealFftPlanner;
use rustfft::num_complex::Complex32;
use std::collections::HashMap;
use std::path::Path;

/// Audio processing parameters
const SAMPLE_RATE: u32 = 16000;
const FRAME_SIZE: usize = 512;
const HOP_SIZE: usize = 256;
const MFCC_COEFFS: usize = 13;

/// MFCC feature extraction
#[allow(dead_code)]
pub struct MFCCExtractor {
  planner: RealFftPlanner<f32>,
  mel_filters: Vec<Vec<f32>>,
}

#[allow(dead_code)]
impl Default for MFCCExtractor {
  fn default() -> Self {
    Self::new()
  }
}

impl MFCCExtractor {
  pub fn new() -> Self {
    let planner = RealFftPlanner::<f32>::new();
    let mel_filters = Self::create_mel_filters();

    Self {
      planner,
      mel_filters,
    }
  }

  /// Extract MFCC features from audio data
  pub fn extract_features(&mut self, audio_data: &[f32]) -> Result<Vec<Vec<f32>>> {
    let mut features = Vec::new();

    // Pre-emphasis filter
    let pre_emphasized = self.pre_emphasis(audio_data);

    // Frame the audio
    let frames = self.frame_audio(&pre_emphasized);

    for frame in frames {
      // Apply Hamming window
      let windowed = self.apply_hamming_window(&frame);

      // Compute FFT
      let fft = self.compute_fft(&windowed)?;

      // Compute power spectrum
      let power_spectrum = self.compute_power_spectrum(&fft);

      // Apply mel filters
      let mel_energies = self.apply_mel_filters(&power_spectrum);

      // Compute MFCC
      let mfcc = self.compute_mfcc(&mel_energies);

      features.push(mfcc);
    }

    Ok(features)
  }

  fn pre_emphasis(&self, audio: &[f32]) -> Vec<f32> {
    let alpha = 0.97;
    let mut result = Vec::with_capacity(audio.len());

    if audio.is_empty() {
      return result;
    }

    result.push(audio[0]);
    for i in 1..audio.len() {
      result.push(audio[i] - alpha * audio[i - 1]);
    }

    result
  }

  fn frame_audio(&self, audio: &[f32]) -> Vec<Vec<f32>> {
    let mut frames = Vec::new();

    for i in (0..audio.len().saturating_sub(FRAME_SIZE)).step_by(HOP_SIZE) {
      let frame: Vec<f32> = audio[i..i + FRAME_SIZE].to_vec();
      frames.push(frame);
    }

    frames
  }

  fn apply_hamming_window(&self, frame: &[f32]) -> Vec<f32> {
    frame
      .iter()
      .enumerate()
      .map(|(i, &sample)| {
        let window_value =
          0.54 - 0.46 * (2.0 * std::f32::consts::PI * i as f32 / (FRAME_SIZE - 1) as f32).cos();
        sample * window_value
      })
      .collect()
  }

  fn compute_fft(&mut self, windowed: &[f32]) -> Result<Vec<Complex32>> {
    let fft = self.planner.plan_fft_forward(FRAME_SIZE);
    let mut buffer = windowed.to_vec();
    let mut spectrum = vec![Complex32::new(0.0, 0.0); FRAME_SIZE / 2 + 1];

    fft.process(&mut buffer, &mut spectrum)?;

    Ok(spectrum)
  }

  fn compute_power_spectrum(&self, fft: &[Complex32]) -> Vec<f32> {
    fft.iter().map(|c| c.norm_sqr()).collect()
  }

  fn apply_mel_filters(&self, power_spectrum: &[f32]) -> Vec<f32> {
    self
      .mel_filters
      .iter()
      .map(|filter| {
        filter
          .iter()
          .zip(power_spectrum.iter())
          .map(|(f, p)| f * p)
          .sum()
      })
      .collect()
  }

  fn compute_mfcc(&self, mel_energies: &[f32]) -> Vec<f32> {
    // Log mel energies
    let log_mel: Vec<f32> = mel_energies
      .iter()
      .map(|&energy| if energy > 0.0 { energy.ln() } else { -10.0 })
      .collect();

    // DCT to get MFCC coefficients
    let mut mfcc = Vec::with_capacity(MFCC_COEFFS);

    for i in 0..MFCC_COEFFS {
      let mut sum = 0.0;
      for (j, &log_energy) in log_mel.iter().enumerate() {
        let cos_term = (std::f32::consts::PI * i as f32 * (2 * j + 1) as f32
          / (2.0 * log_mel.len() as f32))
          .cos();
        sum += log_energy * cos_term;
      }
      mfcc.push(sum * (2.0 / log_mel.len() as f32).sqrt());
    }

    mfcc
  }

  fn create_mel_filters() -> Vec<Vec<f32>> {
    let num_filters = 26;
    let mut filters = Vec::new();

    for i in 0..num_filters {
      let mut filter = vec![0.0; FRAME_SIZE / 2 + 1];

      // Simplified mel filter bank
      let center = (i + 1) * (FRAME_SIZE / 2) / (num_filters + 1);
      let width = center / 3;

      for (j, item) in filter.iter_mut().enumerate() {
        let distance = (j as i32 - center as i32).abs() as f32;
        if distance <= width as f32 {
          *item = 1.0 - distance / width as f32;
        }
      }

      filters.push(filter);
    }

    filters
  }
}

/// Dynamic Time Warping for alignment
#[allow(dead_code)]
pub struct DTWAligner;

#[allow(dead_code)]
impl DTWAligner {
  pub fn align(seq1: &[Vec<f32>], seq2: &[Vec<f32>]) -> f32 {
    let n = seq1.len();
    let m = seq2.len();

    if n == 0 || m == 0 {
      return f32::INFINITY;
    }

    let mut dtw = vec![vec![f32::INFINITY; m + 1]; n + 1];
    dtw[0][0] = 0.0;

    for i in 1..=n {
      for j in 1..=m {
        let cost = Self::euclidean_distance(&seq1[i - 1], &seq2[j - 1]);
        dtw[i][j] = cost + dtw[i - 1][j].min(dtw[i][j - 1]).min(dtw[i - 1][j - 1]);
      }
    }

    dtw[n][m]
  }

  fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
      .zip(b.iter())
      .map(|(x, y)| (x - y).powi(2))
      .sum::<f32>()
      .sqrt()
  }
}

/// Pronunciation scoring service
pub struct PronunciationScorer {
  mfcc_extractor: MFCCExtractor,
}

impl Default for PronunciationScorer {
  fn default() -> Self {
    Self::new()
  }
}

impl PronunciationScorer {
  pub fn new() -> Self {
    Self {
      mfcc_extractor: MFCCExtractor::new(),
    }
  }

  /// Score pronunciation by comparing user audio with reference
  pub fn score_pronunciation(
    &mut self,
    user_audio: &[f32],
    reference_audio: &[f32],
  ) -> Result<PronunciationScore> {
    // Extract MFCC features
    let user_features = self.mfcc_extractor.extract_features(user_audio)?;
    let reference_features = self.mfcc_extractor.extract_features(reference_audio)?;

    // Align sequences using DTW
    let alignment_cost = DTWAligner::align(&user_features, &reference_features);

    // Convert alignment cost to percentage score
    let max_cost = (user_features.len() + reference_features.len()) as f32 * 10.0;
    let normalized_cost = (alignment_cost / max_cost).min(1.0);
    let percentage_score = (1.0 - normalized_cost) * 100.0;

    // Calculate per-phoneme scores (simplified)
    let per_phoneme = self.calculate_per_phoneme_scores(&user_features, &reference_features)?;

    Ok(PronunciationScore {
      overall_pct: percentage_score.clamp(0.0, 100.0),
      per_phoneme,
      alignment_cost,
      confidence: self.calculate_confidence(percentage_score),
    })
  }

  fn calculate_per_phoneme_scores(
    &self,
    user_features: &[Vec<f32>],
    reference_features: &[Vec<f32>],
  ) -> Result<HashMap<String, f32>> {
    let mut scores = HashMap::new();

    // Simplified per-phoneme scoring
    // In a real implementation, you would use forced alignment
    let num_phonemes = 5; // Simplified assumption

    for i in 0..num_phonemes {
      let phoneme_key = format!("phoneme_{}", i);
      let start_idx = i * user_features.len() / num_phonemes;
      let end_idx = (i + 1) * user_features.len() / num_phonemes;

      if start_idx < user_features.len() && start_idx < reference_features.len() {
        let user_segment = &user_features[start_idx..end_idx.min(user_features.len())];
        let ref_segment = &reference_features[start_idx..end_idx.min(reference_features.len())];

        let segment_cost = DTWAligner::align(user_segment, ref_segment);
        let segment_score = (1.0 - (segment_cost / 10.0).min(1.0)) * 100.0;

        scores.insert(phoneme_key, segment_score.clamp(0.0, 100.0));
      } else {
        scores.insert(phoneme_key, 0.0);
      }
    }

    Ok(scores)
  }

  fn calculate_confidence(&self, score: f32) -> f32 {
    // Confidence based on score consistency
    if score > 80.0 {
      0.9
    } else if score > 60.0 {
      0.7
    } else if score > 40.0 {
      0.5
    } else {
      0.3
    }
  }
}

/// Pronunciation scoring result
#[derive(Debug, Clone)]
pub struct PronunciationScore {
  pub overall_pct: f32,
  pub per_phoneme: HashMap<String, f32>,
  pub alignment_cost: f32,
  pub confidence: f32,
}

/// Audio processing utilities
pub struct AudioProcessor;

impl AudioProcessor {
  /// Convert audio to 16kHz mono for processing
  pub fn preprocess_audio(audio_data: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
    if sample_rate == SAMPLE_RATE {
      return Ok(audio_data.to_vec());
    }

    // Simple resampling (in production, use a proper resampling library)
    let ratio = sample_rate as f32 / SAMPLE_RATE as f32;
    let new_length = (audio_data.len() as f32 / ratio) as usize;
    let mut resampled = Vec::with_capacity(new_length);

    for i in 0..new_length {
      let src_idx = (i as f32 * ratio) as usize;
      if src_idx < audio_data.len() {
        resampled.push(audio_data[src_idx]);
      }
    }

    Ok(resampled)
  }

  /// Load audio from WAV file
  pub fn load_wav_file<P: AsRef<Path>>(path: P) -> Result<Vec<f32>> {
    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();

    let samples: Result<Vec<f32>, _> = reader
      .samples::<i16>()
      .map(|s| s.map(|s| s as f32 / i16::MAX as f32))
      .collect();

    let audio_data = samples?;
    Self::preprocess_audio(&audio_data, spec.sample_rate)
  }

  /// Save audio to WAV file
  pub fn save_wav_file<P: AsRef<Path>>(path: P, audio_data: &[f32]) -> Result<()> {
    let spec = WavSpec {
      channels: 1,
      sample_rate: SAMPLE_RATE,
      bits_per_sample: 16,
      sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(path, spec)?;
    for &sample in audio_data {
      let sample_int = (sample * i16::MAX as f32) as i16;
      writer.write_sample(sample_int)?;
    }
    writer.finalize()?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_mfcc_extraction() {
    let mut extractor = MFCCExtractor::new();
    let audio_data = vec![0.0; 1000]; // Silent audio
    let features = extractor.extract_features(&audio_data).unwrap();
    assert!(!features.is_empty());
    assert_eq!(features[0].len(), MFCC_COEFFS);
  }

  #[test]
  fn test_dtw_alignment() {
    let seq1 = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let seq2 = vec![vec![1.1, 2.1], vec![3.1, 4.1]];
    let cost = DTWAligner::align(&seq1, &seq2);
    assert!(cost < 1.0); // Should be small for similar sequences
  }

  #[test]
  fn test_pronunciation_scoring() {
    let mut scorer = PronunciationScorer::new();
    let user_audio = vec![0.1; 1000];
    let reference_audio = vec![0.1; 1000];

    let score = scorer
      .score_pronunciation(&user_audio, &reference_audio)
      .unwrap();
    assert!(score.overall_pct >= 0.0 && score.overall_pct <= 100.0);
    assert!(score.confidence >= 0.0 && score.confidence <= 1.0);
  }
}
