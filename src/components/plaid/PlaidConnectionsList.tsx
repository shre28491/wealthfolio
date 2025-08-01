import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { useToast } from '@/components/ui/use-toast';
import { Icons } from '@/components/icons';
import { Badge } from '@/components/ui/badge';
import { formatDate } from '@/lib/utils';

interface PlaidConnection {
  id: string;
  institution_name: string;
  created_at: string;
  updated_at: string;
}

interface PlaidAccount {
  id: string;
  account_name: string;
  account_type: string;
  account_subtype?: string;
  currency: string;
  current_balance: number;
  available_balance?: number;
}

export function PlaidConnectionsList() {
  const [connections, setConnections] = useState<PlaidConnection[]>([]);
  const [accounts, setAccounts] = useState<Record<string, PlaidAccount[]>>({});
  const [isLoading, setIsLoading] = useState(true);
  const [isSyncing, setIsSyncing] = useState<string | null>(null);
  const { toast } = useToast();

  const fetchConnections = async () => {
    try {
      setIsLoading(true);
      const conns = await invoke<PlaidConnection[]>('get_plaid_connections');
      setConnections(conns);

      // Fetch accounts for each connection
      const accountsMap: Record<string, PlaidAccount[]> = {};
      for (const conn of conns) {
        const accs = await invoke<PlaidAccount[]>('get_plaid_accounts', {
          connectionId: conn.id,
        });
        accountsMap[conn.id] = accs;
      }
      setAccounts(accountsMap);
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to load bank connections',
        variant: 'destructive',
      });
      console.error('Error fetching connections:', error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchConnections();
  }, []);

  const handleSync = async (connectionId: string) => {
    try {
      setIsSyncing(connectionId);
      
      // Sync accounts
      await invoke('sync_plaid_accounts', { connectionId });
      
      // Sync last 30 days of transactions
      const endDate = new Date().toISOString().split('T')[0];
      const startDate = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000)
        .toISOString()
        .split('T')[0];
      
      await invoke('sync_plaid_transactions', {
        connectionId,
        startDate,
        endDate,
      });

      toast({
        title: 'Success',
        description: 'Bank data synced successfully',
      });

      // Refresh the data
      await fetchConnections();
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to sync bank data',
        variant: 'destructive',
      });
      console.error('Error syncing:', error);
    } finally {
      setIsSyncing(null);
    }
  };

  const handleRemove = async (connectionId: string) => {
    try {
      await invoke('remove_plaid_connection', { connectionId });
      toast({
        title: 'Success',
        description: 'Bank connection removed',
      });
      await fetchConnections();
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to remove connection',
        variant: 'destructive',
      });
      console.error('Error removing connection:', error);
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center p-8">
        <Icons.spinner className="h-8 w-8 animate-spin" />
      </div>
    );
  }

  if (connections.length === 0) {
    return (
      <Card>
        <CardContent className="text-center py-8">
          <p className="text-muted-foreground">No bank connections yet</p>
        </CardContent>
      </Card>
    );
  }

  return (
    <div className="space-y-4">
      {connections.map((connection) => (
        <Card key={connection.id}>
          <CardHeader>
            <div className="flex items-center justify-between">
              <div>
                <CardTitle>{connection.institution_name}</CardTitle>
                <CardDescription>
                  Connected on {formatDate(connection.created_at)}
                </CardDescription>
              </div>
              <div className="flex gap-2">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => handleSync(connection.id)}
                  disabled={isSyncing === connection.id}
                >
                  {isSyncing === connection.id ? (
                    <Icons.spinner className="h-4 w-4 animate-spin" />
                  ) : (
                    <Icons.refresh className="h-4 w-4" />
                  )}
                  Sync
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => handleRemove(connection.id)}
                >
                  <Icons.trash className="h-4 w-4" />
                  Remove
                </Button>
              </div>
            </div>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {accounts[connection.id]?.map((account) => (
                <div
                  key={account.id}
                  className="flex items-center justify-between p-3 bg-muted rounded-lg"
                >
                  <div>
                    <p className="font-medium">{account.account_name}</p>
                    <div className="flex gap-2 mt-1">
                      <Badge variant="secondary">{account.account_type}</Badge>
                      {account.account_subtype && (
                        <Badge variant="outline">{account.account_subtype}</Badge>
                      )}
                    </div>
                  </div>
                  <div className="text-right">
                    <p className="font-medium">
                      {account.currency} {account.current_balance.toFixed(2)}
                    </p>
                    {account.available_balance !== undefined && (
                      <p className="text-sm text-muted-foreground">
                        Available: {account.currency} {account.available_balance.toFixed(2)}
                      </p>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      ))}
    </div>
  );
}