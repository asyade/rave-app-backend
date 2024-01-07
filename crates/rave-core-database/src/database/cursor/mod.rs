use sqlx::Postgres;

pub use external_user::*;

mod external_user;

type CursorExecutor = <Postgres as sqlx::Database>::Connection;
