use actix_web::web::{Data, Json, ReqData};
use actix_web::{post, HttpResponse, get, delete};

use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::middleware::TokenClaims;
use crate::models::votes::{Vote, VoteType};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VoteTypeBody {
    Positive,
    Negative,
}

impl Into<VoteType> for VoteTypeBody {
    fn into(self) -> VoteType {
        match self {
            VoteTypeBody::Positive => VoteType::Positive,
            VoteTypeBody::Negative => VoteType::Negative,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserVoteBody {
    pub movie_id: i32,
    pub vote: VoteTypeBody,
}

impl From<Vote> for UserVoteBody {
    fn from(vote: Vote) -> Self {
        Self {
            movie_id: vote.movie_id,
            vote: match vote.vote {
                VoteType::Positive => VoteTypeBody::Positive,
                VoteType::Negative => VoteTypeBody::Negative,
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct DeleteVoteBody {
    pub movie_id: i32,
}

#[post("/vote")]
pub async fn create_vote(
    app_data: Data<AppState>,
    req_user: Option<ReqData<TokenClaims>>,
    vote: Json<UserVoteBody>,
) -> HttpResponse {
    let vote = vote.into_inner();
    let vote = match req_user {
        Some(user) => Vote {
            user_id: user.user_id,
            movie_id: vote.movie_id,
            vote: vote.vote.into(),
        },
        _ => {
            return HttpResponse::Unauthorized().json("Log in to add a vote.");
        }
    };

    match app_data.db.vote(vote) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[delete("/delete_vote")]
pub async fn delete_vote(
    app_data: Data<AppState>,
    req_user: Option<ReqData<TokenClaims>>,
    to_delete: Json<DeleteVoteBody>,
) -> HttpResponse {
    let to_delete = to_delete.into_inner();
    let to_delete = match req_user {
        Some(user) => (user.user_id, to_delete.movie_id),
        _ => {
            return HttpResponse::Unauthorized().json("Log in to delete a vote.");
        }
    };

    match app_data.db.delete_vode(to_delete) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("/get_my_votes")]
pub async fn get_user_votes(
    app_data: Data<AppState>,
    req_user: Option<ReqData<TokenClaims>>,
) -> HttpResponse {
    let user_id = match req_user {
        Some(user) => user.user_id,
        _ => {
            return HttpResponse::Unauthorized().json("Log in to get your votes.");
        }
    };

    match app_data.db.get_votes_by_user(user_id) {
        Ok(votes) => {
            let votes = votes
                            .into_iter()
                            .map(|vote| UserVoteBody::from(vote))
                            .collect::<Vec<UserVoteBody>>();
            HttpResponse::Ok().json(votes)
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

