import apiClient, { type ApiError } from './client';

export interface PracticeSession {
  id: string;
  word_id: string;
  user_id: string;
  audio_url?: string;
  score?: number;
  feedback?: PracticeFeedback;
  created_at: string;
  updated_at: string;
}

export interface PracticeFeedback {
  overall_score: number;
  phoneme_scores: Record<string, number>;
  suggestions: string[];
  accuracy: number;
  fluency: number;
  pronunciation: number;
  word_accuracy: number;
  rhythm_score: number;
  stress_score: number;
}

export interface PracticeStats {
  total_sessions: number;
  average_score: number;
  words_practiced: number;
  current_streak: number;
  daily_goal_progress: number;
  weekly_progress: Array<{
    date: string;
    practice_time: number;
    sessions: number;
  }>;
  monthly_progress: Array<{
    month: string;
    practice_time: number;
    sessions: number;
    average_score: number;
  }>;
  difficulty_breakdown: Record<string, number>;
  category_breakdown: Record<string, number>;
}

export interface PracticeGoal {
  id: string;
  user_id: string;
  type: 'daily_time' | 'daily_sessions' | 'weekly_words' | 'monthly_score';
  target_value: number;
  current_value: number;
  start_date: string;
  end_date: string;
  is_active: boolean;
}

export const practiceApi = {
  async startSession(wordId: string): Promise<PracticeSession> {
    try {
      const response = await apiClient.post<PracticeSession>('/practice/sessions', { word_id: wordId });
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async submitRecording(sessionId: string, audioBlob: Blob): Promise<PracticeFeedback> {
    try {
      const formData = new FormData();
      formData.append('audio', audioBlob);
      
      const response = await apiClient.post<PracticeFeedback>(`/practice/sessions/${sessionId}/submit`, formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
      });
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async getSession(sessionId: string): Promise<PracticeSession> {
    try {
      const response = await apiClient.get<PracticeSession>(`/practice/sessions/${sessionId}`);
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async getStats(): Promise<PracticeStats> {
    try {
      const response = await apiClient.get<PracticeStats>('/practice/stats');
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async getRecentSessions(limit = 10): Promise<PracticeSession[]> {
    try {
      const response = await apiClient.get<PracticeSession[]>('/practice/sessions/recent', {
        params: { limit },
      });
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async getSessionsByWord(wordId: string, limit = 10): Promise<PracticeSession[]> {
    try {
      const response = await apiClient.get<PracticeSession[]>(`/practice/sessions/word/${wordId}`, {
        params: { limit },
      });
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async deleteSession(sessionId: string): Promise<void> {
    try {
      await apiClient.delete(`/practice/sessions/${sessionId}`);
    } catch (error) {
      throw error as ApiError;
    }
  },

  async getGoals(): Promise<PracticeGoal[]> {
    try {
      const response = await apiClient.get<PracticeGoal[]>('/practice/goals');
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async createGoal(goal: Omit<PracticeGoal, 'id' | 'user_id' | 'current_value'>): Promise<PracticeGoal> {
    try {
      const response = await apiClient.post<PracticeGoal>('/practice/goals', goal);
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async updateGoal(goalId: string, updates: Partial<PracticeGoal>): Promise<PracticeGoal> {
    try {
      const response = await apiClient.put<PracticeGoal>(`/practice/goals/${goalId}`, updates);
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async deleteGoal(goalId: string): Promise<void> {
    try {
      await apiClient.delete(`/practice/goals/${goalId}`);
    } catch (error) {
      throw error as ApiError;
    }
  },
};
