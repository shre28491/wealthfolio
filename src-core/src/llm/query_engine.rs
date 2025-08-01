use crate::db::connection::DbConnection;
use crate::errors::WealthfolioError;
use crate::llm::{LlmClient, LlmConfig};
use crate::accounts::Account;
use crate::activities::Activity;
use crate::assets::Asset;
use crate::portfolio::{PortfolioService, Holdings};
use diesel::prelude::*;
use serde_json::json;

pub struct QueryEngine {
    llm_client: LlmClient,
    db_connection: DbConnection,
}

impl QueryEngine {
    pub fn new(config: LlmConfig, db_connection: DbConnection) -> Self {
        let llm_client = LlmClient::new(config);
        Self {
            llm_client,
            db_connection,
        }
    }

    pub async fn query(&self, user_query: &str) -> Result<String, WealthfolioError> {
        // Gather context from the database
        let context = self.gather_financial_context()?;
        
        // Create system prompt with financial data context
        let system_prompt = format!(
            r#"You are a helpful financial assistant for the Wealthfolio app. You have access to the user's financial data and can answer questions about their portfolio, transactions, and overall financial health.

Current Financial Context:
{}

Guidelines:
- Provide accurate information based on the data available
- Format monetary values appropriately with currency symbols
- Be concise but informative
- If asked about specific calculations, show your work
- If data is missing or unclear, acknowledge it
- Respect user privacy and don't make assumptions beyond the data provided"#,
            context
        );

        // Generate response
        self.llm_client
            .generate_response(&system_prompt, user_query)
            .await
    }

    fn gather_financial_context(&self) -> Result<String, WealthfolioError> {
        use crate::schema::{accounts, activities, assets};

        // Get accounts
        let accounts: Vec<Account> = accounts::table
            .load(&mut self.db_connection.get()?)?;

        // Get recent activities
        let recent_activities: Vec<Activity> = activities::table
            .order(activities::date.desc())
            .limit(50)
            .load(&mut self.db_connection.get()?)?;

        // Get portfolio summary
        let portfolio_service = PortfolioService::new(self.db_connection.clone());
        let holdings = portfolio_service.get_holdings()?;

        // Get total portfolio value
        let total_value: f64 = holdings.iter().map(|h| h.market_value).sum();

        // Build context
        let mut context = String::new();
        
        // Accounts summary
        context.push_str("Accounts:\n");
        for account in &accounts {
            context.push_str(&format!(
                "- {} ({}, {}): {}\n",
                account.name,
                account.account_type,
                account.currency,
                if account.is_active { "Active" } else { "Inactive" }
            ));
        }

        // Portfolio summary
        context.push_str(&format!("\nTotal Portfolio Value: ${:.2}\n", total_value));
        context.push_str("\nTop Holdings:\n");
        
        let mut sorted_holdings = holdings.clone();
        sorted_holdings.sort_by(|a, b| b.market_value.partial_cmp(&a.market_value).unwrap());
        
        for (i, holding) in sorted_holdings.iter().take(10).enumerate() {
            context.push_str(&format!(
                "{}. {} - {:.2} shares @ ${:.2} = ${:.2} ({:.1}%)\n",
                i + 1,
                holding.symbol,
                holding.quantity,
                holding.market_price.unwrap_or(0.0),
                holding.market_value,
                (holding.market_value / total_value) * 100.0
            ));
        }

        // Recent transactions
        context.push_str("\nRecent Transactions:\n");
        for activity in recent_activities.iter().take(10) {
            context.push_str(&format!(
                "- {} {} {:.2} {} @ {:.2} on {}\n",
                activity.date.format("%Y-%m-%d"),
                activity.activity_type,
                activity.quantity,
                activity.asset_id,
                activity.unit_price,
                activity.currency
            ));
        }

        // Plaid connections if any
        if let Ok(plaid_connections) = self.get_plaid_connections_summary() {
            context.push_str(&plaid_connections);
        }

        Ok(context)
    }

    fn get_plaid_connections_summary(&self) -> Result<String, WealthfolioError> {
        use crate::schema::{plaid_connections, plaid_accounts};
        use crate::plaid::models::{PlaidConnection, PlaidAccount};

        let connections: Vec<PlaidConnection> = plaid_connections::table
            .load(&mut self.db_connection.get()?)?;

        if connections.is_empty() {
            return Ok(String::new());
        }

        let mut summary = String::from("\nConnected Banks (via Plaid):\n");
        
        for connection in connections {
            let accounts: Vec<PlaidAccount> = plaid_accounts::table
                .filter(plaid_accounts::connection_id.eq(&connection.id))
                .load(&mut self.db_connection.get()?)?;

            summary.push_str(&format!("- {} ({} accounts)\n", 
                connection.institution_name,
                accounts.len()
            ));

            for account in accounts {
                summary.push_str(&format!(
                    "  * {} ({}): {} {:.2}\n",
                    account.account_name,
                    account.account_type,
                    account.currency,
                    account.current_balance
                ));
            }
        }

        Ok(summary)
    }

    pub async fn generate_financial_insights(&self) -> Result<String, WealthfolioError> {
        let prompt = r#"Based on the financial data provided, generate a brief summary of insights including:
1. Portfolio diversification assessment
2. Recent transaction patterns
3. Any notable risks or opportunities
4. Suggestions for improvement

Keep the response concise and actionable."#;

        self.query(prompt).await
    }
}