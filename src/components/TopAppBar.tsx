import React from 'react';
import { List, Bell } from 'lucide-react'; // Добавил иконку уведомлений для баланса
import { useNavigate } from 'react-router-dom';

interface TopAppBarProps {
  title: string;
  onMenuClick?: () => void;
}

const TopAppBar: React.FC<TopAppBarProps> = ({ title, onMenuClick }) => {
  const navigate = useNavigate();

  return (
    <header className="fixed top-0 left-0 right-0 z-50 w-full h-16 bg-[#0d0018]/40 backdrop-blur-xl flex items-center justify-between px-6 border-b border-white/5">

      {/* Левая часть: Меню и Титул */}
      <div className="flex items-center gap-4">
        <button
          onClick={onMenuClick}
          className="relative group p-2 hover:bg-purple-500/10 rounded-xl transition-all duration-300"
        >
          <List className="w-6 h-6 text-slate-400 group-hover:text-purple-400 group-active:scale-90 transition-all" />
          {/* Маленькая точка-индикатор для красоты */}
          <div className="absolute top-2 right-2 w-2 h-2 bg-purple-500 rounded-full opacity-0 group-hover:opacity-100 blur-[2px] transition-opacity" />
        </button>

        <div className="flex flex-col">
          <h1 className="text-sm font-bold tracking-widest text-white uppercase leading-none">
            {title}
          </h1>
          <span className="text-[10px] text-purple-400/60 font-medium tracking-tight">Personal Dashboard</span>
        </div>
      </div>

      {/* Правая часть: Аватар или Доп. кнопка (для баланса дизайна) */}
      <div className="flex items-center gap-2">
        <button className="p-2 text-slate-400 hover:text-white transition-colors">
          <Bell size={20} />
        </button>
        <div
          onClick={() => navigate('/profile')}
          className="w-8 h-8 rounded-full bg-gradient-to-tr from-purple-600 to-indigo-500 border border-white/20 cursor-pointer hover:scale-110 transition-transform flex items-center justify-center text-[10px] font-bold text-white shadow-lg shadow-purple-500/20"
        >
          TB
        </div>
      </div>

    </header>
  );
};

export default TopAppBar;
