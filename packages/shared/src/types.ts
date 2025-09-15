export type Dialect = 'GA' | 'RP' | 'AU' | 'CA' | 'NZ' | 'SA' | 'IN' | 'IE' | 'SC' | 'WA';

export type PartOfSpeech = 
  | 'noun' 
  | 'verb' 
  | 'adjective' 
  | 'adverb' 
  | 'preposition' 
  | 'conjunction' 
  | 'interjection' 
  | 'pronoun' 
  | 'determiner' 
  | 'article';

export interface User {
  id: string;
  email: string;
  name: string;
  avatar_url?: string;
  dialect: Dialect;
  created_at: string;
}

export interface Word {
  id: string;
  text: string;
  language: string;
  pos?: PartOfSpeech;
  difficulty: number;
  created_at: string;
}

export interface DialectVariant {
  id: string;
  word_id: string;
  dialect: Dialect;
  ipa: string;
  audio_url?: string;
  video_url?: string;
  created_at: string;
}

export interface Phoneme {
  id: string;
  symbol: string;
  description?: string;
}

export interface WordWithVariants {
  id: string;
  text: string;
  language: string;
  pos?: PartOfSpeech;
  difficulty: number;
  created_at: string;
  variants: DialectVariant[];
  phonemes: Phoneme[];
  minimal_pairs: MinimalPair[];
}

export interface MinimalPair {
  id: string;
  word_a_id: string;
  word_b_id: string;
  phoneme_diff: string;
  created_at: string;
}

export interface PracticeSession {
  id: string;
  user_id: string;
  started_at: string;
  ended_at?: string;
  total_ms: number;
}

export interface Recording {
  id: string;
  user_id: string;
  session_id?: string;
  word_id: string;
  dialect: Dialect;
  media_url: string;
  duration_ms: number;
  created_at: string;
}

export interface Score {
  id: string;
  recording_id: string;
  overall_pct: number;
  per_phoneme: Record<string, number>;
  latency_ms: number;
  created_at: string;
}

export interface DailyUsage {
  user_id: string;
  date: string;
  active_ms: number;
}

export interface LiveRoom {
  id: string;
  host_id: string;
  created_at: string;
  closed_at?: string;
}

export interface LiveMember {
  room_id: string;
  user_id: string;
  joined_at: string;
  left_at?: string;
}

export interface Invite {
  id: string;
  email: string;
  issuer_id: string;
  token: string;
  expires_at: string;
  accepted_at?: string;
}

// API Request/Response types
export interface AuthResponse {
  access_token: string;
  user: User;
}

export interface CreateUser {
  email: string;
  password: string;
  name: string;
  dialect?: Dialect;
}

export interface LoginUser {
  email: string;
  password: string;
}

export interface VocabSearchQuery {
  query?: string;
  dialect?: Dialect;
  page?: number;
  limit?: number;
}

export interface CreateWord {
  text: string;
  language: string;
  pos?: PartOfSpeech;
  difficulty: number;
  variants: CreateDialectVariant[];
}

export interface CreateDialectVariant {
  dialect: Dialect;
  ipa: string;
  audio_url?: string;
  video_url?: string;
}

export interface ScoreRequest {
  recording_id: string;
  word_id: string;
  dialect: Dialect;
}

export interface ScoreResponse {
  status: 'pending' | 'processing' | 'completed' | 'failed';
  overall_pct?: number;
  per_phoneme?: Record<string, number>;
  job_id?: string;
}

export interface UsageTick {
  active_ms: number;
}

export interface TimeCapSettings {
  minutes: number;
}

export interface CreateRoom {
  name?: string;
}

export interface RoomResponse {
  room_id: string;
  ws_url: string;
}

export interface InviteRequest {
  email: string;
}

// WebSocket message types
export interface WebSocketMessage {
  event: string;
  data: any;
}

export interface ScoreCompletedEvent {
  recording_id: string;
  overall_pct: number;
  per_phoneme: Record<string, number>;
}

export interface RoomUserJoinedEvent {
  room_id: string;
  user_id: string;
  user_name: string;
}

export interface RoomUserLeftEvent {
  room_id: string;
  user_id: string;
}

export interface PushToTalkEvent {
  room_id: string;
  audio_data: number[];
}
