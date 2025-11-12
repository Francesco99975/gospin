-- name: GetAllUsers :many
SELECT id, username, email, created
FROM users
ORDER BY created DESC;

-- name: GetUserByID :one
SELECT id, username, email, created
FROM users
WHERE id = $1;

-- name: CreateUser :one
INSERT INTO users (id, username, email)
VALUES ($1, $2, $3)
RETURNING id, username, email, created;

-- name: UpdateUserEmail :one
UPDATE users
SET email = $1
WHERE id = $2
RETURNING id, username, email, created;

-- name: DeleteUser :execrows
DELETE FROM users
WHERE id = $1;

-- name: CountUsers :one
SELECT COUNT(*) FROM users;
