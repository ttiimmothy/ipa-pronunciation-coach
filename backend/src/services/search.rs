#![allow(dead_code)]

use anyhow::Result;
use meilisearch_sdk::{client::Client, indexes::Index, search::SearchQuery};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Search service for vocabulary
pub struct SearchService {
    client: Client,
    vocab_index: Index,
}

/// Searchable word document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDocument {
    pub id: String,
    pub text: String,
    pub ipa: String,
    pub dialect: String,
    pub pos: Option<String>,
    pub difficulty: i32,
    pub language: String,
    pub audio_url: Option<String>,
    pub video_url: Option<String>,
    pub created_at: String,
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub hits: Vec<WordDocument>,
    pub total_hits: u64,
    pub processing_time_ms: u64,
    pub query: String,
}

/// Search filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub dialect: Option<String>,
    pub pos: Option<String>,
    pub difficulty_min: Option<i32>,
    pub difficulty_max: Option<i32>,
    pub language: Option<String>,
}

impl SearchService {
    /// Create a new search service
    pub async fn new(meilisearch_url: &str, meilisearch_key: &str) -> Result<Self> {
        let client = Client::new(meilisearch_url, Some(meilisearch_key));
        let vocab_index = client.index("vocabulary");
        
        let service = Self {
            client,
            vocab_index,
        };
        
        // Initialize the index
        service.initialize_index().await?;
        
        Ok(service)
    }
    
    /// Initialize the Meilisearch index with settings
    async fn initialize_index(&self) -> Result<()> {
        // Set up searchable attributes
        let searchable_attributes = vec![
            "text".to_string(),
            "ipa".to_string(),
            "pos".to_string(),
        ];
        
        self.vocab_index
            .set_searchable_attributes(&searchable_attributes)
            .await?;
        
        // Set up filterable attributes
        let filterable_attributes = vec![
            "dialect".to_string(),
            "pos".to_string(),
            "difficulty".to_string(),
            "language".to_string(),
        ];
        
        self.vocab_index
            .set_filterable_attributes(&filterable_attributes)
            .await?;
        
        // Set up sortable attributes
        let sortable_attributes = vec![
            "text".to_string(),
            "difficulty".to_string(),
            "created_at".to_string(),
        ];
        
        self.vocab_index
            .set_sortable_attributes(&sortable_attributes)
            .await?;
        
        // Configure typo tolerance (simplified)
        // Note: TypoTolerance struct may not be available in this version
        // self.vocab_index
        //     .set_typo_tolerance(&typo_tolerance)
        //     .await?;
        
        // Configure synonyms
        let synonyms = HashMap::from([
            ("hello".to_string(), vec!["hi".to_string(), "hey".to_string()]),
            ("pronunciation".to_string(), vec!["pronounce".to_string(), "speech".to_string()]),
        ]);
        
        self.vocab_index
            .set_synonyms(&synonyms)
            .await?;
        
        Ok(())
    }
    
