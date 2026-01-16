import React, { ReactNode } from 'react';
import { X, Home, User, Settings, LogOut, Sparkles } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { commands } from '../bindings';

interface NavigationDrawerProps {
  isOpen: boolean;
  onClose: () => void;
}

interface NavigationItemProps {
  icon: ReactNode;
  label: string;
  active?: boolean;
  color?: string;
  onClick?: () => void | Promise<void>;
}

const NavigationItem: React.FC<NavigationItemProps> = ({ icon, label, active, color = "text-slate-400", onClick }) => (
  <button
    onClick={onClick}
    className={`w-full flex items-center gap-4 p-4 rounded-2xl transition-all duration-300 active:scale-[0.95] group ${active
        ? 'bg-gradient-to-r from-purple-600/20 to-transparent text-purple-400 shadow-[inset_0_0_20px_rgba(168,85,247,0.05)] border-l-4 border-purple-500'
        : `hover:bg-purple-500/5 ${color} hover:text-white`
      }`}
  >
    <span className={`transition-transform duration-300 group-hover:scale-110 ${active ? "text-purple-400 drop-shadow-[0_0_8px_rgba(168,85,247,0.5)]" : "text-slate-500"}`}>
      {icon}
    </span>
    <span className={`font-bold text-sm tracking-wide ${active ? "text-white" : ""}`}>{label}</span>
  </button>
);

const NavigationDrawer: React.FC<NavigationDrawerProps> = ({ isOpen, onClose }) => {
  const navigate = useNavigate();

  return (
    <>
      {/* Overlay с глубоким размытием */}
      <div
        className={`fixed inset-0 bg-[#05000a]/60 backdrop-blur-xl z-[60] transition-opacity duration-500 ${isOpen ? 'opacity-100' : 'opacity-0 pointer-events-none'
          }`}
        onClick={onClose}
      />

      {/* Панель (Drawer) */}
      <aside className={`fixed top-0 left-0 bottom-0 z-[70] w-80 bg-[#0d0018] border-r border-white/5 shadow-[20px_0_50px_rgba(0,0,0,0.3)] transform transition-transform duration-500 cubic-bezier(0.4, 0, 0.2, 1) ${isOpen ? 'translate-x-0' : '-translate-x-full'
        }`}>

        {/* Шапка меню */}
        <div className="p-8 mb-4 flex justify-between items-center">
          <div className="flex flex-col gap-1">
            <div className="flex items-center gap-2">
              <Sparkles size={16} className="text-purple-500" />
              <span className="text-2xl font-black text-white tracking-tighter italic">QUEST LOG</span>
            </div>
            <span className="text-[10px] text-purple-500/60 uppercase tracking-[0.3em] font-bold">Main Menu</span>
          </div>
          <button
            onClick={onClose}
            className="p-2.5 bg-purple-950/30 border border-white/5 rounded-xl transition-all hover:bg-purple-500/20 text-slate-400 hover:text-white"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Список ссылок */}
        <nav className="px-4 space-y-1">
          <NavigationItem
            icon={<Home size={22} />}
            label="Dashboard"
            active
            onClick={() => { navigate("/"); onClose(); }}
          />
          <NavigationItem
            icon={<User size={22} />}
            label="Player Profile"
            onClick={() => { navigate("/profile"); onClose(); }}
          />
          <NavigationItem
            icon={<Settings size={22} />}
            label="System Settings"
            onClick={() => { navigate("/settings"); onClose(); }}
          />

          <div className="mx-4 my-8 border-t border-white/5 pt-8">
            <NavigationItem
              icon={<LogOut size={22} />}
              label="Terminate Session"
              color="text-rose-400 hover:bg-rose-500/10"
              onClick={async () => {
                const logout_result = await commands.logout();
                console.log(logout_result);
                navigate("/login");
              }}
            />
          </div>
        </nav>

        {/* Футер меню */}
        <div className="absolute bottom-10 left-8 right-8">
          <div className="p-4 bg-purple-950/20 rounded-2xl border border-white/5">
            <p className="text-[10px] text-slate-500 uppercase tracking-[0.2em] mb-1">Build Version</p>
            <p className="text-xs font-mono text-purple-400/80">v1.0.4-STABLE</p>
          </div>
        </div>
      </aside>
    </>
  );
};

export default NavigationDrawer;
