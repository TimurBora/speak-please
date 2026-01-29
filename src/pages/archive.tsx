import React, { useState, useEffect, useCallback } from 'react';
import {
  Archive as ArchiveIcon,
  RefreshCcw,
  Calendar,
  Sparkles,
  CheckCircle2,
  Clock,
  ShieldAlert,
  Trophy
} from 'lucide-react';

import TopAppBar from '../components/TopAppBar';
import NavigationDrawer from '../components/NavigationDrawer';
import BottomAppBar from '../components/BottomAppBar';
import { commands, ProofDetailsResponse } from '../bindings';

const Archive: React.FC = () => {
  const [isDrawerOpen, setIsDrawerOpen] = useState(false);
  const [proofs, setProofs] = useState<ProofDetailsResponse[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  const fetchMyArchive = useCallback(async () => {
    setIsLoading(true);
    try {
      const res = await commands.getMyJournal();
      if (res.status === 'ok') setProofs(res.data);
    } catch (err) {
      console.error("Failed to fetch journal", err);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchMyArchive();
  }, [fetchMyArchive]);

  const getStatusIcon = (status: string) => {
    switch (status.toUpperCase()) {
      case 'COMPLETED': return <CheckCircle2 size={14} className="text-emerald-400" />;
      case 'PENDING': return <Clock size={14} className="text-amber-400" />;
      default: return <ShieldAlert size={14} className="text-rose-400" />;
    }
  };

  return (
    <div className="min-h-screen bg-[#05000a] text-slate-200">
      <TopAppBar title="Personal Archive" onMenuClick={() => setIsDrawerOpen(true)} />
      <NavigationDrawer isOpen={isDrawerOpen} onClose={() => setIsDrawerOpen(false)} />

      <main className="pt-28 pb-40 px-6 max-w-4xl mx-auto">
        {/* Header */}
        <header className="mb-10 flex justify-between items-end">
          <div>
            <div className="flex items-center gap-2 text-purple-500 mb-2">
              <Trophy size={16} />
              <span className="text-[10px] font-black uppercase tracking-[0.4em]">Legacy</span>
            </div>
            <h1 className="text-5xl font-black italic uppercase text-white tracking-tighter">
              My <span className="text-purple-600">Archive</span>
            </h1>
          </div>
          <button
            onClick={fetchMyArchive}
            className="p-4 bg-white/5 border border-white/10 rounded-2xl active:scale-90 transition-all"
          >
            <RefreshCcw size={20} className={isLoading ? 'animate-spin' : ''} />
          </button>
        </header>

        {isLoading ? (
          <div className="space-y-6">
            {[1, 2, 3].map(i => (
              <div key={i} className="h-40 bg-white/5 rounded-[2rem] animate-pulse" />
            ))}
          </div>
        ) : proofs.length === 0 ? (
          <div className="py-20 text-center border-2 border-dashed border-white/5 rounded-[3rem]">
            <p className="text-slate-500 font-black uppercase italic tracking-widest">Archive is empty. Go earn some XP.</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-4">
            {proofs.map((proof) => (
              <div
                key={proof.ulid}
                className="group relative overflow-hidden bg-[#0d0415] border border-white/5 rounded-[2.5rem] p-6 hover:border-purple-500/40 transition-all duration-500"
              >
                {proof.photo_urls?.[0] && (
                  <div className="absolute top-0 right-0 w-1/3 h-full opacity-20 pointer-events-none">
                    <img src={proof.photo_urls[0]} className="w-full h-full object-cover" style={{ maskImage: 'linear-gradient(to left, black, transparent)' }} />
                  </div>
                )}

                <div className="relative z-10">
                  <div className="flex justify-between items-start mb-4">
                    <div className="flex items-center gap-3">
                      <div className="px-3 py-1 bg-white/5 rounded-full flex items-center gap-2 border border-white/10">
                        {getStatusIcon(proof.status)}
                        <span className="text-[9px] font-black uppercase tracking-tighter">{proof.status}</span>
                      </div>
                      <span className="text-amber-400 text-[10px] font-black italic">+{proof.xp_reward} XP</span>
                    </div>
                    <div className="flex items-center gap-1.5 text-slate-500">
                      <Calendar size={12} />
                      <span className="text-[10px] font-bold">{new Date(proof.created_at).toLocaleDateString()}</span>
                    </div>
                  </div>

                  <h3 className="text-2xl font-black uppercase italic text-white mb-2 leading-none group-hover:text-purple-400 transition-colors">
                    {proof.quest_title}
                  </h3>

                  {proof.proof_text && (
                    <p className="text-slate-400 text-sm italic mb-6 max-w-lg line-clamp-1">
                      "{proof.proof_text}"
                    </p>
                  )}

                  <div className="flex items-center gap-3">
                    <div className="flex items-center gap-2 px-3 py-1.5 bg-purple-500/10 border border-purple-500/20 rounded-xl">
                      <div className="flex -space-x-1.5">
                        {[...Array(Math.min(3, proof.beliefs_count))].map((_, i) => (
                          <div key={i} className="w-4 h-4 rounded-full bg-purple-600 border border-[#0d0415]" />
                        ))}
                      </div>
                      <span className="text-[10px] font-black text-purple-300 uppercase">
                        {proof.beliefs_count} BELIEVERS
                      </span>
                    </div>

                    {proof.voice_urls?.length > 0 && (
                      <div className="px-3 py-1.5 bg-blue-500/10 border border-blue-500/20 rounded-xl text-[10px] font-black text-blue-400 uppercase">
                        AUDIO ATTACHED
                      </div>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </main>

      <BottomAppBar isHidden={isDrawerOpen} />
    </div>
  );
};

export default Archive;
