import React from 'react';
import { Plus, LayoutGrid, CalendarDays, CheckSquare, Briefcase } from 'lucide-react';

interface BottomAppBarProps {
  isHidden?: boolean;
}

const BottomAppBar: React.FC<BottomAppBarProps> = ({ isHidden = false }) => {
  return (
    <div className={`fixed bottom-0 left-0 right-0 z-[60] transition-transform duration-500 ease-in-out ${isHidden ? 'translate-y-full' : 'translate-y-0'}`} style={{
      paddingBottom: 'env(safe-area-inset-bottom)',
      height: 'calc(var(--bottom-bar-height) + env(safe-area-inset-bottom))',
    }}
    >
      {/* Мягкая тень сверху для отделения от контента */}
      <div className="absolute -top-12 inset-x-0 h-12 bg-gradient-to-t from-[#0d0018] to-transparent pointer-events-none" />

      <div className="relative flex items-center justify-around bg-[#1a0029]/90 backdrop-blur-2xl h-20 px-4 border-t border-white/10 shadow-[0_-10px_40px_rgba(0,0,0,0.5)]">
        <NavItem icon={<LayoutGrid size={22} />} label="Habits" />
        <NavItem icon={<CalendarDays size={22} />} label="Dailies" />

        <div className="relative -top-6">
          <div className="absolute inset-0 bg-purple-500 blur-xl opacity-20 animate-pulse" />
          <button className="relative bg-gradient-to-tr from-[#7c3aed] to-[#a78bfa] p-4 rounded-2xl rotate-45 shadow-[0_8px_20px_rgba(124,58,237,0.3)] hover:scale-110 hover:rotate-[135deg] transition-all duration-500 border-4 border-[#0d0018]">
            <div className="-rotate-45">
              <Plus size={28} color="white" strokeWidth={3} />
            </div>
          </button>
        </div>

        <NavItem icon={<CheckSquare size={22} />} label="To Do's" active />
        <NavItem icon={<Briefcase size={22} />} label="Rewards" />
      </div>
    </div>
  );
};

const NavItem = ({ icon, label, active = false }: { icon: React.ReactNode, label: string, active?: boolean }) => (
  <button className={`flex flex-col items-center transition-all duration-300 ${active ? 'text-purple-400' : 'text-slate-500 hover:text-white'}`}>
    <div className={`${active ? 'drop-shadow-[0_0_8px_rgba(168,85,247,0.5)]' : ''}`}>
      {icon}
    </div>
    <span className="text-[10px] font-medium mt-1 uppercase tracking-wider">{label}</span>
  </button>
);

export default BottomAppBar;
