import React from 'react';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { PlaidLinkButton } from '@/components/plaid/PlaidLinkButton';
import { PlaidConnectionsList } from '@/components/plaid/PlaidConnectionsList';
import { FinancialAssistant } from '@/components/llm/FinancialAssistant';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Icons } from '@/components/icons';

export default function IntegrationsPage() {
  const [refreshKey, setRefreshKey] = React.useState(0);

  const handlePlaidSuccess = () => {
    // Refresh the connections list
    setRefreshKey((prev) => prev + 1);
  };

  return (
    <div className="container mx-auto p-6">
      <div className="mb-6">
        <h1 className="text-3xl font-bold">Integrations</h1>
        <p className="text-muted-foreground mt-2">
          Connect your bank accounts and chat with your financial assistant
        </p>
      </div>

      <Tabs defaultValue="banking" className="space-y-4">
        <TabsList className="grid w-full grid-cols-2 max-w-[400px]">
          <TabsTrigger value="banking">
            <Icons.building className="h-4 w-4 mr-2" />
            Banking
          </TabsTrigger>
          <TabsTrigger value="assistant">
            <Icons.messageSquare className="h-4 w-4 mr-2" />
            AI Assistant
          </TabsTrigger>
        </TabsList>

        <TabsContent value="banking" className="space-y-6">
          <Card>
            <CardHeader>
              <div className="flex items-center justify-between">
                <div>
                  <CardTitle>Bank Connections</CardTitle>
                  <CardDescription>
                    Connect your bank accounts to automatically sync transactions and balances
                  </CardDescription>
                </div>
                <PlaidLinkButton onSuccess={handlePlaidSuccess} />
              </div>
            </CardHeader>
            <CardContent>
              <PlaidConnectionsList key={refreshKey} />
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>How it works</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex gap-3">
                <div className="flex-shrink-0 w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center">
                  <span className="text-sm font-semibold">1</span>
                </div>
                <div>
                  <h4 className="font-medium">Connect your bank</h4>
                  <p className="text-sm text-muted-foreground">
                    Click "Connect Bank Account" and securely link your bank through Plaid
                  </p>
                </div>
              </div>
              <div className="flex gap-3">
                <div className="flex-shrink-0 w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center">
                  <span className="text-sm font-semibold">2</span>
                </div>
                <div>
                  <h4 className="font-medium">Automatic syncing</h4>
                  <p className="text-sm text-muted-foreground">
                    Your accounts and transactions are automatically imported and categorized
                  </p>
                </div>
              </div>
              <div className="flex gap-3">
                <div className="flex-shrink-0 w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center">
                  <span className="text-sm font-semibold">3</span>
                </div>
                <div>
                  <h4 className="font-medium">Track everything</h4>
                  <p className="text-sm text-muted-foreground">
                    View all your financial data in one place with real-time updates
                  </p>
                </div>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="assistant" className="space-y-6">
          <FinancialAssistant />

          <Card>
            <CardHeader>
              <CardTitle>About the Financial Assistant</CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              <p className="text-sm text-muted-foreground">
                The AI-powered financial assistant can help you understand your portfolio better.
                It has access to your financial data and can answer questions about:
              </p>
              <ul className="list-disc list-inside space-y-1 text-sm text-muted-foreground ml-4">
                <li>Portfolio performance and holdings</li>
                <li>Transaction history and patterns</li>
                <li>Asset allocation and diversification</li>
                <li>Investment insights and suggestions</li>
                <li>Connected bank account balances</li>
              </ul>
              <p className="text-sm text-muted-foreground">
                All conversations are processed locally and your data never leaves your device.
              </p>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
}