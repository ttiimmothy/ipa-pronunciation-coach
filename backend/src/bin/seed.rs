use anyhow::Result;
use sqlx::Row;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
  // Load .env file if it exists
  let _ = dotenvy::dotenv();

  // Initialize tracing
  tracing_subscriber::fmt::init();

  // Get database URL from environment
  let database_url = env::var("DATABASE_URL").unwrap();
  // .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ipa_pronunciation_coach".to_string());

  // Create database pool
  let pool = sqlx::PgPool::connect(&database_url).await?;

  // Run migrations
  sqlx::migrate!("./migrations").run(&pool).await?;

  // Seed data
  seed_users(&pool).await?;
  seed_words(&pool).await?;
  seed_phonemes(&pool).await?;

  println!("✅ Database seeded successfully!");

  Ok(())
}

async fn seed_users(pool: &sqlx::PgPool) -> Result<()> {
  println!("🌱 Seeding users...");

  // Simple password hashing for demo
  let admin_password = "password123"; // In production, use proper hashing
  let demo_password = "password123";

  sqlx::query(
    "INSERT INTO users (email, pass_hash, name, dialect) VALUES ($1, $2, $3, $4) ON CONFLICT (email) DO NOTHING"
  )
  .bind("admin@example.com")
  .bind(admin_password)
  .bind("Admin User")
  .bind("GA")
  .execute(pool)
  .await?;

  sqlx::query(
    "INSERT INTO users (email, pass_hash, name, dialect) VALUES ($1, $2, $3, $4) ON CONFLICT (email) DO NOTHING"
  )
  .bind("demo@example.com")
  .bind(demo_password)
  .bind("Demo User")
  .bind("GA")
  .execute(pool)
  .await?;

  Ok(())
}

async fn seed_words(pool: &sqlx::PgPool) -> Result<()> {
  println!("🌱 Seeding words...");

  let words = vec![
    ("hello", "həˈloʊ", "noun", 1),
    ("world", "wɜːrld", "noun", 1),
    ("pronunciation", "prəˌnʌnsiˈeɪʃən", "noun", 3),
    ("practice", "ˈpræktɪs", "noun", 2),
    ("language", "ˈlæŋɡwɪdʒ", "noun", 2),
    ("difficult", "ˈdɪfɪkəlt", "adjective", 3),
    ("beautiful", "ˈbjuːtɪfəl", "adjective", 2),
    ("education", "ˌedʒʊˈkeɪʃən", "noun", 3),
    ("communication", "kəˌmjuːnɪˈkeɪʃən", "noun", 4),
    ("international", "ˌɪntərˈnæʃənəl", "adjective", 4),
  ];

  for (text, ipa, pos, difficulty) in words {
    // Insert word
    let result = sqlx::query(
          "INSERT INTO words (text, language, pos, difficulty) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING RETURNING id"
      )
      .bind(text)
      .bind("en")
      .bind(pos)
      .bind(difficulty)
      .fetch_optional(pool)
      .await?;

    if let Some(row) = result {
      let word_id: uuid::Uuid = row.get("id");

      // Insert GA dialect variant
      sqlx::query(
              "INSERT INTO dialect_variants (word_id, dialect, ipa) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
          )
          .bind(word_id)
          .bind("GA")
          .bind(ipa)
          .execute(pool)
          .await?;
    }
  }

  Ok(())
}

async fn seed_phonemes(pool: &sqlx::PgPool) -> Result<()> {
  println!("🌱 Seeding phonemes...");

  let phonemes = vec![
    ("p", "voiceless bilabial plosive"),
    ("b", "voiced bilabial plosive"),
    ("t", "voiceless alveolar plosive"),
    ("d", "voiced alveolar plosive"),
    ("k", "voiceless velar plosive"),
    ("g", "voiced velar plosive"),
    ("f", "voiceless labiodental fricative"),
    ("v", "voiced labiodental fricative"),
    ("θ", "voiceless dental fricative"),
    ("ð", "voiced dental fricative"),
    ("s", "voiceless alveolar fricative"),
    ("z", "voiced alveolar fricative"),
    ("ʃ", "voiceless postalveolar fricative"),
    ("ʒ", "voiced postalveolar fricative"),
    ("h", "voiceless glottal fricative"),
    ("m", "bilabial nasal"),
    ("n", "alveolar nasal"),
    ("ŋ", "velar nasal"),
    ("l", "alveolar lateral approximant"),
    ("r", "alveolar approximant"),
    ("w", "labial-velar approximant"),
    ("j", "palatal approximant"),
    ("i", "close front unrounded vowel"),
    ("ɪ", "near-close near-front unrounded vowel"),
    ("e", "close-mid front unrounded vowel"),
    ("ɛ", "open-mid front unrounded vowel"),
    ("æ", "near-open front unrounded vowel"),
    ("ɑ", "open back unrounded vowel"),
    ("ɔ", "open-mid back rounded vowel"),
    ("o", "close-mid back rounded vowel"),
    ("ʊ", "near-close near-back rounded vowel"),
    ("u", "close back rounded vowel"),
    ("ʌ", "open-mid back unrounded vowel"),
    ("ə", "mid central vowel"),
  ];

  for (symbol, description) in phonemes {
    sqlx::query(
      "INSERT INTO phonemes (symbol, description) VALUES ($1, $2) ON CONFLICT (symbol) DO NOTHING",
    )
    .bind(symbol)
    .bind(description)
    .execute(pool)
    .await?;
  }

  Ok(())
}
