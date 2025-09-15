-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- Create custom types
CREATE TYPE dialect AS ENUM ('GA', 'RP', 'AU', 'CA', 'NZ', 'SA', 'IN', 'IE', 'SC', 'WA');
CREATE TYPE part_of_speech AS ENUM ('noun', 'verb', 'adjective', 'adverb', 'preposition', 'conjunction', 'interjection', 'pronoun', 'determiner', 'article');

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    pass_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    avatar_url TEXT,
    dialect dialect NOT NULL DEFAULT 'GA',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Words table
CREATE TABLE words (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    text VARCHAR(255) NOT NULL,
    language VARCHAR(10) NOT NULL DEFAULT 'en',
    pos part_of_speech,
    difficulty INTEGER NOT NULL DEFAULT 1 CHECK (difficulty >= 1 AND difficulty <= 5),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Dialect variants table
CREATE TABLE dialect_variants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    word_id UUID NOT NULL REFERENCES words(id) ON DELETE CASCADE,
    dialect dialect NOT NULL,
    ipa TEXT NOT NULL,
    audio_url TEXT,
    video_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(word_id, dialect)
);

-- Phonemes table
CREATE TABLE phonemes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    symbol VARCHAR(10) NOT NULL UNIQUE,
    description TEXT
);

-- Word phonemes mapping table
CREATE TABLE word_phonemes (
    word_id UUID NOT NULL REFERENCES words(id) ON DELETE CASCADE,
    phoneme_id UUID NOT NULL REFERENCES phonemes(id) ON DELETE CASCADE,
    order_index INTEGER NOT NULL,
    PRIMARY KEY (word_id, phoneme_id, order_index)
);

-- Minimal pairs table
CREATE TABLE minimal_pairs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    word_a_id UUID NOT NULL REFERENCES words(id) ON DELETE CASCADE,
    word_b_id UUID NOT NULL REFERENCES words(id) ON DELETE CASCADE,
    phoneme_diff TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Practice sessions table
CREATE TABLE practice_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    started_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    ended_at TIMESTAMP WITH TIME ZONE,
    total_ms INTEGER DEFAULT 0
);

-- Recordings table
CREATE TABLE recordings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_id UUID REFERENCES practice_sessions(id) ON DELETE SET NULL,
    word_id UUID NOT NULL REFERENCES words(id) ON DELETE CASCADE,
    dialect dialect NOT NULL,
    media_url TEXT NOT NULL,
    duration_ms INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Scores table
CREATE TABLE scores (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    recording_id UUID NOT NULL REFERENCES recordings(id) ON DELETE CASCADE,
    overall_pct NUMERIC(5,2) NOT NULL CHECK (overall_pct >= 0 AND overall_pct <= 100),
    per_phoneme JSONB NOT NULL,
    latency_ms INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(recording_id)
);

-- Daily usage table
CREATE TABLE daily_usage (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    active_ms INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, date)
);

-- Live rooms table
CREATE TABLE live_rooms (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    host_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    closed_at TIMESTAMP WITH TIME ZONE
);

-- Live members table
CREATE TABLE live_members (
    room_id UUID NOT NULL REFERENCES live_rooms(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    left_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (room_id, user_id)
);

-- Invites table
CREATE TABLE invites (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL,
    issuer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token VARCHAR(255) NOT NULL UNIQUE,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    accepted_at TIMESTAMP WITH TIME ZONE
);

-- Search sync cursor table
CREATE TABLE search_sync_cursor (
    name VARCHAR(255) PRIMARY KEY,
    last_seq BIGINT NOT NULL DEFAULT 0
);

-- Create indexes
CREATE INDEX idx_word_text_trgm ON words USING gin (text gin_trgm_ops);
CREATE INDEX idx_score_pct ON scores (overall_pct);
CREATE INDEX idx_daily_usage_user_date ON daily_usage (user_id, date);
CREATE INDEX idx_recordings_user_id ON recordings (user_id);
CREATE INDEX idx_recordings_word_id ON recordings (word_id);
CREATE INDEX idx_dialect_variants_word_id ON dialect_variants (word_id);
CREATE INDEX idx_dialect_variants_dialect ON dialect_variants (dialect);
CREATE INDEX idx_minimal_pairs_word_a ON minimal_pairs (word_a_id);
CREATE INDEX idx_minimal_pairs_word_b ON minimal_pairs (word_b_id);
