package repository

import (
	"context"

	"testing"
	"time"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5/pgconn"
	"github.com/pashagolub/pgxmock/v2"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestUniqueConstraint_Violation(t *testing.T) {
	t.Parallel()
	mock, _ := pgxmock.NewPool()
	q := New(mock)

	id1 := uuid.New()
	id2 := uuid.New()
	username := "duplicate_user"

	// First insert succeeds
	mock.ExpectQuery("INSERT").WithArgs(id1, username, "a@x.com").
		WillReturnRows(pgxmock.NewRows([]string{"id"}).AddRow(id1))

	// Second fails with unique violation
	mock.ExpectQuery("INSERT").WithArgs(id2, username, "b@x.com").
		WillReturnError(&pgconn.PgError{Code: "23505"}) // unique_violation

	_, _ = q.CreateUser(context.Background(), CreateUserParams{ID: id1, Username: username, Email: "a@x.com"})
	_, err := q.CreateUser(context.Background(), CreateUserParams{ID: id2, Username: username, Email: "b@x.com"})

	require.Error(t, err)
	pgErr := err.(*pgconn.PgError)
	assert.Equal(t, "23505", pgErr.Code)
}

func TestContextCancellation_StopsQuery(t *testing.T) {
	t.Parallel()
	mock, _ := pgxmock.NewPool()
	q := New(mock)

	ctx, cancel := context.WithCancel(context.Background())
	mock.ExpectQuery("SELECT").WillDelayFor(100 * time.Millisecond).
		WillReturnRows(pgxmock.NewRows([]string{"count"}).AddRow(int64(1)))

	cancel() // Cancel before query

	_, err := q.CountUsers(ctx)
	assert.ErrorIs(t, err, context.Canceled)
}
