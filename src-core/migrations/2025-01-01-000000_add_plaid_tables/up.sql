-- Create plaid_connections table
CREATE TABLE plaid_connections (
    id TEXT PRIMARY KEY NOT NULL,
    access_token TEXT NOT NULL,
    item_id TEXT NOT NULL,
    institution_id TEXT NOT NULL,
    institution_name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create plaid_accounts table
CREATE TABLE plaid_accounts (
    id TEXT PRIMARY KEY NOT NULL,
    connection_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    account_name TEXT NOT NULL,
    account_type TEXT NOT NULL,
    account_subtype TEXT,
    currency TEXT NOT NULL,
    current_balance REAL NOT NULL,
    available_balance REAL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (connection_id) REFERENCES plaid_connections(id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX idx_plaid_accounts_connection_id ON plaid_accounts(connection_id);
CREATE INDEX idx_plaid_accounts_account_id ON plaid_accounts(account_id);
CREATE UNIQUE INDEX idx_plaid_connections_item_id ON plaid_connections(item_id);