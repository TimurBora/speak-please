import { BrowserRouter as Router, Routes, Route, Navigate, useLocation } from 'react-router-dom';
import Register from './pages/registration';
import Home from './pages/home';
import { useEffect, ReactNode } from 'react';
import { useAuthStore } from './stores/authStore';
import Login from './pages/login';
import Feed from './pages/feed';
import Archive from './pages/archive';
import Lobbies from './pages/lobby_feed';

const AuthGuard = ({ children, requireAuth }: { children: ReactNode, requireAuth: boolean }) => {
  const { checkSession, isAuthenticated, isLoading } = useAuthStore();
  const location = useLocation();

  useEffect(() => {
    checkSession();
  }, [location.pathname]);

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

function App() {
  return (
    <Router>
      <Routes>
        <Route
          path="/register"
          element={
            <AuthGuard requireAuth={false}>
              <Register />
            </AuthGuard>
          }
        />

        <Route
          path="/login"
          element={
            <AuthGuard requireAuth={false}>
              <Login />
            </AuthGuard>
          }
        />

        <Route
          path="/home"
          element={
            <AuthGuard requireAuth={true}>
              <Home />
            </AuthGuard>
          }
        />

        <Route
          path="/feed"
          element={
            <AuthGuard requireAuth={true}>
              <Feed />
            </AuthGuard>
          }
        />

        <Route
          path="/profile"
          element={
            <AuthGuard requireAuth={true}>
              <Archive />
            </AuthGuard>
          }
        />

        <Route
          path="/lobby"
          element={
            <AuthGuard requireAuth={true}>
              <Lobbies></Lobbies>
            </AuthGuard>
          }
        />

        <Route path="/" element={<Navigate to="/home" replace />} />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </Router>
  );
}

export default App;
