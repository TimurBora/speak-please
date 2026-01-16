use sea_orm::{ActiveValue::Set, entity::prelude::*};
use serde::{Deserialize, Serialize};
use shared::models::quest_dto::{Complexity, QuestDto};
use ulid::Ulid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "quests")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ulid: String,

    pub title: String,
    pub description: Option<String>,
    pub complexity: Complexity,

    pub xp_reward: u32,

    pub validation_type: String, // TODO: ENUM(AUTOMATIC, COMMUNITY, MODERATION AND OTHER)
    pub target_value: u32,
}

impl ActiveModel {
    pub fn new_daily_quest(
        title: &str,
        description: Option<String>,
        xp_reward: Option<u32>,
        validation_type: &str,
        target_value: Option<u32>,
        complexity: Option<Complexity>,
    ) -> Self {
        let xp_reward = xp_reward.unwrap_or(10);
        let target_value = target_value.unwrap_or(1);
        let complexity = complexity.unwrap_or(Complexity::Easy);
        Self {
            ulid: Set(Ulid::new().to_string()),
            title: Set(title.to_string()),
            description: Set(description),
            xp_reward: Set(xp_reward),
            validation_type: Set(validation_type.to_string()),
            target_value: Set(target_value),
            complexity: Set(complexity),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Deserialize)]
struct QuestSeed {
    title: String,
    description: Option<String>,
    complexity: String,
    xp_reward: u32,
    validation_type: String,
    target_value: u32,
}

pub async fn seed_quests(db: &DatabaseConnection) -> Result<(), DbErr> {
    // Читаем JSON
    let data = include_str!("../quests.json");

    let seeds: Vec<QuestSeed> = serde_json::from_str(data)
        .map_err(|e| DbErr::Custom(format!("JSON parse error: {}", e)))?;

    for seed in seeds {
        // Проверяем, существует ли уже квест с таким заголовком
        let exists = Entity::find()
            .filter(Column::Title.eq(&seed.title))
            .one(db)
            .await?;

        if exists.is_some() {
            println!("Skipping: '{}' (already exists)", seed.title);
            continue;
        }

        // Определяем сложность
        let complexity = match seed.complexity.to_lowercase().as_str() {
            "medium" => Complexity::Medium,
            "hard" => Complexity::Hard,
            _ => Complexity::Easy,
        };

        // Создаем модель без action_type
        let active_model = ActiveModel {
            ulid: Set(ulid::Ulid::new().to_string()),
            title: Set(seed.title.clone()),
            description: Set(seed.description),
            complexity: Set(complexity),
            xp_reward: Set(seed.xp_reward),
            validation_type: Set(seed.validation_type),
            target_value: Set(seed.target_value),
        };

        active_model.insert(db).await?;
        println!("Inserted: '{}'", seed.title);
    }

    Ok(())
}

impl From<Model> for QuestDto {
    fn from(m: Model) -> Self {
        Self {
            ulid: m.ulid,
            title: m.title,
            description: m.description,
            complexity: m.complexity,
            xp_reward: m.xp_reward,
            validation_type: m.validation_type,
            target_value: m.target_value,
        }
    }
}
