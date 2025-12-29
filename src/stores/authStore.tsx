
import { create } from 'zustand';
import { commands, UserSession } from '../bindings';

interface AuthState {
  isAuthenticated: boolean;
  userSession: UserSession | null;
  error: string | null;
  isLoading: boolean;
  checkSession: () => Promise<void>;
  setIsLoading: (isLoading: boolean) => void;
  setIsAuthenticated: (isAuthenticated: boolean) => void;
  setError: (error: string | null) => void;
  logout: () => Promise<void>;
}

export const useAuthStore = create<AuthState>((set) => ({
  isAuthenticated: false,
  userSession: null,
  error: null,
  isLoading: true,

  checkSession: async () => {
    if (useAuthStore.getState().isLoading && useAuthStore.getState().userSession !== null) return;

    set({ isLoading: true, error: null });

    try {
      const response = await commands.getCurrentSession();

      if (response.status === "ok") {
        if (response.data) {
          set({
            isAuthenticated: true,
            userSession: response.data,
            isLoading: false,
            error: null
          });
        } else {
          set({ isAuthenticated: false, userSession: null, isLoading: false });
        }
      } else {
        set({
          isAuthenticated: false,
          userSession: null,
          isLoading: false,
          error: null,
        });
      }
    } catch (err) {
      console.error("IPC Bridge Error:", err);
      set({
        isAuthenticated: false,
        isLoading: false
      });
    }
  },

  logout: async () => {
    try {
      await commands.logout();
      set({ isAuthenticated: false, userSession: null });
    } catch (err) {
      console.error("Logout error:", err);
    }
  },

  setIsLoading: (isLoading: boolean) => set({ isLoading }),
  setError: (error: string | null) => set({ error }),
  setIsAuthenticated: (isAuthenticated: boolean) => set({ isAuthenticated })
}));
