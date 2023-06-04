mod vote_type;
pub use vote_type::VoteType;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use crate::schema::votes;

#[derive(Debug, Clone, Insertable, Identifiable, Queryable, AsChangeset)]
#[diesel(table_name=votes,primary_key(user_id, movie_id))]
pub struct Vote {
    pub user_id: i32,
    pub movie_id: i32,
    pub vote: VoteType,
}
