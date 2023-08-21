use sqlx::PgPool;
use uuid::Uuid;
use crate::errors::EzyTutorError;
use crate::models::tutor::{CreateTutorDto, Tutor};
use chrono::Utc;

pub async fn get_tutors(pool: &PgPool) -> Result<Vec<Tutor>, EzyTutorError> {
    Ok(sqlx::query_as!(Tutor, r#"
        SELECT  id,
                name,
                pic_url,
                profile,
                created_at,
                updated_at,
                deleted_at
        FROM tutors WHERE deleted_at is null
    "#).fetch_all(pool).await?)
}

pub async fn by_id(pool: &PgPool, id: Uuid) -> Result<Tutor, EzyTutorError> {
    Ok(sqlx::query_as!(Tutor, r#"
        SELECT  id,
                name,
                pic_url,
                profile,
                created_at,
                updated_at,
                deleted_at
        FROM tutors WHERE id = $1 AND deleted_at is null
    "#,
    id,
    ).fetch_one(pool).await?)
}

pub async fn create(pool: &PgPool, dto: CreateTutorDto) -> Result<Tutor, EzyTutorError> {
    sqlx::query_as!(Tutor, r#"
        INSERT INTO
            tutors (
                id,
                name,
                pic_url,
                profile,
                created_at,
                updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
                id,
                name,
                pic_url,
                profile,
                created_at,
                updated_at,
                deleted_at
        "#,
        Uuid::new_v4(),
        dto.name,
        dto.pic_url,
        dto.profile,
        Utc::now().naive_utc(),
        Utc::now().naive_utc(),
    ).fetch_one(pool).await.map_err(|err| err.into())
}

pub async fn update(pool: &PgPool, update_data: CreateTutorDto, id: Uuid) -> Result<Tutor, EzyTutorError> {
    Ok(sqlx::query_as!(Tutor, r#"
        UPDATE tutors
          SET   name = $1,
                pic_url = $2,
                profile = $3,
                updated_at = $4
        WHERE id = $5 AND deleted_at is null
        RETURNING
                id,
                name,
                pic_url,
                profile,
                created_at,
                updated_at,
                deleted_at
        "#,
        update_data.name,
        update_data.pic_url,
        update_data.profile,
        Utc::now().naive_utc(),
        id,
    ).fetch_one(pool).await?)
}

pub async fn soft_delete(pool: &PgPool, id: Uuid) -> Result<Tutor, EzyTutorError> {
    sqlx::query_as!(Tutor, r#"
        UPDATE tutors
          SET  deleted_at = $1
        WHERE id = $2 AND deleted_at is null
        RETURNING
                id,
                name,
                pic_url,
                profile,
                created_at,
                updated_at,
                deleted_at
        "#,
        Utc::now().naive_utc(),
        id,
    ).fetch_one(pool).await.map_err(|err| err.into())
}