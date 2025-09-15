import { VueQueryPlugin, QueryClient } from '@tanstack/vue-query';
import type { ApiError } from '../api/client';

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000, // 5 minutes
      gcTime: 10 * 60 * 1000, // 10 minutes (formerly cacheTime)
      retry: (failureCount, error: any) => {
        // Don't retry on 4xx errors
        if (error?.status >= 400 && error?.status < 500) {
          return false;
        }
        return failureCount < 3;
      },
      refetchOnWindowFocus: false, // Disable refetch on window focus
      refetchOnReconnect: true, // Refetch when reconnecting to internet
    },
    mutations: {
      retry: false,
      onError: (error: ApiError) => {
        // Global error handling for mutations
        console.error('Mutation error:', error);
      },
    },
  },
});

// Global error handler
queryClient.setMutationDefaults(['auth'], {
  onError: (error: ApiError) => {
    console.error('Auth mutation error:', error);
  },
});

queryClient.setMutationDefaults(['vocabulary'], {
  onError: (error: ApiError) => {
    console.error('Vocabulary mutation error:', error);
  },
});

queryClient.setMutationDefaults(['practice'], {
  onError: (error: ApiError) => {
    console.error('Practice mutation error:', error);
  },
});

export default VueQueryPlugin;
