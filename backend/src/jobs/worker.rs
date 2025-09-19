use anyhow::Result;
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::{Config, Pool, Runtime};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::db::DbPool;
use crate::services::scoring::PronunciationScorer;

/// Job types for the queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobType {
  PronunciationScoring {
    recording_id: Uuid,
    word_id: Uuid,
    dialect: String,
    audio_url: String,
  },
  SearchIndexUpdate {
    word_id: Uuid,
  },
}

/// Job structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
  pub id: Uuid,
  pub job_type: JobType,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub retry_count: u32,
  pub max_retries: u32,
}

/// Job result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobResult {
  Success { data: serde_json::Value },
  Failure { error: String, retryable: bool },
}

/// Background job worker
#[allow(dead_code)]
pub struct JobWorker {
  redis_pool: Pool,
  db_pool: DbPool,
  scorer: PronunciationScorer,
  running: bool,
}

#[allow(dead_code)]
impl JobWorker {
  pub fn new(redis_url: &str, db_pool: DbPool) -> Result<Self> {
    let config = Config::from_url(redis_url);
    let redis_pool = config.create_pool(Some(Runtime::Tokio1))?;

    Ok(Self {
      redis_pool,
      db_pool,
      scorer: PronunciationScorer::new(),
      running: false,
    })
  }

  /// Start the worker
  pub async fn start(&mut self) -> Result<()> {
    self.running = true;
    info!("Starting background job worker");

    while self.running {
      match self.process_next_job().await {
        Ok(processed) => {
          if !processed {
            // No jobs available, wait before checking again
            sleep(Duration::from_secs(1)).await;
          }
        }
        Err(e) => {
          error!("Error processing job: {}", e);
          sleep(Duration::from_secs(5)).await;
        }
      }
    }

    info!("Background job worker stopped");
    Ok(())
  }

  /// Stop the worker
  pub fn stop(&mut self) {
    self.running = false;
  }

  /// Process the next job in the queue
  async fn process_next_job(&mut self) -> Result<bool> {
    let mut conn = self.redis_pool.get().await?;

    // Pop job from queue (blocking with timeout)
    let result: Option<String> = conn.blpop("job_queue", 1.0).await?;

    if let Some(job_json) = result {
      let job: Job = serde_json::from_str(&job_json)?;
      info!("Processing job: {:?}", job.id);

      match self.execute_job(&job).await {
        Ok(result) => {
          self.handle_job_result(&job, &result).await?;
          info!("Job {} completed successfully", job.id);
        }
        Err(e) => {
          error!("Job {} failed: {}", job.id, e);
          self.handle_job_failure(&job, &e.to_string()).await?;
        }
      }

      return Ok(true);
    }

    Ok(false)
  }

  /// Execute a specific job
  async fn execute_job(&mut self, job: &Job) -> Result<JobResult> {
    match &job.job_type {
      JobType::PronunciationScoring {
        recording_id,
        word_id,
        dialect,
        audio_url,
      } => {
        self
          .process_pronunciation_scoring(*recording_id, *word_id, dialect, audio_url)
          .await
      }
      JobType::SearchIndexUpdate { word_id } => self.process_search_index_update(*word_id).await,
    }
  }

  /// Process pronunciation scoring job
  async fn process_pronunciation_scoring(
    &mut self,
    recording_id: Uuid,
    word_id: Uuid,
    dialect: &str,
    audio_url: &str,
  ) -> Result<JobResult> {
    info!(
      "Processing pronunciation scoring for recording {}",
      recording_id
    );

    // Download audio file (simplified - in production, use proper S3 client)
    let audio_data = self.download_audio(audio_url).await?;

    // Get reference audio for the word
    let reference_audio = self.get_reference_audio(word_id, dialect).await?;

    // Score pronunciation
    let score = self
      .scorer
      .score_pronunciation(&audio_data, &reference_audio)?;

    // Save score to database
    self.save_pronunciation_score(recording_id, &score).await?;

    // Emit WebSocket notification
    self.notify_score_completion(recording_id, &score).await?;

    Ok(JobResult::Success {
      data: serde_json::json!({
          "recording_id": recording_id,
          "overall_pct": score.overall_pct,
          "per_phoneme": score.per_phoneme,
          "confidence": score.confidence
      }),
    })
  }

  /// Process search index update job
  async fn process_search_index_update(&self, word_id: Uuid) -> Result<JobResult> {
    info!("Processing search index update for word {}", word_id);

    // Get word data from database
    let word_data = self.get_word_data(word_id).await?;

    // Update Meilisearch index
    self.update_search_index(&word_data).await?;

    Ok(JobResult::Success {
      data: serde_json::json!({
          "word_id": word_id,
          "status": "updated"
      }),
    })
  }

  /// Download audio file (simplified implementation)
  async fn download_audio(&self, _url: &str) -> Result<Vec<f32>> {
    // In production, this would download from S3/MinIO
    // For now, return dummy audio data
    Ok(vec![0.1; 16000]) // 1 second of audio at 16kHz
  }

  /// Get reference audio for a word
  async fn get_reference_audio(&self, _word_id: Uuid, _dialect: &str) -> Result<Vec<f32>> {
    // In production, this would fetch from database and download from S3
    // For now, return dummy reference audio
    Ok(vec![0.1; 16000])
  }

