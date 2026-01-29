use crate::endpoints::{API, QuestUlid, UserUlid};

pub enum UserQuestEndpoints {
    GetJournal(UserUlid),               // user_ulid
    GetDailyQuests(UserUlid),           // user_ulid
    CompleteQuest(UserUlid, QuestUlid), // user_ulid, quest_id
}

impl API for UserQuestEndpoints {
    fn path(&self) -> String {
        match self {
            Self::GetJournal(user_ulid) => format!("/users/{}/quests", user_ulid),
            Self::GetDailyQuests(user_ulid) => format!("/users/{}/quests/daily", user_ulid),
            Self::CompleteQuest(user_ulid, quest_ulid) => {
                format!("/users/{}/quests/{}", user_ulid, quest_ulid)
            }
        }
    }

    fn template(&self) -> &'static str {
        match self {
            Self::GetJournal(_) => "/users/{user_id}/quests",
            Self::GetDailyQuests(_) => "/users/{user_id}/quests/daily",
            Self::CompleteQuest(_, _) => "/users/{user_id}/quests/{quest_id}",
        }
    }

    fn is_auth_endpoint(&self) -> bool {
        true
    }
}
