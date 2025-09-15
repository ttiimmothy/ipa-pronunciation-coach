import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query';
import { vocabularyApi, type VocabularySearchParams, type Word, type PracticeWord } from '../api/vocabulary';
import type { ApiError } from '../api/client';

export const useVocabulary = (params: VocabularySearchParams = {}) => {
  const queryClient = useQueryClient();

  // Get vocabulary words
  const { 
    data, 
    isLoading, 
    error, 
    refetch,
    isFetching 
  } = useQuery({
    queryKey: ['vocabulary', 'words', params],
    queryFn: () => vocabularyApi.getWords(params),
    staleTime: 2 * 60 * 1000, // 2 minutes
    enabled: true,
  });

  // Get single word
  const useWord = (id: string) => {
    return useQuery({
      queryKey: ['vocabulary', 'word', id],
      queryFn: () => vocabularyApi.getWord(id),
      enabled: !!id,
      staleTime: 5 * 60 * 1000, // 5 minutes
    });
  };

  // Get practice words
  const { 
    data: practiceWords, 
    isLoading: isLoadingPractice,
    error: practiceError,
    refetch: refetchPractice 
  } = useQuery({
    queryKey: ['vocabulary', 'practice'],
    queryFn: vocabularyApi.getPracticeWords,
    staleTime: 1 * 60 * 1000, // 1 minute
  });

  // Get categories
  const { data: categories, isLoading: isLoadingCategories } = useQuery({
    queryKey: ['vocabulary', 'categories'],
    queryFn: vocabularyApi.getCategories,
    staleTime: 10 * 60 * 1000, // 10 minutes
  });

  // Get tags
  const { data: tags, isLoading: isLoadingTags } = useQuery({
    queryKey: ['vocabulary', 'tags'],
    queryFn: vocabularyApi.getTags,
    staleTime: 10 * 60 * 1000, // 10 minutes
  });

  // Add to practice mutation
  const addToPracticeMutation = useMutation({
    mutationFn: vocabularyApi.addToPractice,
    onSuccess: (_, wordId) => {
      // Invalidate practice words query
      queryClient.invalidateQueries({ queryKey: ['vocabulary', 'practice'] });
      // Update the specific word in the words list
      queryClient.setQueryData(['vocabulary', 'words', params], (oldData: any) => {
        if (!oldData) return oldData;
        return {
          ...oldData,
          words: oldData.words.map((word: Word) => 
            word.id === wordId 
              ? { ...word, practice_count: word.practice_count + 1 }
              : word
          ),
        };
      });
    },
    onError: (error: ApiError) => {
      console.error('Failed to add word to practice:', error);
    },
  });

  // Remove from practice mutation
  const removeFromPracticeMutation = useMutation({
    mutationFn: vocabularyApi.removeFromPractice,
    onSuccess: (_, wordId) => {
      // Invalidate practice words query
      queryClient.invalidateQueries({ queryKey: ['vocabulary', 'practice'] });
      // Update the specific word in the words list
      queryClient.setQueryData(['vocabulary', 'words', params], (oldData: any) => {
        if (!oldData) return oldData;
        return {
          ...oldData,
          words: oldData.words.map((word: Word) => 
            word.id === wordId 
              ? { ...word, practice_count: Math.max(0, word.practice_count - 1) }
              : word
          ),
        };
      });
    },
    onError: (error: ApiError) => {
      console.error('Failed to remove word from practice:', error);
    },
  });

  // Search words mutation
  const searchMutation = useMutation({
    mutationFn: ({ query, params: searchParams }: { query: string; params?: Omit<VocabularySearchParams, 'query'> }) =>
      vocabularyApi.searchWords(query, searchParams),
    onError: (error: ApiError) => {
      console.error('Search failed:', error);
    },
  });

  const addToPractice = (wordId: string) => addToPracticeMutation.mutate(wordId);
  const removeFromPractice = (wordId: string) => removeFromPracticeMutation.mutate(wordId);
  const searchWords = (query: string, searchParams?: Omit<VocabularySearchParams, 'query'>) => 
    searchMutation.mutate({ query, params: searchParams });

  return {
    // Data
    words: data.value?.words || [],
    total: data.value?.total || 0,
    page: data.value?.page || 1,
    totalPages: data.value?.total_pages || 1,
    practiceWords: practiceWords.value || [],
    categories: categories.value || [],
    tags: tags.value || [],
    
    // Loading states
    isLoading,
    isLoadingPractice,
    isLoadingCategories,
    isLoadingTags,
    isFetching,
    
    // Errors
    error,
    practiceError,
    
    // Actions
    refetch,
    refetchPractice,
    addToPractice,
    removeFromPractice,
    searchWords,
    
    // Mutation states
    isAddingToPractice: addToPracticeMutation.isPending.value,
    addToPracticeError: addToPracticeMutation.error.value,
    isRemovingFromPractice: removeFromPracticeMutation.isPending.value,
    removeFromPracticeError: removeFromPracticeMutation.error.value,
    isSearching: searchMutation.isPending.value,
    searchError: searchMutation.error.value,
    searchResults: searchMutation.data.value,
    
    // Hooks
    useWord,
  };
};
