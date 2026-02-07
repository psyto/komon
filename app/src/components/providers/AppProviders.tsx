'use client';

import { ReactNode } from 'react';
import { WalletContextProvider } from './WalletProvider';

interface Props {
  children: ReactNode;
}

export function AppProviders({ children }: Props) {
  return (
    <WalletContextProvider>
      {children}
    </WalletContextProvider>
  );
}
