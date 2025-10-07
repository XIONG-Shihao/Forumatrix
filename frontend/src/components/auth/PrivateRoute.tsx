import React from 'react';
import { Navigate, useLocation } from 'react-router-dom';
import { getAuth } from '../../features/auth/state';

type Props = { children: React.ReactNode };

export default function PrivateRoute({ children }: Props) {
  const auth = getAuth();
  const location = useLocation();

  if (!auth?.userId) {
    // send them to /auth and remember where they tried to go
    return <Navigate to="/auth" replace state={{ from: location }} />;
  }

  return <>{children}</>;
}