-- Включаем поддержку внешних ключей
PRAGMA foreign_keys = ON;

--------------------------------------------------------------------------------
-- 1. ПОЛЬЗОВАТЕЛИ И ПРОГРЕСС
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    xp_balance INTEGER DEFAULT 0,       -- Валюта для трат
    total_xp_accumulated INTEGER DEFAULT 0, -- Общий опыт (для уровня)
    level INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    daily_quests_streak INTEGER DEFAULT 0,
);

--------------------------------------------------------------------------------
-- 2. СЛОЖНОЕ ДЕРЕВО НАВЫКОВ
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    max_level INTEGER DEFAULT 5,
    base_cost INTEGER DEFAULT 100 -- Стоимость 1-го уровня
);

-- Таблица требований (пререквизитов)
CREATE TABLE IF NOT EXISTS skill_prerequisites (
    skill_id INTEGER,
    required_skill_id INTEGER,
    required_level INTEGER DEFAULT 1,
    PRIMARY KEY (skill_id, required_skill_id),
    FOREIGN KEY (skill_id) REFERENCES skills(id),
    FOREIGN KEY (required_skill_id) REFERENCES skills(id)
);

-- Прогресс навыков пользователя
CREATE TABLE IF NOT EXISTS user_skills (
    user_id INTEGER,
    skill_id INTEGER,
    current_level INTEGER DEFAULT 0,
    PRIMARY KEY (user_id, skill_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (skill_id) REFERENCES skills(id)
);

--------------------------------------------------------------------------------
-- 3. СОЦИАЛЬНАЯ ИГРОВАЯ МЕХАНИКА (ЛОББИ И АРЕНА)
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS lobbies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    creator_id INTEGER,
    title TEXT NOT NULL,
    interest_tag TEXT, -- "Психология", "Игры", "Карьера"
    rules TEXT,
    is_private BOOLEAN DEFAULT 0,
    password TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creator_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS arena_battles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    topic TEXT NOT NULL,
    status TEXT DEFAULT 'open', -- 'open', 'active', 'voting', 'finished'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Участники битв (спикеры)
CREATE TABLE IF NOT EXISTS arena_participants (
    battle_id INTEGER,
    user_id INTEGER,
    score_received INTEGER DEFAULT 0,
    PRIMARY KEY (battle_id, user_id),
    FOREIGN KEY (battle_id) REFERENCES arena_battles(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Голосование зрителей
CREATE TABLE IF NOT EXISTS arena_votes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    battle_id INTEGER,
    voter_id INTEGER,
    target_user_id INTEGER, -- За кого голос
    rating INTEGER CHECK (rating BETWEEN 1 AND 10),
    comment TEXT,
    FOREIGN KEY (battle_id) REFERENCES arena_battles(id),
    FOREIGN KEY (voter_id) REFERENCES users(id),
    FOREIGN KEY (target_user_id) REFERENCES users(id)
);

--------------------------------------------------------------------------------
-- 4. КОНТЕНТ (ДНЕВНИК И БЛОГ)
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS journal_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    content TEXT NOT NULL,
    entry_type TEXT DEFAULT 'private', -- 'private' (дневник) или 'public' (блог)
    mood_score INTEGER,                -- Самочувствие от 1 до 10
    word_count INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

--------------------------------------------------------------------------------
-- 5. ЕЖЕДНЕВНЫЕ КВЕСТЫ И МАГАЗИН
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS daily_quests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    xp_reward INTEGER DEFAULT 50,
    action_type TEXT, -- 'create_blog', 'vote_arena', 'join_lobby'
    validation_type TEXT,
    target_value INTEGER DEFAULT 1
);

CREATE TABLE IF NOT EXISTS quest_proofs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    quest_id INTEGER,
    proof_text TEXT, -- "Сегодня я подошел к незнакомцу и спросил дорогу"
    status TEXT DEFAULT 'pending', -- 'pending', 'approved', 'rejected'
    votes_count INTEGER DEFAULT 0,  -- Для квестов с проверкой сообществом
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (quest_id) REFERENCES daily_quests(id)
);

CREATE TABLE IF NOT EXISTS user_quest_status (
    user_id INTEGER,
    quest_id INTEGER,
    is_completed BOOLEAN DEFAULT 0,
    updated_at DATE DEFAULT CURRENT_DATE,
    PRIMARY KEY (user_id, quest_id, updated_at),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (quest_id) REFERENCES daily_quests(id)
);

CREATE TABLE IF NOT EXISTS shop_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    price INTEGER NOT NULL,
    category TEXT -- 'badge', 'color', 'theme'
);

CREATE TABLE IF NOT EXISTS user_inventory (
    user_id INTEGER,
    item_id INTEGER,
    is_equipped BOOLEAN DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (item_id) REFERENCES shop_items(id)
);

CREATE TABLE IF NOT EXISTS refresh_tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token_hash TEXT UNIQUE NOT NULL,
    expires_at DATETIME NOT NULL,
);
