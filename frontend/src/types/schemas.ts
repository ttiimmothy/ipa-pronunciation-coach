import { z } from 'zod';

export const DialectSchema = z.enum(['GA', 'RP', 'AU', 'CA', 'NZ', 'SA', 'IN', 'IE', 'SC', 'WA']);

export const PartOfSpeechSchema = z.enum([
  'noun',
  'verb', 
  'adjective',
  'adverb',
  'preposition',
  'conjunction',
  'interjection',
  'pronoun',
  'determiner',
  'article'
]);

export const UserSchema = z.object({
  id: z.string().uuid(),
  email: z.string().email(),
  name: z.string().min(1),
  avatar_url: z.string().url().optional(),
  dialect: DialectSchema,
  created_at: z.string().datetime(),
});

export const WordSchema = z.object({
  id: z.string().uuid(),
  text: z.string().min(1),
  language: z.string().min(2).max(10),
  pos: PartOfSpeechSchema.optional(),
  difficulty: z.number().int().min(1).max(5),
  created_at: z.string().datetime(),
});

export const DialectVariantSchema = z.object({
  id: z.string().uuid(),
  word_id: z.string().uuid(),
  dialect: DialectSchema,
  ipa: z.string().min(1),
  audio_url: z.string().url().optional(),
  video_url: z.string().url().optional(),
  created_at: z.string().datetime(),
});

export const PhonemeSchema = z.object({
  id: z.string().uuid(),
  symbol: z.string().min(1).max(10),
  description: z.string().optional(),
});

export const MinimalPairSchema = z.object({
  id: z.string().uuid(),
  word_a_id: z.string().uuid(),
  word_b_id: z.string().uuid(),
  phoneme_diff: z.string().min(1),
  created_at: z.string().datetime(),
});

export const WordWithVariantsSchema = WordSchema.extend({
  variants: z.array(DialectVariantSchema),
  phonemes: z.array(PhonemeSchema),
  minimal_pairs: z.array(MinimalPairSchema),
});

export const PracticeSessionSchema = z.object({
  id: z.string().uuid(),
  user_id: z.string().uuid(),
  started_at: z.string().datetime(),
  ended_at: z.string().datetime().optional(),
  total_ms: z.number().int().min(0),
});

export const RecordingSchema = z.object({
  id: z.string().uuid(),
  user_id: z.string().uuid(),
  session_id: z.string().uuid().optional(),
  word_id: z.string().uuid(),
  dialect: DialectSchema,
  media_url: z.string().url(),
  duration_ms: z.number().int().min(0),
  created_at: z.string().datetime(),
});

export const ScoreSchema = z.object({
  id: z.string().uuid(),
  recording_id: z.string().uuid(),
  overall_pct: z.number().min(0).max(100),
  per_phoneme: z.record(z.string(), z.number().min(0).max(100)),
  latency_ms: z.number().int().min(0),
  created_at: z.string().datetime(),
});

export const DailyUsageSchema = z.object({
  user_id: z.string().uuid(),
  date: z.string().date(),
  active_ms: z.number().int().min(0),
});

export const LiveRoomSchema = z.object({
  id: z.string().uuid(),
  host_id: z.string().uuid(),
  created_at: z.string().datetime(),
  closed_at: z.string().datetime().optional(),
});

export const LiveMemberSchema = z.object({
  room_id: z.string().uuid(),
  user_id: z.string().uuid(),
  joined_at: z.string().datetime(),
  left_at: z.string().datetime().optional(),
});

export const InviteSchema = z.object({
  id: z.string().uuid(),
  email: z.string().email(),
  issuer_id: z.string().uuid(),
  token: z.string().min(1),
  expires_at: z.string().datetime(),
  accepted_at: z.string().datetime().optional(),
});

// API Request/Response schemas
export const AuthResponseSchema = z.object({
  access_token: z.string().min(1),
  user: UserSchema,
});

export const CreateUserSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
  name: z.string().min(1),
  dialect: DialectSchema.optional(),
});

export const LoginUserSchema = z.object({
  email: z.string().email(),
  password: z.string().min(1),
});

export const VocabSearchQuerySchema = z.object({
  query: z.string().optional(),
  dialect: DialectSchema.optional(),
  page: z.number().int().min(1).optional(),
  limit: z.number().int().min(1).max(100).optional(),
});

export const CreateDialectVariantSchema = z.object({
  dialect: DialectSchema,
  ipa: z.string().min(1),
  audio_url: z.string().url().optional(),
  video_url: z.string().url().optional(),
});

export const CreateWordSchema = z.object({
  text: z.string().min(1),
  language: z.string().min(2).max(10),
  pos: PartOfSpeechSchema.optional(),
  difficulty: z.number().int().min(1).max(5),
  variants: z.array(CreateDialectVariantSchema),
});

export const ScoreRequestSchema = z.object({
  recording_id: z.string().uuid(),
  word_id: z.string().uuid(),
  dialect: DialectSchema,
});

export const ScoreResponseSchema = z.object({
  status: z.enum(['pending', 'processing', 'completed', 'failed']),
  overall_pct: z.number().min(0).max(100).optional(),
  per_phoneme: z.record(z.string(), z.number().min(0).max(100)).optional(),
  job_id: z.string().optional(),
});

export const UsageTickSchema = z.object({
  active_ms: z.number().int().min(0),
});

export const TimeCapSettingsSchema = z.object({
  minutes: z.number().int().min(1).max(1440), // Max 24 hours
});

export const CreateRoomSchema = z.object({
  name: z.string().optional(),
});

export const RoomResponseSchema = z.object({
  room_id: z.string().uuid(),
  ws_url: z.string().url(),
});

export const InviteRequestSchema = z.object({
  email: z.string().email(),
});

// WebSocket message schemas
export const WebSocketMessageSchema = z.object({
  event: z.string().min(1),
  data: z.any(),
});

export const ScoreCompletedEventSchema = z.object({
  recording_id: z.string().uuid(),
  overall_pct: z.number().min(0).max(100),
  per_phoneme: z.record(z.string(), z.number().min(0).max(100)),
});

export const RoomUserJoinedEventSchema = z.object({
  room_id: z.string().uuid(),
  user_id: z.string().uuid(),
  user_name: z.string().min(1),
});

export const RoomUserLeftEventSchema = z.object({
  room_id: z.string().uuid(),
  user_id: z.string().uuid(),
});

export const PushToTalkEventSchema = z.object({
  room_id: z.string().uuid(),
  audio_data: z.array(z.number()),
});
