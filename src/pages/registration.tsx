import React, { useState, ChangeEvent, useEffect } from 'react';
import { ErrorBody, commands, RegisterRequest, ErrorCode } from "../bindings";
import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../stores/authStore';

const UI_MESSAGES: Record<ErrorCode, string> = {
  AUTH_INVALID: "Invalid credentials. Please try again.",
  USER_EXISTS: "This username or email is already taken.",
  VALIDATION_ERROR: "Check your input data for errors.",
  NOT_FOUND: "The requested resource was not found.",
  DATABASE_ERROR: "Server database error. Try again later.",
  SERVER_ERROR: "An unexpected server error occurred."
};

const Register: React.FC = () => {
  const [formData, setFormData] = useState<RegisterRequest>({
    username: '',
    email: '',
    password: ''
  });

  const { setIsLoading, isLoading, error, setError } = useAuthStore();
  const navigate = useNavigate();

  useEffect(() => {
    setError(null);
  }, [setError]);

  const formatError = (err: ErrorBody): string => {
    return UI_MESSAGES[err.error_type] || err.message || "Unknown error";
  };

  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (error) setError(null);
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setError(null);

    try {
      setIsLoading(true);
      const response = await commands.register(formData);

      if (response.status === "error") {
        setError(formatError(response.error));
        return;
      }

      console.log('Registration successful:', response);
      navigate("/home");
    } catch (err) {
      setError("Critical connection error. Please check your internet.");
      console.error('IPC Critical Error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-[#05050a] flex items-center justify-center p-4 font-sans text-slate-200">
      <div className="w-full max-w-sm bg-[#0f111a] border border-[#1e2235] rounded-2xl p-8 shadow-2xl">
        <div className="mb-8">
          <h1 className="text-2xl font-semibold tracking-tight">Create Account</h1>
          <p className="text-slate-500 text-sm mt-1">Fill in the details to get started.</p>
        </div>

        {error && (
          <div className="mb-6 p-3 bg-red-500/10 border border-red-500/50 rounded-xl text-red-500 text-xs animate-in fade-in slide-in-from-top-1">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-1.5">
            <label className="text-[10px] font-bold text-slate-500 ml-1 uppercase tracking-widest">Username</label>
            <input
              name="username"
              type="text"
              required
              value={formData.username}
              onChange={handleChange}
              disabled={isLoading}
              className="w-full bg-[#161925] border border-[#262a3d] rounded-xl p-3 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/20 transition-all disabled:opacity-50"
              placeholder="Enter username"
            />
          </div>

          <div className="space-y-1.5">
            <label className="text-[10px] font-bold text-slate-500 ml-1 uppercase tracking-widest">Email Address</label>
            <input
              name="email"
              type="email"
              required
              value={formData.email}
              onChange={handleChange}
              disabled={isLoading}
              className="w-full bg-[#161925] border border-[#262a3d] rounded-xl p-3 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/20 transition-all disabled:opacity-50"
              placeholder="mail@example.com"
            />
          </div>

          <div className="space-y-1.5">
            <label className="text-[10px] font-bold text-slate-500 ml-1 uppercase tracking-widest">Password</label>
            <input
              name="password"
              type="password"
              required
              value={formData.password}
              onChange={handleChange}
              disabled={isLoading}
              className="w-full bg-[#161925] border border-[#262a3d] rounded-xl p-3 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/20 transition-all disabled:opacity-50"
              placeholder="••••••••"
            />
          </div>

          <button
            type="submit"
            disabled={isLoading}
            className={`w-full bg-indigo-600 hover:bg-indigo-500 text-white font-bold py-3.5 rounded-xl transition-all mt-4 active:scale-[0.97] flex justify-center items-center ${isLoading ? 'opacity-70 cursor-not-allowed' : 'shadow-lg shadow-blue-900/20'
              }`}
          >
            {isLoading ? (
              <span className="flex items-center gap-2">
                <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                Processing...
              </span>
            ) : 'Sign Up'}
          </button>
        </form>

        <div className="mt-8 text-center border-t border-[#1e2235] pt-6">
          <p className="text-slate-500 text-xs">
            Already have an account?{' '}
            <button
              onClick={() => navigate('/login')}
              className="text-indigo-500 font-bold hover:text-indigo-400 transition-colors"
            >
              Sign In
            </button>
          </p>
        </div>
      </div>
    </div>
  );
};

export default Register;
