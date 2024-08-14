import { FC } from 'react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import '@solana/wallet-adapter-react-ui/styles.css';

export const AppBar: FC = () => {
  return (
    <div>
      <WalletMultiButton />
    </div>
  );
};
