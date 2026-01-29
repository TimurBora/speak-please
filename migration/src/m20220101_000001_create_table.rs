use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // 1. Primary tables (no dependencies)
    // 2. Secondary tables (depend on users/lobbies)
    // 3. Junction/Detail tables (complex relations)
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(ColumnDef::new(Users::LastActiveAt).date_time().not_null())
                    .col(ColumnDef::new(Users::AvatarUrl).string())
                    .col(ColumnDef::new(Users::Bio).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Lobbies::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Lobbies::Ulid).string_len(26).primary_key())
                    .col(ColumnDef::new(Lobbies::Name).string().not_null())
                    .col(ColumnDef::new(Lobbies::Topic).string().not_null())
                    .col(ColumnDef::new(Lobbies::Description).string())
                    .col(ColumnDef::new(Lobbies::OwnerId).string_len(26).not_null())
                    .col(ColumnDef::new(Lobbies::CreatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-lobby-owner")
                            .from(Lobbies::Table, Lobbies::OwnerId)
                            .to(Users::Table, Users::Ulid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Quests::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Quests::Ulid).string_len(26).primary_key())
                    .col(ColumnDef::new(Quests::LobbyId).string_len(26).null())
                    .col(ColumnDef::new(Quests::Title).string().not_null())
                    .col(ColumnDef::new(Quests::Description).string())
                    .col(ColumnDef::new(Quests::Complexity).char_len(1).not_null())
                    .col(ColumnDef::new(Quests::XpReward).integer().not_null())
                    .col(ColumnDef::new(Quests::ValidationType).string().not_null())
                    .col(ColumnDef::new(Quests::TargetValue).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-quest-lobby_id")
                            .from(Quests::Table, Quests::LobbyId)
                            .to(Lobbies::Table, Lobbies::Ulid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserQuestStatus::Table)
                    .if_not_exists()
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
                            .default("IN_PENDING"),
                    )
                    .col(
                        ColumnDef::new(QuestProofs::BeliefsCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(QuestProofs::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(QuestProofs::UpdatedAt)
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
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(QuestProofBeliefs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QuestProofBeliefs::UserId)
                            .string_len(26)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(QuestProofBeliefs::ProofId)
                            .string_len(26)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(QuestProofBeliefs::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(QuestProofBeliefs::UserId)
                            .col(QuestProofBeliefs::ProofId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-belief-user")
                            .from(QuestProofBeliefs::Table, QuestProofBeliefs::UserId)
                            .to(Users::Table, Users::Ulid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-belief-proof")
                            .from(QuestProofBeliefs::Table, QuestProofBeliefs::ProofId)
                            .to(QuestProofs::Table, QuestProofs::Ulid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(LobbyMembers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LobbyMembers::LobbyId)
                            .string_len(26)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LobbyMembers::UserId)
                            .string_len(26)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LobbyMembers::JoinedAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LobbyMembers::Role)
                            .char_len(15)
                            .default("MEMBER"),
                    )
                    .primary_key(
                        Index::create()
                            .col(LobbyMembers::LobbyId)
                            .col(LobbyMembers::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-lobby_members-lobby_id")
                            .from(LobbyMembers::Table, LobbyMembers::LobbyId)
                            .to(Lobbies::Table, Lobbies::Ulid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-lobby_members-user_id")
                            .from(LobbyMembers::Table, LobbyMembers::UserId)
                            .to(Users::Table, Users::Ulid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    // Drop tables in reverse order of creation to avoid foreign key violations
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LobbyMembers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(QuestProofBeliefs::Table).to_owned())
            .await?;
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
            .drop_table(Table::drop().table(Lobbies::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        Ok(())
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
    LastActiveAt,
    AvatarUrl,
    Bio,
}

#[derive(DeriveIden)]
enum Quests {
    Table,
    Ulid,
    LobbyId,
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
    BeliefsCount,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum QuestProofBeliefs {
    Table,
    UserId,
    ProofId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Lobbies {
    Table,
    Ulid,
    Name,
    Topic,
    Description,
    OwnerId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum LobbyMembers {
    Table,
    LobbyId,
    UserId,
    JoinedAt,
    Role,
}
