use crate::db::connection::DbConnection;
use crate::errors::WealthfolioError;
use crate::plaid::{PlaidClient, PlaidConfig};
use crate::plaid::models::*;
use chrono::{DateTime, Utc, NaiveDate};
use diesel::prelude::*;
use uuid::Uuid;

pub struct PlaidService {
    client: PlaidClient,
    db_connection: DbConnection,
}

impl PlaidService {
    pub fn new(config: PlaidConfig, db_connection: DbConnection) -> Self {
        let client = PlaidClient::new(config);
        Self {
            client,
            db_connection,
        }
    }

    /// Creates a link token for Plaid Link initialization
    pub async fn create_link_token(&self, user_id: &str) -> Result<String, WealthfolioError> {
        let response = self.client.create_link_token(user_id).await?;
        Ok(response.link_token)
    }

    /// Exchanges public token for access token and stores the connection
    pub async fn exchange_public_token(
        &self,
        public_token: &str,
        institution_name: &str,
    ) -> Result<String, WealthfolioError> {
        // Exchange public token for access token
        let exchange_response = self.client.exchange_public_token(public_token).await?;
        
        // Create connection record
        let connection_id = Uuid::new_v4().to_string();
        let new_connection = NewPlaidConnection {
            id: connection_id.clone(),
            access_token: exchange_response.access_token.clone(),
            item_id: exchange_response.item_id.clone(),
            institution_id: String::new(), // Will be updated when fetching accounts
            institution_name: institution_name.to_string(),
        };

        // Store in database
        use crate::schema::plaid_connections::dsl::*;
        diesel::insert_into(plaid_connections)
            .values(&new_connection)
            .execute(&mut self.db_connection.get()?)?;

        // Sync accounts immediately after connection
        self.sync_accounts(&connection_id).await?;

        Ok(connection_id)
    }

    /// Syncs accounts for a given connection
    pub async fn sync_accounts(&self, connection_id: &str) -> Result<Vec<PlaidAccount>, WealthfolioError> {
        use crate::schema::plaid_connections::dsl as conn_dsl;
        use crate::schema::plaid_accounts::dsl as acc_dsl;

        // Get the connection
        let connection: PlaidConnection = conn_dsl::plaid_connections
            .find(connection_id)
            .first(&mut self.db_connection.get()?)?;

        // Fetch accounts from Plaid
        let accounts_response = self.client.get_accounts(&connection.access_token).await?;

        // Update institution_id if needed
        if let Some(inst_id) = accounts_response.item.institution_id {
            diesel::update(conn_dsl::plaid_connections.find(connection_id))
                .set(conn_dsl::institution_id.eq(&inst_id))
                .execute(&mut self.db_connection.get()?)?;
        }

        // Store accounts in database
        let mut stored_accounts = Vec::new();
        for plaid_account in accounts_response.accounts {
            let account_id = Uuid::new_v4().to_string();
            let new_account = NewPlaidAccount {
                id: account_id.clone(),
                connection_id: connection_id.to_string(),
                account_id: plaid_account.account_id,
                account_name: plaid_account.name,
                account_type: plaid_account.account_type,
                account_subtype: plaid_account.subtype,
                currency: plaid_account.balances.iso_currency_code,
                current_balance: plaid_account.balances.current,
                available_balance: plaid_account.balances.available,
            };

            diesel::insert_into(acc_dsl::plaid_accounts)
                .values(&new_account)
                .execute(&mut self.db_connection.get()?)?;

            let account: PlaidAccount = acc_dsl::plaid_accounts
                .find(&account_id)
                .first(&mut self.db_connection.get()?)?;
            
            stored_accounts.push(account);
        }

        Ok(stored_accounts)
    }

