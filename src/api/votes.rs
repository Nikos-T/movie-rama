use actix_web::web::{Data, Json, ReqData};
use actix_web::{post, HttpResponse};

use serde::{Deserialize, Serialize};

use crate::db::Database;
use crate::middleware::TokenClaims;
use crate::models::votes::{Vote, VoteType};

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct NewVoteBody {
    pub movie_id: i32,
    pub vote: VoteTypeBody,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteVoteBody {
    pub movie_id: i32,
}

#[post("/vote")]
pub async fn create_vote(
    db: Data<Database>,
    req_user: Option<ReqData<TokenClaims>>,
    vote: Json<NewVoteBody>,
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

    match db.vote(vote) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[post("/delete_vote")]
pub async fn delete_vote(
    db: Data<Database>,
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

    match db.delete_vode(to_delete) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
