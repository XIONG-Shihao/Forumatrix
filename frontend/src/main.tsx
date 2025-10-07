// frontend/src/main.tsx
import React from 'react';
import ReactDOM from 'react-dom/client';

// 🚫 remove: import './index.css';
// 🚫 remove: import './App.css';
import './styles/globals.css'; // ✅ our single global stylesheet

import { App } from './app/App';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
