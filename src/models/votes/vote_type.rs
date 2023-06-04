use std::io::Write;

use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::AsExpression;

use crate::schema::sql_types::Vote as SchemaVote;

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = SchemaVote)]
pub enum VoteType {
    Positive,
    Negative,
}

impl ToSql<SchemaVote, Pg> for VoteType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            VoteType::Positive => out.write_all(b"positive")?,
            VoteType::Negative => out.write_all(b"negative")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<SchemaVote, Pg> for VoteType {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"positive" => Ok(VoteType::Positive),
            b"negative" => Ok(VoteType::Negative),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
