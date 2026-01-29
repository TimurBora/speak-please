import { create } from 'zustand';
import { commands, UserSession } from '../bindings';

interface AuthState {
  isAuthenticated: boolean;
  userSession: UserSession | null;
  error: string | null;
  isLoading: boolean;
  getUserId: () => string | null;
  checkSession: () => Promise<void>;
  setIsLoading: (isLoading: boolean) => void;
  setError: (error: string | null) => void;
  logout: () => Promise<void>;
  resetError: () => void;
}

export const useAuthStore = create<AuthState>((set, get) => ({
  isAuthenticated: false,
  userSession: null,
  error: null,
  isLoading: true,

  getUserId: () => get().userSession?.user_ulid || null,

  checkSession: async () => {
    try {
      const response = await commands.getCurrentSession();
      if (response.status === "ok" && response.data) {
        set({
          isAuthenticated: true,
          userSession: response.data,
        });
      } else {
        set({ isAuthenticated: false, userSession: null });
      }
    } catch (err) {
      set({ isAuthenticated: false, userSession: null });
    } finally {
      set({ isLoading: false });
    }
  },

  logout: async () => {
    try {
      await commands.logout();
    } finally {
      set({ isAuthenticated: false, userSession: null, error: null });
    }
  },

  setIsLoading: (loading: boolean) => set({ isLoading: loading }),
  setError: (err: string | null) => set({ error: err }),
  resetError: () => set({ error: null }),
}));
