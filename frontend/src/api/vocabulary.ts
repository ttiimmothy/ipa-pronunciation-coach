import apiClient, { type ApiError } from './client';

export interface Word {
  id: string;
  word: string;
  ipa: string;
  definition: string;
  difficulty: 'beginner' | 'intermediate' | 'advanced';
  category: string;
  tags: string[];
  practice_count: number;
  created_at: string;
  updated_at: string;
}

export interface VocabularySearchParams {
  query?: string;
  difficulty?: string;
  category?: string;
  tags?: string[];
  page?: number;
  limit?: number;
  sort?: 'word' | 'difficulty' | 'practice_count' | 'created_at';
  order?: 'asc' | 'desc';
}

export interface VocabularyResponse {
  words: Word[];
  total: number;
  page: number;
  limit: number;
  total_pages: number;
}

export interface PracticeWord extends Word {
  last_practiced?: string;
  best_score?: number;
  attempts_count: number;
}

export const vocabularyApi = {
  async getWords(params: VocabularySearchParams = {}): Promise<VocabularyResponse> {
    try {
      const response = await apiClient.get<VocabularyResponse>('/vocabulary', { params });
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async getWord(id: string): Promise<Word> {
    try {
      const response = await apiClient.get<Word>(`/vocabulary/${id}`);
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async addToPractice(wordId: string): Promise<void> {
    try {
      await apiClient.post(`/vocabulary/${wordId}/practice`);
    } catch (error) {
      throw error as ApiError;
    }
  },

  async removeFromPractice(wordId: string): Promise<void> {
    try {
      await apiClient.delete(`/vocabulary/${wordId}/practice`);
    } catch (error) {
      throw error as ApiError;
    }
  },

  async getPracticeWords(): Promise<PracticeWord[]> {
    try {
      const response = await apiClient.get<PracticeWord[]>('/vocabulary/practice');
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async searchWords(query: string, params: Omit<VocabularySearchParams, 'query'> = {}): Promise<VocabularyResponse> {
    try {
      const response = await apiClient.get<VocabularyResponse>('/vocabulary/search', {
        params: { query, ...params }
      });
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async getCategories(): Promise<string[]> {
    try {
      const response = await apiClient.get<string[]>('/vocabulary/categories');
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async getTags(): Promise<string[]> {
    try {
      const response = await apiClient.get<string[]>('/vocabulary/tags');
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },
};
