# IPA Pronunciation Coach 🎯

A comprehensive pronunciation learning platform that uses AI-powered feedback to help users master English pronunciation through IPA (International Phonetic Alphabet) analysis.

## ✨ Features

- 🗣️ **Vocabulary & IPA Browser** - Search and browse words with IPA transcriptions, audio, and video examples
- 🎵 **Pronunciation Practice** - Record and get instant feedback on pronunciation accuracy
- 📊 **Progress Tracking** - Daily logs, streaks, and detailed analytics
- ⏰ **Time Management** - Configurable daily practice limits with smart blocking
- 🎮 **Interactive Modes** - Call-and-response, minimal pairs, tongue twisters, and quizzes
- 👥 **Live Sessions** - Real-time practice with friends or tutors via WebSocket
- 🌍 **Multi-Dialect Support** - GA, RP, AU, CA, and more pronunciation variants
- ♿ **Accessibility** - Keyboard-first controls, screen reader support, dark mode
- 📱 **PWA Ready** - Offline support and mobile-optimized interface

## 🛠️ Tech Stack

### Backend
- **Language**: Rust
- **Framework**: Axum (async web framework)
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT with Argon2 password hashing
- **Real-time**: WebSocket support
- **Storage**: S3-compatible (MinIO for dev)
- **Search**: Meilisearch
- **Queue**: Redis with background job processing
- **Audio Processing**: Custom MFCC/DTW implementation

### Frontend
- **Framework**: Astro + Vue 3 (Composition API)
- **Language**: TypeScript
- **Styling**: TailwindCSS
- **State Management**: Pinia
- **Data Fetching**: Axios + TanStack Query
- **Charts**: Recharts
- **Real-time**: Socket.io-client

### Infrastructure
- **Containerization**: Docker & Docker Compose
- **Database**: PostgreSQL 15
- **Cache**: Redis 7
- **Search**: Meilisearch
- **Storage**: MinIO (S3-compatible)
- **CI/CD**: GitHub Actions

## 🚀 Prerequisites

- **Node.js** 18+ and npm
- **Rust** 1.80+ and Cargo
- **Docker** and Docker Compose
- **Git**

## 📦 Setup Instructions

### 1. Clone the Repository

```bash
git clone https://github.com:ttiimmothy/ipa-pronunciation-coach.git
cd ipa-pronunciation-coach
```

### 2. Start Development Environment

```bash
# Start all services (PostgreSQL, Redis, MinIO, Meilisearch)
cd backend
docker-compose up -d postgres redis minio meilisearch

# Wait for services to be ready (about 30 seconds)
```

### 3. Backend Setup

```bash
cd backend

# Install Rust dependencies
cargo build

# Copy environment variables
cp .env.example .env

# Database migrations run automatically when the backend starts

# Start the backend server
cargo run
```

The backend will be available at `http://localhost:8787`

### 4. Frontend Setup

```bash
cd frontend

# Install dependencies
npm install

# Copy environment variables
cp .env.example .env

# Start the development server
npm run start
```

The frontend will be available at `http://localhost:4320`

### 5. Shared Package Setup

```bash
cd packages/shared

# Install dependencies
npm install

# Build the package
npm run build
```

## 🎯 Demo Credentials

The application comes with seeded demo data:

- **Admin User**: `admin@example.com` / `password123`
- **Demo User**: `demo@example.com` / `password123`

## 📁 Project Structure

```
ipa_pronunciation_coach/
├── frontend/                 # Astro + Vue 3 frontend
│   ├── src/
│   │   ├── components/      # Vue components
│   │   ├── pages/          # Astro pages
│   │   ├── stores/         # Pinia stores
│   │   ├── types/          # TypeScript types
│   │   │   └── shared/     # Shared types for frontend
│   │   └── utils/          # Utility functions
│   ├── astro.config.mjs
│   └── package.json
├── backend/                 # Rust Axum backend
│   ├── src/
│   │   ├── routes/         # API route handlers
│   │   ├── services/       # Business logic
│   │   ├── models/         # Data models
│   │   ├── jobs/           # Background workers
│   │   └── main.rs
│   ├── migrations/         # Database migrations
│   ├── docker-compose.yml  # Development environment
│   ├── Cargo.toml
│   └── Dockerfile
├── packages/
│   └── shared/             # Shared TypeScript types
│       ├── src/
│       │   ├── types.ts    # Type definitions
│       │   └── schemas.ts  # Zod validation schemas
│       └── package.json
└── README.md
```

