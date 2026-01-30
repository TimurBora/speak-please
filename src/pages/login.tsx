import React, { useState, ChangeEvent, useEffect } from 'react';
import { ErrorBody, commands, ErrorCode, LoginRequest } from "../bindings";
import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../stores/authStore';
import { Sparkles, ShieldCheck, Mail, Lock, AlertTriangle, Info } from 'lucide-react';

const UI_MESSAGES: Record<ErrorCode, string> = {
  AUTH_INVALID: "Invalid credentials. Double-check your email and password.",
  USER_EXISTS: "Identity already exists in the database.",
  VALIDATION_ERROR: "Input verification failed. See details below.",
  NOT_FOUND: "System core: Resource not located.",
  DATABASE_ERROR: "Storage failure. Data link interrupted.",
  SERVER_ERROR: "Neural link error. Server is unresponsive.",
  CUSTOM_ERROR: "CUSTOM ERROR",
};

const Login: React.FC = () => {
  const [formData, setFormData] = useState<LoginRequest>({
    email: '',
    password: ''
  });

  const { setIsLoading, isLoading, error, setError } = useAuthStore();
  const navigate = useNavigate();

  useEffect(() => {
    setError(null);
  }, [setError]);

  const formatError = (err: ErrorBody): string => {
    return UI_MESSAGES[err.error_type] || err.message || "Critical system failure";
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
      const response = await commands.login(formData);

      if (response.status === "error") {
        setError(formatError(response.error));
        return;
      }

      navigate("/home");
    } catch (err) {
      setError("Network timeout. Check your uplink connection.");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-[#0d0018] relative overflow-hidden flex items-center justify-center p-4 font-sans text-slate-200">

      {/* Background Decor */}
      <div className="absolute top-[-10%] left-[-10%] w-[70%] h-[50%] bg-purple-900/15 rounded-full blur-[120px] pointer-events-none" />
      <div className="absolute bottom-[-10%] right-[-10%] w-[60%] h-[40%] bg-indigo-900/10 rounded-full blur-[100px] pointer-events-none" />

      <div className="relative z-10 w-full max-w-md bg-[#1a0029]/60 backdrop-blur-2xl border border-white/10 rounded-[32px] p-8 shadow-2xl">

        {/* Header */}
        <div className="mb-8 text-center">
          <div className="inline-flex items-center justify-center w-14 h-14 bg-purple-600/10 rounded-2xl border border-purple-500/20 mb-4">
            <ShieldCheck className="w-7 h-7 text-purple-400" />
          </div>
          <h1 className="text-2xl font-black text-white tracking-tighter uppercase">Access Terminal</h1>
          <p className="text-slate-500 text-[10px] font-bold uppercase tracking-[0.2em] mt-1">Authorized Personnel Only</p>
        </div>

        {/* --- УЛУЧШЕННЫЙ БЛОК ОШИБОК --- */}
        {error && (
          <div className="mb-6 overflow-hidden animate-in fade-in slide-in-from-top-4 duration-500">
            <div className="bg-rose-500/10 border border-rose-500/20 rounded-2xl p-4">
              <div className="flex items-start gap-3">
                <AlertTriangle className="w-5 h-5 text-rose-500 shrink-0 mt-0.5" />
                <div className="space-y-1">
                  <p className="text-rose-400 text-xs font-black uppercase tracking-wider">
                    Security Alert
                  </p>
                  <p className="text-rose-200/70 text-xs leading-relaxed">
                    {error}
                  </p>
                </div>
              </div>

              {/* Дополнительная подсказка, если это ошибка валидации */}
              {error.includes("verification") && (
                <div className="mt-3 pt-3 border-t border-rose-500/10 space-y-2">
                  <div className="flex items-center gap-2 text-[9px] text-rose-400/60 font-bold uppercase tracking-tighter">
                    <Info size={10} />
                    Troubleshooting:
                  </div>
                  <ul className="text-[10px] text-rose-300/50 list-disc list-inside space-y-1 ml-1">
                    <li>Ensure email format is correct</li>
                    <li>Password must be at least 8 characters</li>
                  </ul>
                </div>
              )}
            </div>
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-5">
          {/* Email */}
          <div className="space-y-2">
            <label className="text-[10px] font-black text-slate-500 ml-1 uppercase tracking-widest">Login ID (Email)</label>
            <div className="relative group">
              <Mail className="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-600 group-focus-within:text-purple-400 transition-colors" />
              <input
                name="email"
                type="email"
                required
                value={formData.email}
                onChange={handleChange}
                disabled={isLoading}
                className={`w-full bg-purple-950/20 border ${error ? 'border-rose-500/30' : 'border-white/5'} rounded-2xl p-4 pl-12 outline-none focus:border-purple-500/50 transition-all text-white placeholder:text-slate-700`}
                placeholder="mail@identity.com"
              />
            </div>
          </div>

          {/* Password */}
          <div className="space-y-2">
            <label className="text-[10px] font-black text-slate-500 ml-1 uppercase tracking-widest">Access Key</label>
            <div className="relative group">
              <Lock className="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-600 group-focus-within:text-purple-400 transition-colors" />
              <input
                name="password"
                type="password"
                required
                value={formData.password}
                onChange={handleChange}
                disabled={isLoading}
                className={`w-full bg-purple-950/20 border ${error ? 'border-rose-500/30' : 'border-white/5'} rounded-2xl p-4 pl-12 outline-none focus:border-purple-500/50 transition-all text-white placeholder:text-slate-700`}
                placeholder="••••••••"
              />
            </div>
          </div>

          <button
            type="submit"
            disabled={isLoading}
            className="group relative w-full h-14 rounded-2xl overflow-hidden transition-all active:scale-[0.98] mt-4"
          >
            <div className={`absolute inset-0 bg-gradient-to-r ${error ? 'from-rose-600 to-rose-500' : 'from-purple-600 via-indigo-500 to-purple-600'} transition-colors duration-500`} />
            <div className="relative flex items-center justify-center gap-3 font-black text-sm uppercase tracking-widest text-white">
              {isLoading ? (
                <div className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin" />
              ) : (
                'Initiate Login'
              )}
            </div>
          </button>
        </form>

        <div className="mt-8 text-center border-t border-white/5 pt-6">
          <button
            onClick={() => navigate('/register')}
            className="text-[10px] font-black uppercase tracking-widest text-slate-500 hover:text-purple-400 transition-colors"
          >
            Create new identity
          </button>
        </div>
      </div>
    </div>
  );
};

export default Login;
