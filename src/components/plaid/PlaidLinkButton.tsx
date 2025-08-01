import React, { useCallback, useState } from 'react';
import { usePlaidLink } from 'react-plaid-link';
import { invoke } from '@tauri-apps/api/tauri';
import { Button } from '@/components/ui/button';
import { useToast } from '@/components/ui/use-toast';
import { Icons } from '@/components/icons';

interface PlaidLinkButtonProps {
  onSuccess?: () => void;
}

export function PlaidLinkButton({ onSuccess }: PlaidLinkButtonProps) {
  const [linkToken, setLinkToken] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const { toast } = useToast();

  // Create link token
  const createLinkToken = useCallback(async () => {
    try {
      setIsLoading(true);
      const token = await invoke<string>('create_plaid_link_token', {
        userId: 'default-user', // You might want to get this from your app state
      });
      setLinkToken(token);
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to initialize Plaid Link',
        variant: 'destructive',
      });
      console.error('Error creating link token:', error);
    } finally {
      setIsLoading(false);
    }
  }, [toast]);

  // Handle successful link
  const handleOnSuccess = useCallback(
    async (publicToken: string, metadata: any) => {
      try {
        setIsLoading(true);
        const connectionId = await invoke<string>('exchange_plaid_public_token', {
          publicToken,
          institutionName: metadata.institution?.name || 'Unknown Bank',
        });

        toast({
          title: 'Success',
          description: `Successfully connected to ${metadata.institution?.name}`,
        });

        // Sync accounts immediately
        await invoke('sync_plaid_accounts', { connectionId });

        if (onSuccess) {
          onSuccess();
        }
      } catch (error) {
        toast({
          title: 'Error',
          description: 'Failed to connect bank account',
          variant: 'destructive',
        });
        console.error('Error exchanging public token:', error);
      } finally {
        setIsLoading(false);
      }
    },
    [toast, onSuccess]
  );

  const config: Parameters<typeof usePlaidLink>[0] = {
    token: linkToken,
    onSuccess: handleOnSuccess,
    onExit: (err, metadata) => {
      if (err) {
        console.error('Plaid Link error:', err);
      }
    },
  };

  const { open, ready } = usePlaidLink(config);

  const handleClick = useCallback(() => {
    if (!linkToken) {
      createLinkToken();
    } else {
      open();
    }
  }, [linkToken, createLinkToken, open]);

  return (
    <Button
      onClick={handleClick}
      disabled={isLoading || (!ready && linkToken !== null)}
      className="flex items-center gap-2"
    >
      {isLoading ? (
        <Icons.spinner className="h-4 w-4 animate-spin" />
      ) : (
        <Icons.plus className="h-4 w-4" />
      )}
      Connect Bank Account
    </Button>
  );
}