  /// Save pronunciation score to database
  async fn save_pronunciation_score(
    &self,
    recording_id: Uuid,
    score: &crate::services::scoring::PronunciationScore,
  ) -> Result<()> {
    let per_phoneme_json = serde_json::to_value(&score.per_phoneme)?;

    sqlx::query(
      "INSERT INTO scores (recording_id, overall_pct, per_phoneme, latency_ms) 
             VALUES ($1, $2, $3, $4) 
             ON CONFLICT (recording_id) DO UPDATE SET 
             overall_pct = EXCLUDED.overall_pct,
             per_phoneme = EXCLUDED.per_phoneme,
             latency_ms = EXCLUDED.latency_ms",
    )
    .bind(recording_id)
    .bind(score.overall_pct as f64)
    .bind(per_phoneme_json)
    .bind(1000) // Dummy latency
    .execute(&self.db_pool)
    .await?;

    Ok(())
  }

  /// Notify score completion via WebSocket
  async fn notify_score_completion(
    &self,
    recording_id: Uuid,
    score: &crate::services::scoring::PronunciationScore,
  ) -> Result<()> {
    // In production, this would send WebSocket message
    info!(
      "Score completed for recording {}: {}%",
      recording_id, score.overall_pct
    );
    Ok(())
  }

  /// Get word data for search indexing
  async fn get_word_data(&self, word_id: Uuid) -> Result<serde_json::Value> {
    // In production, this would fetch from database
    Ok(serde_json::json!({
        "id": word_id,
        "text": "example",
        "ipa": "ɪɡˈzæmpəl",
        "dialect": "GA"
    }))
  }

  /// Update Meilisearch index
  async fn update_search_index(&self, word_data: &serde_json::Value) -> Result<()> {
    // In production, this would update Meilisearch
    info!("Updated search index with word data: {}", word_data);
    Ok(())
  }

  /// Handle job result
  async fn handle_job_result(&self, job: &Job, result: &JobResult) -> Result<()> {
    let mut conn = self.redis_pool.get().await?;

    // Store result
    let result_key = format!("job_result:{}", job.id);
    let result_json = serde_json::to_string(result)?;
    conn
      .set_ex::<_, _, ()>(&result_key, result_json, 3600)
      .await?; // 1 hour TTL

    // Remove from processing queue
    let processing_key = format!("job_processing:{}", job.id);
    conn.del::<_, ()>(&processing_key).await?;

    Ok(())
  }

  /// Handle job failure
  async fn handle_job_failure(&self, job: &Job, error: &str) -> Result<()> {
    let mut conn = self.redis_pool.get().await?;

    if job.retry_count < job.max_retries {
      // Retry job
      let mut retry_job = job.clone();
      retry_job.retry_count += 1;

      let job_json = serde_json::to_string(&retry_job)?;
      conn.lpush::<_, _, ()>("job_queue", job_json).await?;

      warn!(
        "Job {} failed, retrying ({}/{})",
        job.id, retry_job.retry_count, job.max_retries
      );
    } else {
      // Mark as permanently failed
      let result = JobResult::Failure {
        error: error.to_string(),
        retryable: false,
      };

      let result_key = format!("job_result:{}", job.id);
      let result_json = serde_json::to_string(&result)?;
      conn
        .set_ex::<_, _, ()>(&result_key, result_json, 3600)
        .await?;

      error!(
        "Job {} permanently failed after {} retries",
        job.id, job.max_retries
      );
    }

    Ok(())
  }
}

/// Job queue manager
#[allow(dead_code)]
pub struct JobQueue {
  redis_pool: Pool,
}

#[allow(dead_code)]
impl JobQueue {
  pub fn new(redis_url: &str) -> Result<Self> {
    let config = Config::from_url(redis_url);
    let redis_pool = config.create_pool(Some(Runtime::Tokio1))?;

    Ok(Self { redis_pool })
  }

  /// Enqueue a new job
  pub async fn enqueue(&self, job: Job) -> Result<()> {
    let mut conn = self.redis_pool.get().await?;

    let job_json = serde_json::to_string(&job)?;
    conn.lpush::<_, _, ()>("job_queue", job_json).await?;

    info!("Enqueued job: {:?}", job.id);
    Ok(())
  }

  /// Get job result
  pub async fn get_result(&self, job_id: Uuid) -> Result<Option<JobResult>> {
    let mut conn = self.redis_pool.get().await?;

    let result_key = format!("job_result:{}", job_id);
    let result_json: Option<String> = conn.get(&result_key).await?;

    if let Some(json) = result_json {
      let result: JobResult = serde_json::from_str(&json)?;
      Ok(Some(result))
    } else {
      Ok(None)
    }
  }

  /// Check if job is completed
  pub async fn is_completed(&self, job_id: Uuid) -> Result<bool> {
    let mut conn = self.redis_pool.get().await?;

    let result_key = format!("job_result:{}", job_id);
    let exists: bool = conn.exists(&result_key).await?;

    Ok(exists)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_job_serialization() {
    let job = Job {
      id: Uuid::new_v4(),
      job_type: JobType::PronunciationScoring {
        recording_id: Uuid::new_v4(),
        word_id: Uuid::new_v4(),
        dialect: "GA".to_string(),
        audio_url: "https://example.com/audio.wav".to_string(),
      },
      created_at: chrono::Utc::now(),
      retry_count: 0,
      max_retries: 3,
    };

    let json = serde_json::to_string(&job).unwrap();
    let deserialized: Job = serde_json::from_str(&json).unwrap();

    assert_eq!(job.id, deserialized.id);
  }
}
