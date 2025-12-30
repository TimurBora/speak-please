use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
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
                    .col(
                        ColumnDef::new(Users::DailyQuestsStreak)
                            .integer()
                            .not_null()
                            .default(0),
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
                    .table(DailyQuests::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DailyQuests::Ulid)
                            .string_len(26)
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DailyQuests::Title).string().not_null())
                    .col(ColumnDef::new(DailyQuests::Description).string())
                    .col(
                        ColumnDef::new(DailyQuests::Complexity)
                            .char_len(1)
                            .not_null(),
                    )
                    .col(ColumnDef::new(DailyQuests::XpReward).integer().not_null())
                    .col(ColumnDef::new(DailyQuests::ActionType).string().not_null())
                    .col(
                        ColumnDef::new(DailyQuests::ValidationType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DailyQuests::TargetValue)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RefreshTokens::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DailyQuests::Table).to_owned())
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
    DailyQuestsStreak,
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
enum DailyQuests {
    Table,
    Ulid,
    Title,
    Description,
    Complexity,
    XpReward,
    ActionType,
    ValidationType,
    TargetValue,
}
