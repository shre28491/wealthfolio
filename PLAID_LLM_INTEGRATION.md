# Plaid and LLM Integration for Wealthfolio

This branch adds Plaid bank integration and an AI-powered financial assistant to the Wealthfolio app.

## Features

### 🏦 Plaid Bank Integration
- Connect any bank account supported by Plaid
- Automatically sync account balances
- Import transactions and categorize them
- Support for multiple bank connections
- Secure authentication through Plaid Link

### 🤖 AI Financial Assistant
- Chat with an AI assistant about your financial data
- Ask questions about portfolio performance, holdings, and transactions
- Get personalized financial insights
- All data processing happens locally - your data never leaves your device

## Setup

### Prerequisites
1. **Plaid Account**: Sign up at [https://dashboard.plaid.com/](https://dashboard.plaid.com/)
   - Create a development account (free)
   - Get your Client ID and Secret from the dashboard

2. **OpenAI API Key**: Get from [https://platform.openai.com/](https://platform.openai.com/)
   - Create an account and generate an API key

### Installation

1. Clone this branch:
```bash
git clone -b plaid-llm-integration https://github.com/shre28491/wealthfolio.git
cd wealthfolio
```

2. Install dependencies:
```bash
# Install frontend dependencies
pnpm install

# The Rust dependencies will be installed automatically when building
```

3. Set up environment variables:
```bash
cp .env.example .env
```

Edit `.env` and add your credentials:
```env
PLAID_CLIENT_ID=your_plaid_client_id
PLAID_SECRET=your_plaid_secret_key
OPENAI_API_KEY=your_openai_api_key
```

4. Run the app:
```bash
pnpm tauri dev
```

## Usage

### Connecting Bank Accounts

1. Navigate to the **Integrations** page from the sidebar
2. Click **Connect Bank Account**
3. Select your bank from the Plaid Link interface
4. Enter your credentials (in sandbox mode, use test credentials)
5. Your accounts will be automatically imported

### Using the Financial Assistant

1. Go to the **Integrations** page
2. Click on the **AI Assistant** tab
3. Type your questions in the chat interface
4. The assistant has access to:
   - Your portfolio holdings and performance
   - Transaction history
   - Account balances from connected banks
   - Asset allocation data

### Example Questions
- "What is my total portfolio value?"
- "Show me my top 5 holdings"
- "How much did I invest last month?"
- "What's my portfolio diversification?"
- "How much cash do I have across all accounts?"

## Technical Details

### Backend Architecture
- **Plaid Integration**: Located in `src-core/src/plaid/`
  - Handles OAuth token exchange
  - Syncs accounts and transactions
  - Stores data in SQLite database

- **LLM Integration**: Located in `src-core/src/llm/`
  - Uses OpenAI's GPT-4 for natural language processing
  - Gathers context from local database
  - Supports embeddings for future semantic search

### Frontend Components
- **Plaid Components**: `src/components/plaid/`
  - PlaidLinkButton: Initiates bank connection
  - PlaidConnectionsList: Manages connected accounts

- **LLM Components**: `src/components/llm/`
  - FinancialAssistant: Chat interface

### Database Schema
New tables added:
- `plaid_connections`: Stores bank connection info
- `plaid_accounts`: Stores account details

### Security Considerations
- Plaid access tokens are encrypted and stored locally
- OpenAI API calls include only aggregated financial data
- No personal information is sent to external services
- All data remains on your local machine

## Testing with Plaid Sandbox

In sandbox mode, use these test credentials:
- Username: `user_good`
- Password: `pass_good`

This will create sample accounts with test data.

## Troubleshooting

### Plaid Connection Issues
- Ensure your Plaid credentials are correct
- Check that you're using the sandbox environment for testing
- Verify your internet connection

### LLM Assistant Issues
- Verify your OpenAI API key is valid
- Check that you have sufficient API credits
- Ensure your local database has data to query

## Future Enhancements
- Real-time transaction categorization
- Budget tracking and alerts
- Investment recommendations
- Multi-language support
- Voice input for the assistant