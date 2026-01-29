use crate::endpoints::{API, QuestProofUlid, QuestUlid, UserUlid};

// TODO: Think about to save methods in struct
pub enum QuestProofEndpoints {
    // POST /users/{user_id}/quests/{quest_id}/proofs
    InitSubmission(UserUlid, QuestUlid),
    // PATCH /proofs/{proof_id}/confirm
    ConfirmSubmission(QuestProofUlid),
    // GET /proofs/{proof_id}
    GetDetails(QuestProofUlid),
    // GET /users/{user_id}/feed
    GetFeed(UserUlid),
    // POST /proofs/{proof_id}/likes/{user_id}
    BeliefProof(QuestProofUlid, UserUlid),

    // GET /users/{user_id}/journal
    GetUserJournal(UserUlid),
}

impl API for QuestProofEndpoints {
    fn path(&self) -> String {
        match self {
            Self::InitSubmission(user_id, quest_id) => {
                format!("/users/{user_id}/quests/{quest_id}/proofs")
            }

            Self::ConfirmSubmission(proof_id) => format!("/proofs/{proof_id}/confirm"),

            Self::GetDetails(proof_id) => format!("/proofs/{proof_id}"),

            Self::GetFeed(user_id) => format!("/users/{user_id}/feed"),

            Self::BeliefProof(proof_id, user_id) => {
                format!("/proofs/{proof_id}/likes/{user_id}")
            }

            Self::GetUserJournal(user_id) => {
                format!("/users/{user_id}/journal")
            }
        }
    }

    fn template(&self) -> &'static str {
        match self {
            Self::InitSubmission(_, _) => "/users/{user_id}/quests/{quest_id}/proofs",
            Self::ConfirmSubmission(_) => "/proofs/{proof_id}/confirm",
            Self::GetDetails(_) => "/proofs/{proof_id}",
            Self::GetFeed(_) => "/users/{user_id}/feed",
            Self::BeliefProof(_, _) => "/proofs/{proof_id}/likes/{user_id}",
            Self::GetUserJournal(_) => "/users/{user_id}/journal",
        }
    }

    fn is_auth_endpoint(&self) -> bool {
        true
    }
}
