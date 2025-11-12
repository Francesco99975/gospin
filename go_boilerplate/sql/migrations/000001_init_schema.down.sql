-- Drop the trigger applied to the users table
DROP TRIGGER IF EXISTS trigger_update_updated_users ON users;

-- Drop the apply_update_trigger function
DROP FUNCTION IF EXISTS apply_update_trigger(TEXT);

-- Drop the update_updated trigger function
DROP FUNCTION IF EXISTS update_updated();

-- Drop the users table
DROP TABLE IF EXISTS users;