    /// Syncs transactions for all accounts in a connection
    pub async fn sync_transactions(
        &self,
        connection_id: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<PlaidTransaction>, WealthfolioError> {
        use crate::schema::plaid_connections::dsl::*;

        // Get the connection
        let connection: PlaidConnection = plaid_connections
            .find(connection_id)
            .first(&mut self.db_connection.get()?)?;

        // Fetch transactions from Plaid
        let transactions_response = self.client
            .get_transactions(&connection.access_token, start_date, end_date)
            .await?;

        // Convert transactions to activities format and store them
        for transaction in &transactions_response.transactions {
            self.convert_transaction_to_activity(transaction, connection_id)?;
        }

        Ok(transactions_response.transactions)
    }

    /// Syncs all connections (can be called periodically)
    pub async fn sync_all_connections(&self) -> Result<(), WealthfolioError> {
        use crate::schema::plaid_connections::dsl::*;

        let connections: Vec<PlaidConnection> = plaid_connections
            .load(&mut self.db_connection.get()?)?;

        for connection in connections {
            // Sync accounts
            self.sync_accounts(&connection.id).await?;

            // Sync last 30 days of transactions
            let end_date = chrono::Local::now().naive_local().date();
            let start_date = end_date - chrono::Duration::days(30);
            
            self.sync_transactions(&connection.id, start_date, end_date).await?;
        }

        Ok(())
    }

    /// Gets all connections
    pub fn get_connections(&self) -> Result<Vec<PlaidConnection>, WealthfolioError> {
        use crate::schema::plaid_connections::dsl::*;
        
        let connections = plaid_connections
            .load(&mut self.db_connection.get()?)?;
        
        Ok(connections)
    }

    /// Gets accounts for a specific connection
    pub fn get_accounts_by_connection(&self, connection_id: &str) -> Result<Vec<PlaidAccount>, WealthfolioError> {
        use crate::schema::plaid_accounts::dsl::*;
        
        let accounts = plaid_accounts
            .filter(crate::schema::plaid_accounts::connection_id.eq(connection_id))
            .load(&mut self.db_connection.get()?)?;
        
        Ok(accounts)
    }

    /// Removes a connection and all associated data
    pub fn remove_connection(&self, connection_id: &str) -> Result<(), WealthfolioError> {
        use crate::schema::plaid_connections::dsl as conn_dsl;
        use crate::schema::plaid_accounts::dsl as acc_dsl;

        // Delete accounts first (foreign key constraint)
        diesel::delete(acc_dsl::plaid_accounts.filter(acc_dsl::connection_id.eq(connection_id)))
            .execute(&mut self.db_connection.get()?)?;

        // Delete connection
        diesel::delete(conn_dsl::plaid_connections.find(connection_id))
            .execute(&mut self.db_connection.get()?)?;

        Ok(())
    }

    /// Converts Plaid transaction to Wealthfolio activity
    fn convert_transaction_to_activity(
        &self,
        transaction: &PlaidTransaction,
        connection_id: &str,
    ) -> Result<(), WealthfolioError> {
        use crate::activities::{Activity, NewActivity};
        use crate::schema::activities::dsl::*;
        use crate::schema::plaid_accounts::dsl as acc_dsl;

        // Find the corresponding account
        let account: PlaidAccount = acc_dsl::plaid_accounts
            .filter(acc_dsl::account_id.eq(&transaction.account_id))
            .filter(acc_dsl::connection_id.eq(connection_id))
            .first(&mut self.db_connection.get()?)?;

        // Determine activity type based on transaction
        let activity_type = if transaction.amount < 0.0 {
            "DEPOSIT"
        } else {
            "WITHDRAWAL"
        };

        // Create activity
        let new_activity = NewActivity {
            id: Uuid::new_v4().to_string(),
            account_id: account.id.clone(),
            activity_type: activity_type.to_string(),
            date: transaction.date.and_hms_opt(0, 0, 0).unwrap(),
            quantity: transaction.amount.abs(),
            unit_price: 1.0,
            currency: transaction.iso_currency_code.clone().unwrap_or("USD".to_string()),
            fee: 0.0,
            comment: Some(format!("Plaid: {}", transaction.name)),
            asset_id: format!("CASH_{}", account.currency),
        };

        // Check if transaction already exists (to avoid duplicates)
        let existing = activities
            .filter(crate::schema::activities::comment.eq(&new_activity.comment))
            .filter(crate::schema::activities::date.eq(&new_activity.date))
            .filter(crate::schema::activities::quantity.eq(&new_activity.quantity))
            .first::<Activity>(&mut self.db_connection.get()?)
            .optional()?;

        if existing.is_none() {
            diesel::insert_into(activities)
                .values(&new_activity)
                .execute(&mut self.db_connection.get()?)?;
        }

        Ok(())
    }
}