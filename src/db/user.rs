use crate::db::{errors::*, schema::users, Pool};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use diesel::{
    dsl::{AsSelect, SqlTypeOf},
    pg::Pg,
    prelude::*,
};
use rand_core::OsRng;

#[derive(serde::Serialize, Queryable, Selectable, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub login: String,
    pub hashed_password: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub login: String,
    pub hashed_password: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

type SqlType = SqlTypeOf<AsSelect<User, Pg>>;
type BoxedQuery<'a> = users::BoxedQuery<'a, Pg, SqlType>;

impl User {
    pub async fn register(
        pool: &Pool,
        login: String,
        password: String,
        name: String,
        email: String,
        is_admin: bool,
    ) -> Result<User, DatabaseError<impl std::error::Error>> {
        let user = User::find(pool, User::by_email(email.to_owned()))
            .await
            .map_err(|_| UserError::Query)?;

        if user.is_some() {
            return Err(DatabaseError::Operation(UserError::Exists));
        }

        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .map_err(|_| UserError::HashPassword)
            .map(|hash| hash.to_string())?;

        let new_user = NewUser {
            login,
            hashed_password,
            name,
            email,
            is_admin,
        };

        let user = User::create(pool, new_user)
            .await
            .map_err(|_| UserError::Query)?;

        Ok(user)
    }

    pub async fn create(
        pool: &Pool,
        new_user: NewUser,
    ) -> Result<User, DatabaseError<impl std::error::Error>> {
        let connection = pool.get().await.map_err(DatabaseError::Connection)?;
        let user = connection
            .interact(move |connection| {
                diesel::insert_into(users::table)
                    .values(new_user)
                    .returning(User::as_returning())
                    .get_result(connection)
            })
            .await
            .map_err(|_| DatabaseError::Interaction)?
            .map_err(|_| DatabaseError::Interaction)?;

        Ok(user)
    }

    pub fn by_email(email: String) -> BoxedQuery<'static> {
        users::table
            .into_boxed()
            .select(User::as_select())
            .filter(users::email.eq(email))
    }

    pub fn by_id(id: uuid::Uuid) -> BoxedQuery<'static> {
        users::table
            .into_boxed()
            .select(User::as_select())
            .filter(users::id.eq(id))
    }

    pub async fn find(
        pool: &Pool,
        query: BoxedQuery<'static>,
    ) -> Result<Option<User>, DatabaseError<impl std::error::Error>> {
        let connection = pool.get().await.map_err(DatabaseError::Connection)?;
        let user = connection
            .interact(move |connection| query.first(connection).optional())
            .await
            .map_err(|_| DatabaseError::Interaction)?
            .map_err(|_| DatabaseError::Interaction)?;

        Ok(user)
    }

    pub async fn remove(
        pool: &Pool,
        user: User,
    ) -> Result<(), DatabaseError<impl std::error::Error>> {
        let connection = pool.get().await.map_err(DatabaseError::Connection)?;
        connection
            .interact(move |connection| {
                diesel::delete(users::table.filter(users::id.eq(user.id))).execute(connection)
            })
            .await
            .map_err(|_| DatabaseError::Interaction)?
            .map_err(|_| DatabaseError::Interaction)?;

        Ok(())
    }
}
