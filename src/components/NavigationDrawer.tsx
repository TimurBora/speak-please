import React, { ReactNode } from 'react';
import { X, Home, User, Settings, LogOut } from 'lucide-react';
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
  onClick?: () => Promise<void>;
}

const NavigationItem: React.FC<NavigationItemProps> = ({ icon, label, active, color = "text-slate-300", onClick }) => (
  <button
    onClick={onClick}
    className={`w-full flex items-center gap-4 p-3 rounded-xl transition-all active:scale-[0.98] ${active
      ? 'bg-indigo-600/10 text-indigo-500 shadow-[inset_0_0_0_1px_rgba(79,70,229,0.2)]'
      : `hover:bg-[#161925] ${color}`
      }`}
  >
    <span className={active ? "text-indigo-500" : "text-slate-400"}>
      {icon}
    </span>
    <span className="font-medium text-sm">{label}</span>
  </button>
);

const NavigationDrawer: React.FC<NavigationDrawerProps> = ({ isOpen, onClose }) => {
  const navigate = useNavigate();
  return (
    <>
      {/* Overlay (Затемнение фона) */}
      <div
        className={`fixed inset-0 bg-black/50 backdrop-blur-sm z-[60] transition-opacity duration-300 ${isOpen ? 'opacity-100' : 'opacity-0 pointer-events-none'
          }`}
        onClick={onClose}
      />

      {/* Панель (Drawer) */}
      <aside className={`fixed top-0 left-0 bottom-0 z-[70] w-72 bg-[#0f111a] border-r border-[#1e2235] transform transition-transform duration-300 ease-in-out ${isOpen ? 'translate-x-0' : '-translate-x-full'
        }`}>

        {/* Шапка меню */}
        <div className="p-6 border-b border-[#1e2235] flex justify-between items-center">
          <div className="flex flex-col">
            <span className="text-xl font-bold text-slate-100 tracking-tight">App Name</span>
            <span className="text-[10px] text-slate-500 uppercase tracking-widest font-bold">Navigation</span>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-[#161925] rounded-full transition-colors text-slate-400"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Список ссылок */}
        <nav className="p-4 space-y-2">
          <NavigationItem icon={<Home size={20} />} label="Home" active />
          <NavigationItem icon={<User size={20} />} label="Profile" />
          <NavigationItem icon={<Settings size={20} />} label="Settings" />

          <div className="my-4 border-t border-[#1e2235] pt-4">
            <NavigationItem
              icon={<LogOut size={20} />}
              label="Sign Out"
              color="text-red-400 hover:bg-red-500/5"
              onClick={async () => { const logout_result = await commands.logout(); console.log(logout_result); navigate("/login") }}
            />
          </div>
        </nav>

        {/* Футер меню (опционально) */}
        <div className="absolute bottom-6 left-6 right-6">
          <p className="text-[10px] text-slate-600 uppercase tracking-[0.2em]">Version 1.0.4</p>
        </div>
      </aside>
    </>
  );
};

export default NavigationDrawer;
