/* 
  i just did this file to know how to use indoc and trying to refactor the query itself, 
  but it can't be used with query_as! macro so i'll left it in case is needed for any other
  query that wouldn't need the compile-time checking or what we expect as response.
 */
use indoc::indoc;

pub const LIST_USERS: &str = indoc! {
  r#"
      SELECT
          id,
          first_name,
          last_name,
          email,
          avatar,
          role AS "role: Role",
          created_at,
          updated_at
      FROM users
  "#
};

pub const CREATE_USERS: &str = indoc! {
  r#"
      INSERT INTO users
          (
              id,
              first_name,
              last_name,
              email,
              avatar,
              role,
              created_at,
              updated_at
          )
      VALUES
          ($1, $2, $3, $4, $5, $6, $7, $8)
      RETURNING
          id,
          first_name,
          last_name,
          email,
          avatar,
          role AS "role: Role",
          created_at,
          updated_at
  "#
};

pub const USER_BY_ID: &str = indoc! {
  r#"
      SELECT
          id,
          first_name,
          last_name,
          email,
          avatar,
          role AS "role: Role",
          created_at,
          updated_at
      FROM users 
      WHERE id = $1
  "#
};

pub const UPDATE_USER: &str = indoc! {
  r#"
      UPDATE
          users
          SET
              first_name = COALESCE($1, first_name),
              last_name = COALESCE($2, last_name),
              email = COALESCE($3, email),
              avatar = COALESCE($4, avatar),
              role = COALESCE($5, role)
      WHERE id = $6
      RETURNING 
          first_name,
          last_name,
          email,
          avatar,
          role AS "role: Role"
  "#
};

pub const DELETE_USER: &str = indoc! {
  "DELETE FROM users WHERE id = $1"
};
