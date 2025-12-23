// shared/src/models/user_dto.rs
use serde::{Deserialize, Serialize};

#[derive(Deserialize)] // Для сервера: превратить JSON в структуру
#[derive(Serialize)] // Для клиента: превратить структуру в JSON
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub level: i32,
    // Пароль здесь НЕ НУЖЕН. Никогда не отправляй хеш пароля клиенту!
}

// TODO: THINK ABOUT IT
