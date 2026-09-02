use chrono::Utc;
use sqlx::{PgPool, postgres::PgQueryResult};
use uuid::Uuid;

use crate::schema::representative::{NewRepresentative, NewRepresentativeRelation, Relationship, Representative, RepresentativeWithRelation, StudentRepresentatives, UpdateRepresentative};

pub async fn create_one_with_relation(pool: &PgPool, new_representative: NewRepresentative, representative_relation: NewRepresentativeRelation) -> Result<RepresentativeWithRelation, sqlx::Error>{
    let mut tx = pool.begin().await?;

    let representative = Representative{
        id: Uuid::new_v4(),
        first_name: new_representative.first_name,
        last_name: new_representative.last_name,
        phone: new_representative.phone,
        created_at: Utc::now(),
        updated_at: Utc::now()
    };

    let relation = StudentRepresentatives{
        student_id: representative_relation.student_id,
        representative_id: representative.id,
        relationship: representative_relation.relationship as Relationship,
        is_primary: representative_relation.is_primary
    };
    
    let created_rep = sqlx::query_as!(
        Representative,
        r#"
            INSERT INTO representatives
                (
                    id,
                    first_name,
                    last_name,
                    phone,
                    created_at,
                    updated_at
                )
            VALUES
                ($1, $2, $3, $4, $5, $6)
            RETURNING
                id,
                first_name,
                last_name,
                phone,
                created_at,
                updated_at
        "#,
        representative.id,
        representative.first_name,
        representative.last_name,
        representative.phone,
        representative.created_at,
        representative.updated_at
    )
    .fetch_one(&mut *tx)
    .await?;

    let created_rel = sqlx::query_as!(
        StudentRepresentatives,
        r#"
            INSERT INTO student_representatives
                (
                    student_id,
                    representative_id,
                    relationship,
                    is_primary
                )
            VALUES
                ($1, $2, $3, $4)
            RETURNING
                student_id,
                representative_id,
                relationship AS "relationship: Relationship",
                is_primary
        "#,
        relation.student_id,
        relation.representative_id,
        relation.relationship as Relationship,
        relation.is_primary
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(RepresentativeWithRelation {
    representative: created_rep,
    relation: created_rel
    })
}

pub async fn find_all(pool: &PgPool) -> Result<Vec<Representative>, sqlx::Error>{
    sqlx::query_as!(
        Representative,
        r#"
            SELECT
                id,
                first_name,
                last_name,
                phone,
                created_at,
                updated_at
            FROM representatives
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Representative, sqlx::Error> {
    sqlx::query_as!(
        Representative, 
        r#"
            SELECT
                id,
                first_name,
                last_name,
                phone,
                created_at,
                updated_at
            FROM representatives 
            WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn update_one(pool: &PgPool, id: Uuid, representative_to_update: UpdateRepresentative) -> Result<UpdateRepresentative, sqlx::Error> {
    sqlx::query_as!(
        UpdateRepresentative, 
        r#"
            UPDATE
                representatives
                SET
                    first_name = COALESCE($1, first_name),
                    last_name = COALESCE($2, last_name),
                    phone = COALESCE($3, phone)
            WHERE id = $4
            RETURNING
                first_name,
                last_name, 
                phone
        "#,
        representative_to_update.first_name,
        representative_to_update.last_name,
        representative_to_update.phone,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_one(pool: &PgPool, id: Uuid) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM representatives WHERE id = $1", id)
        .execute(pool)
        .await
}