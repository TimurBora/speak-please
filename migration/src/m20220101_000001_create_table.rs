use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Users Table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Ulid).string_len(26).primary_key())
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(
                        ColumnDef::new(Users::XpBalance)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Users::TotalXpAccumulated)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Users::Level).integer().not_null().default(1))
                    .col(ColumnDef::new(Users::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // 2. Quests Table (Убрали current_value, так как он в UserQuestStatus)
        manager
            .create_table(
                Table::create()
                    .table(Quests::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Quests::Ulid).string_len(26).primary_key())
                    .col(ColumnDef::new(Quests::Title).string().not_null())
                    .col(ColumnDef::new(Quests::Description).string())
                    .col(ColumnDef::new(Quests::Complexity).char_len(1).not_null()) // 'E', 'M', 'H'
                    .col(ColumnDef::new(Quests::XpReward).integer().not_null())
                    .col(ColumnDef::new(Quests::ValidationType).string().not_null())
                    .col(ColumnDef::new(Quests::TargetValue).integer().not_null())
                    .to_owned(),
            )
            .await?;

        // 3. UserQuestStatus Table (Новая таблица на основе вашей модели)
        manager
            .create_table(
                Table::create()
                    .table(UserQuestStatus::Table)
                    .if_not_exists()
                    // Составной первичный ключ
                    .col(
                        ColumnDef::new(UserQuestStatus::UserId)
                            .string_len(26)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserQuestStatus::QuestId)
                            .string_len(26)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserQuestStatus::AssignedAt)
                            .date()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(UserQuestStatus::UserId)
                            .col(UserQuestStatus::QuestId)
                            .col(UserQuestStatus::AssignedAt),
                    )
                    .col(
                        ColumnDef::new(UserQuestStatus::IsCompleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(UserQuestStatus::CurrentValue)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(UserQuestStatus::QuestStatus)
                            .string_len(16)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserQuestStatus::UpdatedAt)
                            .date_time()
                            .not_null(),
                    )
                    // Внешние ключи
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_quest_status-user_id")
                            .from(UserQuestStatus::Table, UserQuestStatus::UserId)
                            .to(Users::Table, Users::Ulid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_quest_status-quest_id")
                            .from(UserQuestStatus::Table, UserQuestStatus::QuestId)
                            .to(Quests::Table, Quests::Ulid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. RefreshTokens
        manager
            .create_table(
                Table::create()
                    .table(RefreshTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RefreshTokens::Ulid)
                            .string_len(26)
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::UserId)
                            .string_len(26)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::TokenHash)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::ExpiresAt)
                            .date_time()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-refresh_token-user_id")
                            .from(RefreshTokens::Table, RefreshTokens::UserId)
                            .to(Users::Table, Users::Ulid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 5. QuestProofs
        manager
            .create_table(
                Table::create()
                    .table(QuestProofs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QuestProofs::Ulid)
                            .string_len(26)
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(QuestProofs::UserId)
                            .string_len(26)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(QuestProofs::QuestId)
                            .string_len(26)
                            .not_null(),
                    )
                    .col(ColumnDef::new(QuestProofs::ProofText).text().not_null())
                    .col(ColumnDef::new(QuestProofs::Photos).json_binary())
                    .col(ColumnDef::new(QuestProofs::VoiceNotes).json_binary())
                    .col(
                        ColumnDef::new(QuestProofs::Status)
                            .string()
                            .not_null()
                            .default("pending"),
                    )
                    .col(
                        ColumnDef::new(QuestProofs::VotesCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(QuestProofs::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-proof-user")
                            .from(QuestProofs::Table, QuestProofs::UserId)
                            .to(Users::Table, Users::Ulid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-proof-quest")
                            .from(QuestProofs::Table, QuestProofs::QuestId)
                            .to(Quests::Table, Quests::Ulid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(QuestProofs::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RefreshTokens::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserQuestStatus::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Quests::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Ulid,
    Username,
    Email,
    PasswordHash,
    XpBalance,
    TotalXpAccumulated,
    Level,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Quests {
    Table,
    Ulid,
    Title,
    Description,
    Complexity,
    XpReward,
    ValidationType,
    TargetValue,
}

#[derive(DeriveIden)]
enum UserQuestStatus {
    Table,
    UserId,
    QuestId,
    IsCompleted,
    CurrentValue,
    QuestStatus,
    AssignedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum RefreshTokens {
    Table,
    Ulid,
    UserId,
    TokenHash,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum QuestProofs {
    Table,
    Ulid,
    UserId,
    QuestId,
    ProofText,
    Photos,
    VoiceNotes,
    Status,
    VotesCount,
    CreatedAt,
}
