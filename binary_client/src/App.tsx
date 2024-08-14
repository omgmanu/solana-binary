import type { FC } from 'react';
import WalletContextProvider from './providers/WalletContextProvider';
import { AppBar } from './components/AppBar';

export const App: FC = () => {
  return (
    <WalletContextProvider>
      <AppBar />
      <div>
        <h1>BTC Binary Options</h1>
      </div>
    </WalletContextProvider>
  );
};

export default App;