    /// Search for words
    pub async fn search_words(
        &self,
        query: &str,
        filters: Option<SearchFilters>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<SearchResult> {
        // Build filter string first
        let filter_string = if let Some(filters) = filters {
            let mut filter_conditions = Vec::new();
            
            if let Some(dialect) = &filters.dialect {
                filter_conditions.push(format!("dialect = {}", dialect));
            }
            
            if let Some(pos) = &filters.pos {
                filter_conditions.push(format!("pos = {}", pos));
            }
            
            if let Some(difficulty_min) = filters.difficulty_min {
                filter_conditions.push(format!("difficulty >= {}", difficulty_min));
            }
            
            if let Some(difficulty_max) = filters.difficulty_max {
                filter_conditions.push(format!("difficulty <= {}", difficulty_max));
            }
            
            if let Some(language) = &filters.language {
                filter_conditions.push(format!("language = {}", language));
            }
            
            if !filter_conditions.is_empty() {
                Some(filter_conditions.join(" AND "))
            } else {
                None
            }
        } else {
            None
        };
        
        // Build search query
        let vocab_index = &self.vocab_index;
        let mut search_query = SearchQuery::new(vocab_index);
        search_query.with_query(query);
        search_query.with_limit(limit.unwrap_or(20));
        search_query.with_offset(offset.unwrap_or(0));
        
        if let Some(filter) = &filter_string {
            search_query.with_filter(filter);
        }
        
        // Execute search
        let search_response = search_query.execute().await?;
        
        // Convert results
        let hits: Vec<WordDocument> = search_response
            .hits
            .into_iter()
            .map(|hit| hit.result)
            .collect();
        
        Ok(SearchResult {
            hits,
            total_hits: search_response.estimated_total_hits.unwrap_or(0) as u64,
            processing_time_ms: search_response.processing_time_ms as u64,
            query: query.to_string(),
        })
    }
    
    /// Add or update a word document
    pub async fn index_word(&self, word: WordDocument) -> Result<()> {
        self.vocab_index
            .add_or_replace(&[word], Some("id"))
            .await?;
        
        Ok(())
    }
    
    /// Add multiple word documents
    pub async fn index_words(&self, words: Vec<WordDocument>) -> Result<()> {
        if words.is_empty() {
            return Ok(());
        }
        
        self.vocab_index
            .add_or_replace(&words, Some("id"))
            .await?;
        
        Ok(())
    }
    
    /// Delete a word document
    pub async fn delete_word(&self, word_id: Uuid) -> Result<()> {
        self.vocab_index
            .delete_document(&word_id.to_string())
            .await?;
        
        Ok(())
    }
    
    /// Get word by ID
    pub async fn get_word(&self, word_id: Uuid) -> Result<Option<WordDocument>> {
        let result = self.vocab_index
            .get_document(&word_id.to_string())
            .await?;
        
        Ok(Some(result))
    }
    
    /// Search with phonetic matching
    pub async fn search_phonetic(
        &self,
        ipa_query: &str,
        dialect: Option<&str>,
    ) -> Result<SearchResult> {
        let filter_string = if let Some(dialect) = dialect {
            Some(format!("dialect = {}", dialect))
        } else {
            None
        };
        
        let vocab_index = &self.vocab_index;
        let mut search_query = SearchQuery::new(vocab_index);
        search_query.with_query(ipa_query);
        search_query.with_limit(20);
        
        if let Some(filter) = &filter_string {
            search_query.with_filter(filter);
        }
        
        let search_response = search_query.execute().await?;
        
        let hits: Vec<WordDocument> = search_response
            .hits
            .into_iter()
            .map(|hit| hit.result)
            .collect();
        
        Ok(SearchResult {
            hits,
            total_hits: search_response.estimated_total_hits.unwrap_or(0) as u64,
            processing_time_ms: search_response.processing_time_ms as u64,
            query: ipa_query.to_string(),
        })
    }
    
    /// Get search suggestions
    pub async fn get_suggestions(&self, query: &str, limit: usize) -> Result<Vec<String>> {
        let search_result = self.search_words(query, None, Some(limit), None).await?;
        
        let suggestions = search_result
            .hits
            .into_iter()
            .map(|hit| hit.text)
            .collect();
        
        Ok(suggestions)
    }
    
    /// Get similar words
    pub async fn get_similar_words(
        &self,
        word_id: Uuid,
        limit: usize,
    ) -> Result<Vec<WordDocument>> {
        // Get the word first
        let word = match self.get_word(word_id).await? {
            Some(word) => word,
            None => return Ok(vec![]),
        };
        
        // Search for words with similar characteristics
        let filters = SearchFilters {
            dialect: Some(word.dialect),
            pos: word.pos,
            difficulty_min: Some((word.difficulty - 1).max(1)),
            difficulty_max: Some((word.difficulty + 1).min(5)),
            language: Some(word.language),
        };
        
        let search_result = self
            .search_words(&word.text, Some(filters), Some(limit + 1), None)
            .await?;
        
        // Filter out the original word
        let similar_words = search_result
            .hits
            .into_iter()
            .filter(|hit| hit.id != word.id)
            .take(limit)
            .collect();
        
        Ok(similar_words)
    }
    
    /// Get index statistics
    pub async fn get_stats(&self) -> Result<HashMap<String, serde_json::Value>> {
        let stats = self.vocab_index.get_stats().await?;
        
        let mut result = HashMap::new();
        result.insert("number_of_documents".to_string(), serde_json::json!(stats.number_of_documents));
        result.insert("is_indexing".to_string(), serde_json::json!(stats.is_indexing));
        result.insert("field_distribution".to_string(), serde_json::json!(stats.field_distribution));
        
        Ok(result)
    }
    
    /// Clear the entire index
    pub async fn clear_index(&self) -> Result<()> {
        self.vocab_index.delete_all_documents().await?;
        Ok(())
    }
}

/// Search service builder
pub struct SearchServiceBuilder {
    meilisearch_url: String,
    meilisearch_key: String,
}

impl SearchServiceBuilder {
    pub fn new(meilisearch_url: String, meilisearch_key: String) -> Self {
        Self {
            meilisearch_url,
            meilisearch_key,
        }
    }
    
    pub async fn build(self) -> Result<SearchService> {
        SearchService::new(&self.meilisearch_url, &self.meilisearch_key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_document_serialization() {
        let word = WordDocument {
            id: "test-id".to_string(),
            text: "hello".to_string(),
            ipa: "həˈloʊ".to_string(),
            dialect: "GA".to_string(),
            pos: Some("noun".to_string()),
            difficulty: 1,
            language: "en".to_string(),
            audio_url: Some("https://example.com/audio.wav".to_string()),
            video_url: None,
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        
        let json = serde_json::to_string(&word).unwrap();
        let deserialized: WordDocument = serde_json::from_str(&json).unwrap();
        
        assert_eq!(word.id, deserialized.id);
        assert_eq!(word.text, deserialized.text);
    }

    #[test]
    fn test_search_filters() {
        let filters = SearchFilters {
            dialect: Some("GA".to_string()),
            pos: Some("noun".to_string()),
            difficulty_min: Some(1),
            difficulty_max: Some(3),
            language: Some("en".to_string()),
        };
        
        // Test that filters can be serialized
        let json = serde_json::to_string(&filters).unwrap();
        let deserialized: SearchFilters = serde_json::from_str(&json).unwrap();
        
        assert_eq!(filters.dialect, deserialized.dialect);
        assert_eq!(filters.pos, deserialized.pos);
    }
}