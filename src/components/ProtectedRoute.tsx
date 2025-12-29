import { Navigate, useLocation } from 'react-router-dom';
import { useEffect, ReactNode } from 'react';
import { useAuthStore } from '../stores/authStore';

const AuthGuard = ({ children, requireAuth }: { children: ReactNode, requireAuth: boolean }) => {
  const { checkSession, isAuthenticated, isLoading } = useAuthStore();
  const location = useLocation();

  useEffect(() => {
    checkSession();
  }, [location.pathname, checkSession]);

  if (isLoading) {
    return <div className="min-h-screen bg-[#05050a] flex items-center justify-center text-white">Loading...</div>;
  }

  if (requireAuth && !isAuthenticated) {
    return <Navigate to="/register" state={{ from: location }} replace />;
  }

  if (!requireAuth && isAuthenticated) {
    return <Navigate to="/home" replace />;
  }

  return <>{children}</>;
};
