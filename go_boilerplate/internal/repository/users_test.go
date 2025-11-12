// internal/repository/users_test.go
package repository_test

import (
	"context"
	"regexp"
	"testing"
	"time"

	"github.com/__username__/go_boilerplate/internal/repository"
	"github.com/google/uuid"
	"github.com/pashagolub/pgxmock/v2"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestCreateUser_SQLAndArgs(t *testing.T) {
	t.Parallel()

	mock, err := pgxmock.NewPool()
	require.NoError(t, err)
	defer mock.Close()

	q := repository.New(mock)

	id := uuid.MustParse("12345678-1234-5678-1234-567812345678")
	expectedSQL := regexp.QuoteMeta(`INSERT INTO users (id, username, email) VALUES ($1, $2, $3) RETURNING id, username, email, created`)

	mock.ExpectQuery(expectedSQL).
		WithArgs(id, "john_doe", "john@example.com").
		WillReturnRows(pgxmock.NewRows([]string{"id", "username", "email", "created"}).
			AddRow(id, "john_doe", "john@example.com", time.Now()))

	result, err := q.CreateUser(context.Background(), repository.CreateUserParams{
		ID:       id,
		Username: "john_doe",
		Email:    "john@example.com",
	})

	require.NoError(t, err)
	assert.Equal(t, id, result.ID)
	assert.Equal(t, "john_doe", result.Username)

	require.NoError(t, mock.ExpectationsWereMet())
}

func TestGetUserByID_SQLAndArgs(t *testing.T) {
	t.Parallel()

	mock, err := pgxmock.NewPool()
	require.NoError(t, err)
	defer mock.Close()

	q := repository.New(mock)

	id := uuid.MustParse("11111111-1111-1111-1111-111111111111")
	expectedSQL := regexp.QuoteMeta(`SELECT id, username, email, created FROM users WHERE id = $1`)

	rows := pgxmock.NewRows([]string{"id", "username", "email", "created"}).
		AddRow(id, "alice", "alice@ex.com", time.Now())

	mock.ExpectQuery(expectedSQL).
		WithArgs(id).
		WillReturnRows(rows)

	user, err := q.GetUserByID(context.Background(), id)
	require.NoError(t, err)
	assert.Equal(t, "alice", user.Username)

	require.NoError(t, mock.ExpectationsWereMet())
}

func TestGetAllUsers_SQL(t *testing.T) {
	t.Parallel()

	mock, err := pgxmock.NewPool()
	require.NoError(t, err)
	defer mock.Close()

	q := repository.New(mock)

	expectedSQL := regexp.QuoteMeta(`SELECT id, username, email, created FROM users ORDER BY created DESC`)

	rows := pgxmock.NewRows([]string{"id", "username", "email", "created"}).
		AddRow(uuid.New(), "user1", "u1@x.com", time.Now()).
		AddRow(uuid.New(), "user2", "u2@x.com", time.Now())

	mock.ExpectQuery(expectedSQL).
		WithArgs().
		WillReturnRows(rows)

	users, err := q.GetAllUsers(context.Background())
	require.NoError(t, err)
	assert.Len(t, users, 2)

	require.NoError(t, mock.ExpectationsWereMet())
}

func TestUpdateUserEmail_SQLAndArgs(t *testing.T) {
	t.Parallel()

	mock, err := pgxmock.NewPool()
	require.NoError(t, err)
	defer mock.Close()

	q := repository.New(mock)

	id := uuid.MustParse("22222222-2222-2222-2222-222222222222")
	expectedSQL := regexp.QuoteMeta(`UPDATE users SET email = $1 WHERE id = $2 RETURNING id, username, email, created`)

	mock.ExpectQuery(expectedSQL).
		WithArgs("new@email.com", id).
		WillReturnRows(pgxmock.NewRows([]string{"id", "username", "email", "created"}).
			AddRow(id, "bob", "new@email.com", time.Now()))

	result, err := q.UpdateUserEmail(context.Background(), repository.UpdateUserEmailParams{
		Email: "new@email.com",
		ID:    id,
	})

	require.NoError(t, err)
	assert.Equal(t, "new@email.com", result.Email)

	require.NoError(t, mock.ExpectationsWereMet())
}

func TestDeleteUser_SQLAndRowsAffected(t *testing.T) {
	t.Parallel()

	mock, err := pgxmock.NewPool()
	require.NoError(t, err)
	defer mock.Close()

	q := repository.New(mock)

	id := uuid.MustParse("33333333-3333-3333-3333-333333333333")
	expectedSQL := regexp.QuoteMeta(`DELETE FROM users WHERE id = $1`)

	mock.ExpectExec(expectedSQL).
		WithArgs(id).
		WillReturnResult(pgxmock.NewResult("DELETE", 1))

	affected, err := q.DeleteUser(context.Background(), id)
	require.NoError(t, err)
	assert.Equal(t, int64(1), affected)

	require.NoError(t, mock.ExpectationsWereMet())
}

func TestCountUsers_SQL(t *testing.T) {
	t.Parallel()

	mock, err := pgxmock.NewPool()
	require.NoError(t, err)
	defer mock.Close()

	q := repository.New(mock)

	expectedSQL := regexp.QuoteMeta(`SELECT COUNT(*) FROM users`)

	mock.ExpectQuery(expectedSQL).
		WithArgs().
		WillReturnRows(pgxmock.NewRows([]string{"count"}).AddRow(int64(42)))

	count, err := q.CountUsers(context.Background())
	require.NoError(t, err)
	assert.Equal(t, int64(42), count)

	require.NoError(t, mock.ExpectationsWereMet())
}
