import React from 'react';
import { Crown, ChevronRight, Hash, Calendar, CheckCircle2 } from 'lucide-react';
import type { LobbyDto } from "../bindings";

interface LobbyItemProps {
  lobby: LobbyDto;
  isMember: boolean;
  onAction: () => void;
}

const LobbyItem: React.FC<LobbyItemProps> = ({ lobby, isMember, onAction }) => {
  const dateFormatted = new Date(lobby.created_at).toLocaleDateString('en-US', {
    day: 'numeric',
    month: 'short',
  });

  return (
    <div className={`group relative w-full mb-4 rounded-[2rem] border transition-all duration-300 overflow-hidden p-6
      ${isMember
        ? 'border-purple-500/30 bg-purple-900/5 shadow-[inset_0_0_20px_rgba(168,85,247,0.05)]'
        : 'border-white/5 bg-[#11051a] hover:border-purple-500/40'}`}>

      {isMember && (
        <div className="absolute top-0 right-0 bg-purple-500 text-[8px] font-black uppercase px-4 py-1 rounded-bl-xl tracking-[0.2em] flex items-center gap-1 shadow-lg">
          <CheckCircle2 size={10} /> Active Member
        </div>
      )}

      <div className="relative z-10">
        <div className="flex justify-between items-start mb-4">
          <div className="flex items-center gap-2 px-3 py-1 bg-purple-500/10 border border-purple-500/20 rounded-full">
            <Hash size={12} className="text-purple-400" />
            <span className="text-[10px] font-black uppercase tracking-widest text-purple-300">
              {lobby.topic}
            </span>
          </div>
          <div className="flex items-center gap-1.5 text-slate-500 font-bold text-[10px] uppercase">
            <Calendar size={12} />
            {dateFormatted}
          </div>
        </div>

        <div className="mb-6">
          <h3 className="text-2xl font-black uppercase italic text-white leading-none mb-2 group-hover:text-purple-400 transition-colors">
            {lobby.name}
          </h3>
          {lobby.description && (
            <p className="text-sm text-slate-400 line-clamp-1 italic font-medium">
              "{lobby.description}"
            </p>
          )}
        </div>

        <div className="flex items-center justify-between border-t border-white/5 pt-4">
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-lg bg-white/5 border border-white/10 flex items-center justify-center">
              <Crown size={14} className="text-amber-400" />
            </div>
            <div className="flex flex-col">
              <span className="text-[9px] text-slate-500 font-black uppercase leading-none">Owner ID</span>
              <span className="text-xs font-bold text-slate-300">{lobby.owner_id.slice(0, 8)}...</span>
            </div>
          </div>

          <button
            onClick={onAction}
            className={`flex items-center gap-2 px-5 py-2.5 rounded-xl font-black text-[11px] uppercase tracking-tighter transition-all active:scale-95
              ${isMember
                ? 'bg-purple-600 text-white shadow-[0_0_15px_rgba(168,85,247,0.4)]'
                : 'bg-white text-black hover:bg-purple-500 hover:text-white'}`}
          >
            {isMember ? 'Open Console' : 'Enter Lobby'} <ChevronRight size={14} />
          </button>
        </div>
      </div>
    </div>
  );
};

export default LobbyItem;
