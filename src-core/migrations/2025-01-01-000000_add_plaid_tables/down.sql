-- Drop indexes
DROP INDEX IF EXISTS idx_plaid_connections_item_id;
DROP INDEX IF EXISTS idx_plaid_accounts_account_id;
DROP INDEX IF EXISTS idx_plaid_accounts_connection_id;

-- Drop tables
DROP TABLE IF EXISTS plaid_accounts;
DROP TABLE IF EXISTS plaid_connections;