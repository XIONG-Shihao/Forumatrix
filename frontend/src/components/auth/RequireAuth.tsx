import { Navigate, Outlet, useLocation } from 'react-router-dom';
import { getAuth } from '../../features/auth/state';

export default function RequireAuth() {
  const auth = getAuth();
  const location = useLocation();

  if (!auth?.userId) {
    // send them to /auth and remember where they tried to go
    return <Navigate to="/auth" replace state={{ from: location }} />;
  }
  return <Outlet />; // render protected children
}