## 🔌 API Endpoints

### Authentication
- `POST /auth/register` - User registration
- `POST /auth/login` - User login
- `GET /me` - Get current user

### Vocabulary
- `GET /vocab` - Search vocabulary
- `GET /vocab/:id` - Get word details
- `POST /vocab` - Create word (admin)

### Practice & Scoring
- `POST /media/recordings` - Upload audio recording
- `POST /practice/score` - Submit for scoring
- `GET /practice/score/:recordingId` - Get score results

### Logs & Usage
- `GET /logs/daily` - Get daily progress
- `POST /usage/tick` - Update usage time
- `PUT /settings/timecap` - Update time limits

### Live Sessions
- `POST /live` - Create live room
- `POST /live/:roomId/invite` - Invite to room
- `WS /ws` - WebSocket connection

## 🧪 Development

### Running Tests

```bash
# Backend tests
cd backend
cargo test

# Frontend tests
cd frontend
npm run test

# E2E tests
npm run test:e2e
```

### Linting & Formatting

```bash
# Backend
cd backend
cargo fmt
cargo clippy

# Frontend
cd frontend
npm run lint
npm run format
```

### Database Management

```bash
# Create new migration
cd backend
# Add new SQL file to migrations/ folder with timestamp prefix
# Example: migrations/002_add_new_table.sql

# Run migrations (happens automatically when backend starts)
cargo run

# Reset database
cd backend
docker-compose down -v
docker-compose up -d postgres
# Migrations will run automatically when you start the backend
```

## 🐳 Docker Deployment

### Development

```bash
# Start all services
cd backend
docker-compose up

# Start specific services
docker-compose up postgres redis minio meilisearch

# View logs
docker-compose logs -f backend
```

### Production

```bash
# Build and start production containers
cd backend
docker-compose -f docker-compose.prod.yml up -d

# Scale services
docker-compose up -d --scale backend=3
```

## 🔧 Environment Variables

### Backend (.env)
```env
DATABASE_URL=postgres://postgres:postgres@postgres:5432/ipa_pronunciation_coach
JWT_SECRET=your_jwt_secret_here
S3_ENDPOINT=http://minio:9000
S3_BUCKET=ipa-media
S3_ACCESS_KEY=minioadmin
S3_SECRET_KEY=minioadmin
S3_REGION=us-east-1
REDIS_URL=redis://localhost:6379/0
MEILISEARCH_URL=http://meilisearch:7700
MEILISEARCH_KEY=masterKey
CLIENT_URL=http://localhost:4320
ALLOW_DEV_GOOGLE_SSO=false
```

### Frontend (.env)
```env
PUBLIC_API_URL=http://localhost:8787
PUBLIC_WS_URL=ws://localhost:8787
PUBLIC_MEDIA_BASE=http://localhost:9000
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow conventional commits
- Write tests for new features
- Update documentation
- Ensure all tests pass
- Follow the existing code style

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Astro](https://astro.build/) for the amazing static site generator
- [Vue.js](https://vuejs.org/) for the reactive framework
- [Axum](https://github.com/tokio-rs/axum) for the Rust web framework
- [TailwindCSS](https://tailwindcss.com/) for the utility-first CSS framework
- [Meilisearch](https://www.meilisearch.com/) for the fast search engine
- [PostgreSQL](https://www.postgresql.org/) for the reliable database

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/your-org/ipa-pronunciation-coach/issues) page
2. Create a new issue with detailed information
3. Join our [Discord community](https://discord.gg/your-discord)

---

**Happy Learning! 🎉**
