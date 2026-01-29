import React, { useState, useRef, useEffect } from 'react';
import { Send, Terminal, Zap, ChevronLeft, MoreVertical } from 'lucide-react';
import type { LobbyDto } from "../bindings";

interface LobbyChatProps {
  lobby: LobbyDto;
  onBack?: () => void;
}

const LobbyChat: React.FC<LobbyChatProps> = ({ lobby, onBack }) => {
  const [message, setMessage] = useState("");
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, []);

  return (
    <div className="flex flex-col h-screen w-full bg-[#0a0412] text-white fixed inset-0 z-[60]">

      <div className="safe-top bg-[#11051a]/80 backdrop-blur-xl border-b border-white/5 sticky top-0 z-20">
        <div className="px-4 py-3 flex items-center gap-3">
          <button onClick={onBack} className="p-2 -ml-2 hover:bg-white/5 rounded-full transition-colors">
            <ChevronLeft size={24} />
          </button>

          <div className="flex-1 min-w-0">
            <h2 className="text-base font-black uppercase italic leading-tight truncate">
              {lobby.name}
            </h2>
            <div className="flex items-center gap-2">
              <div className="w-1.5 h-1.5 bg-green-500 rounded-full animate-pulse" />
              <span className="text-[10px] text-purple-500 font-bold tracking-widest uppercase">
                {lobby.topic || "Secure Line"}
              </span>
            </div>
          </div>

          <button className="p-2 hover:bg-white/5 rounded-full opacity-50">
            <MoreVertical size={20} />
          </button>
        </div>
      </div>

      <div
        ref={scrollRef}
        className="flex-1 overflow-y-auto px-4 py-6 space-y-4 scrollbar-hide"
      >
        <div className="flex gap-3 max-w-[90%]">
          <div className="w-8 h-8 rounded-lg bg-purple-600 flex-shrink-0 flex items-center justify-center text-[10px] font-black border border-purple-400">
            SYS
          </div>
          <div className="bg-white/5 p-3 rounded-2xl rounded-tl-none border border-white/5">
            <p className="text-sm text-slate-300">
              Welcome to the <span className="text-purple-400 font-bold">{lobby.name}</span> terminal.
              Keep transmissions brief.
            </p>
          </div>
        </div>

        <div className="flex gap-3 max-w-[90%] flex-row-reverse ml-auto">
          <div className="bg-purple-600 p-3 rounded-2xl rounded-tr-none shadow-lg shadow-purple-500/10">
            <p className="text-sm font-medium">Is the neural grid stable for deployment?</p>
            <span className="text-[8px] opacity-50 mt-1 block text-right font-black">12:44 PM</span>
          </div>
        </div>
      </div>

      <div className="p-4 bg-[#0a0412] border-t border-white/5 safe-bottom">
        <div className="relative flex items-center gap-2">
          <div className="relative flex-1">
            <input
              type="text"
              value={message}
              onChange={(e) => setMessage(e.target.value)}
              placeholder="Type transmission..."
              className="w-full bg-white/5 border border-white/10 rounded-2xl py-3.5 px-5 pr-12 text-sm font-medium focus:outline-none focus:border-purple-500/50 focus:bg-white/[0.08] transition-all"
            />
            <div className="absolute left-0 -top-6">
              <span className="text-[8px] font-black text-slate-600 uppercase tracking-[0.2em] flex items-center gap-1">
                <Zap size={8} className="fill-current" /> Sync Active
              </span>
            </div>
          </div>

          <button
            className={`p-3.5 rounded-xl transition-all active:scale-90 ${message.trim() ? 'bg-purple-600 text-white shadow-lg shadow-purple-500/20' : 'bg-white/5 text-slate-500'
              }`}
          >
            <Send size={20} />
          </button>
        </div>
      </div>
    </div>
  );
};

export default LobbyChat;
