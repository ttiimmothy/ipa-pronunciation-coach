import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query';
import { practiceApi, type PracticeStats, type PracticeSession, type PracticeGoal, type PracticeFeedback } from '../api/practice';
import type { ApiError } from '../api/client';

export const usePractice = () => {
  const queryClient = useQueryClient();

  // Get practice stats
  const { 
    data: stats, 
    isLoading: isLoadingStats,
    error: statsError,
    refetch: refetchStats 
  } = useQuery({
    queryKey: ['practice', 'stats'],
    queryFn: practiceApi.getStats,
    staleTime: 1 * 60 * 1000, // 1 minute
  });

  // Get recent sessions
  const { 
    data: recentSessions, 
    isLoading: isLoadingSessions,
    error: sessionsError,
    refetch: refetchSessions 
  } = useQuery({
    queryKey: ['practice', 'sessions', 'recent'],
    queryFn: () => practiceApi.getRecentSessions(10),
    staleTime: 30 * 1000, // 30 seconds
  });

  // Get practice goals
  const { 
    data: goals, 
    isLoading: isLoadingGoals,
    error: goalsError,
    refetch: refetchGoals 
  } = useQuery({
    queryKey: ['practice', 'goals'],
    queryFn: practiceApi.getGoals,
    staleTime: 5 * 60 * 1000, // 5 minutes
  });

  // Start session mutation
  const startSessionMutation = useMutation({
    mutationFn: practiceApi.startSession,
    onSuccess: (session) => {
      // Invalidate stats and sessions
      queryClient.invalidateQueries({ queryKey: ['practice', 'stats'] });
      queryClient.invalidateQueries({ queryKey: ['practice', 'sessions'] });
      // Add the new session to recent sessions cache
      queryClient.setQueryData(['practice', 'sessions', 'recent'], (oldData: PracticeSession[]) => {
        return [session, ...(oldData || []).slice(0, 9)];
      });
    },
    onError: (error: ApiError) => {
      console.error('Failed to start practice session:', error);
    },
  });

  // Submit recording mutation
  const submitRecordingMutation = useMutation({
    mutationFn: ({ sessionId, audioBlob }: { sessionId: string; audioBlob: Blob }) =>
      practiceApi.submitRecording(sessionId, audioBlob),
    onSuccess: (feedback, { sessionId }) => {
      // Invalidate stats and sessions
      queryClient.invalidateQueries({ queryKey: ['practice', 'stats'] });
      queryClient.invalidateQueries({ queryKey: ['practice', 'sessions'] });
      // Update the specific session with feedback
      queryClient.setQueryData(['practice', 'sessions', 'recent'], (oldData: PracticeSession[]) => {
        if (!oldData) return oldData;
        return oldData.map(session => 
          session.id === sessionId 
            ? { ...session, feedback, score: feedback.overall_score }
            : session
        );
      });
    },
    onError: (error: ApiError) => {
      console.error('Failed to submit recording:', error);
    },
  });

  // Delete session mutation
  const deleteSessionMutation = useMutation({
    mutationFn: practiceApi.deleteSession,
    onSuccess: (_, sessionId) => {
      // Remove from recent sessions cache
      queryClient.setQueryData(['practice', 'sessions', 'recent'], (oldData: PracticeSession[]) => {
        if (!oldData) return oldData;
        return oldData.filter(session => session.id !== sessionId);
      });
      // Invalidate stats
      queryClient.invalidateQueries({ queryKey: ['practice', 'stats'] });
    },
    onError: (error: ApiError) => {
      console.error('Failed to delete session:', error);
    },
  });

  // Create goal mutation
  const createGoalMutation = useMutation({
    mutationFn: practiceApi.createGoal,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['practice', 'goals'] });
    },
    onError: (error: ApiError) => {
      console.error('Failed to create goal:', error);
    },
  });

  // Update goal mutation
  const updateGoalMutation = useMutation({
    mutationFn: ({ goalId, updates }: { goalId: string; updates: Partial<PracticeGoal> }) =>
      practiceApi.updateGoal(goalId, updates),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['practice', 'goals'] });
    },
    onError: (error: ApiError) => {
      console.error('Failed to update goal:', error);
    },
  });

  // Delete goal mutation
  const deleteGoalMutation = useMutation({
    mutationFn: practiceApi.deleteGoal,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['practice', 'goals'] });
    },
    onError: (error: ApiError) => {
      console.error('Failed to delete goal:', error);
    },
  });

  const startSession = (wordId: string) => startSessionMutation.mutate(wordId);
  const submitRecording = (sessionId: string, audioBlob: Blob) =>
    submitRecordingMutation.mutate({ sessionId, audioBlob });
  const deleteSession = (sessionId: string) => deleteSessionMutation.mutate(sessionId);
  const createGoal = (goal: Omit<PracticeGoal, 'id' | 'user_id' | 'current_value'>) => 
    createGoalMutation.mutate(goal);
  const updateGoal = (goalId: string, updates: Partial<PracticeGoal>) => 
    updateGoalMutation.mutate({ goalId, updates });
  const deleteGoal = (goalId: string) => deleteGoalMutation.mutate(goalId);

  return {
    // Data
    stats: stats.value,
    recentSessions: recentSessions.value || [],
    goals: goals.value || [],
    
    // Loading states
    isLoadingStats,
    isLoadingSessions,
    isLoadingGoals,
    
    // Errors
    statsError,
    sessionsError,
    goalsError,
    
    // Actions
    startSession,
    submitRecording,
    deleteSession,
    createGoal,
    updateGoal,
    deleteGoal,
    refetchStats,
    refetchSessions,
    refetchGoals,
    
    // Mutation states
    isStartingSession: startSessionMutation.isPending.value,
    startSessionError: startSessionMutation.error.value,
    isSubmittingRecording: submitRecordingMutation.isPending.value,
    submitRecordingError: submitRecordingMutation.error.value,
    isDeletingSession: deleteSessionMutation.isPending.value,
    deleteSessionError: deleteSessionMutation.error.value,
    isCreatingGoal: createGoalMutation.isPending.value,
    createGoalError: createGoalMutation.error.value,
    isUpdatingGoal: updateGoalMutation.isPending.value,
    updateGoalError: updateGoalMutation.error.value,
    isDeletingGoal: deleteGoalMutation.isPending.value,
    deleteGoalError: deleteGoalMutation.error.value,
  };
};
