import React, { useState, useEffect, useCallback } from 'react';
import {
  User as UserIcon,
  RefreshCcw,
  ArrowLeft,
  Share2,
  CheckCircle2,
  Star,
  Sparkles,
  HeartHandshake,
  Loader2,
} from 'lucide-react';

import Layout from '../components/Layout';
import { commands, ProofDetailsResponse } from '../bindings';

const CyberAudioPlayer: React.FC<{ src: string }> = ({ src }) => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [duration, setDuration] = useState(0);
  const [currentTime, setCurrentTime] = useState(0);
  const audioRef = React.useRef<HTMLAudioElement>(null);

  const formatTime = (time: number) => {
    const mins = Math.floor(time / 60);
    const secs = Math.floor(time % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const onLoadedMetadata = () => {
    if (audioRef.current) setDuration(audioRef.current.duration);
  };

  const onTimeUpdate = () => {
    if (audioRef.current) setCurrentTime(audioRef.current.currentTime);
  };

  const togglePlay = () => {
    if (isPlaying) audioRef.current?.pause();
    else audioRef.current?.play();
    setIsPlaying(!isPlaying);
  };

  const handleSeek = (e: React.ChangeEvent<HTMLInputElement>) => {
    const time = Number(e.target.value);
    if (audioRef.current) {
      audioRef.current.currentTime = time;
      setCurrentTime(time);
    }
  };

  return (
    <div className="mt-10 p-5 bg-[#1a0b2e]/60 border border-purple-500/20 rounded-[2rem] backdrop-blur-xl relative overflow-hidden group transition-all hover:border-purple-500/40">
      <audio
        ref={audioRef}
        src={src}
        onLoadedMetadata={onLoadedMetadata}
        onTimeUpdate={onTimeUpdate}
        onEnded={() => setIsPlaying(false)}
        className="hidden"
      />

      <div className="relative flex items-center gap-5">
        <button
          onClick={togglePlay}
          className="w-16 h-16 flex-shrink-0 flex items-center justify-center rounded-2xl bg-purple-600 text-white shadow-[0_0_30px_rgba(168,85,247,0.3)] hover:scale-105 active:scale-95 transition-all"
        >
          {isPlaying ? (
            <div className="flex gap-1.5">
              <div className="w-1.5 h-5 bg-white rounded-full animate-[bounce_1s_infinite_0.1s]" />
              <div className="w-1.5 h-5 bg-white rounded-full animate-[bounce_1s_infinite_0.3s]" />
              <div className="w-1.5 h-5 bg-white rounded-full animate-[bounce_1s_infinite_0.2s]" />
            </div>
          ) : (
            <div className="ml-1 w-0 h-0 border-y-[10px] border-y-transparent border-l-[16px] border-l-white rounded-sm" />
          )}
        </button>

        <div className="flex-1 min-w-0">
          <div className="flex justify-between items-end mb-3">
            <div>
              <p className="text-[10px] font-black uppercase tracking-[0.2em] text-purple-400 italic">Voice Evidence</p>
              <p className="text-xs font-bold text-slate-300 mt-0.5">
                {isPlaying ? 'System Playing...' : 'Ready to Decode'}
              </p>
            </div>
            <div className="text-[10px] font-mono text-purple-400 bg-purple-500/10 px-2 py-0.5 rounded-md border border-purple-500/20">
              {formatTime(currentTime)} / {formatTime(duration)}
            </div>
          </div>

          <div className="relative h-2 w-full bg-white/5 rounded-full overflow-hidden">
            <div
              className="absolute top-0 left-0 h-full bg-gradient-to-r from-purple-600 to-fuchsia-500 shadow-[0_0_15px_rgba(168,85,247,0.8)] transition-all duration-100"
              style={{ width: `${(currentTime / duration) * 100}%` }}
            />
            <input
              type="range"
              min="0"
              max={duration || 0}
              value={currentTime}
              onChange={handleSeek}
              className="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10"
            />
          </div>
        </div>
      </div>
    </div>
  );
};

const BelieveButton: React.FC<{
  proofUlid: string;
  isBelieved: boolean;
  count: number;
  variant?: 'compact' | 'full';
  onToggle: (newStatus: boolean, newCount: number) => void;
}> = ({ proofUlid, isBelieved, count, variant = 'full', onToggle }) => {
  const [isPending, setIsPending] = useState(false);

  const handleToggle = async (e: React.MouseEvent) => {
    e.stopPropagation();
    if (isPending) return;

    const next = !isBelieved;
    const nextCount = next ? count + 1 : count - 1;

    onToggle(next, nextCount);
    setIsPending(true);

    try {
      const res = await commands.toggleProofBelief(proofUlid);
      if (res.status !== 'ok') onToggle(isBelieved, count);
    } catch {
      onToggle(isBelieved, count);
    } finally {
      setIsPending(false);
    }
  };

  const activeStyles = isBelieved
    ? 'bg-purple-600 border-purple-400 text-white shadow-[0_0_20px_rgba(168,85,247,0.4)] scale-105'
    : 'bg-white/5 border-white/10 text-slate-500 hover:border-purple-500/50';

  if (variant === 'compact') {
    return (
      <button
        onClick={handleToggle}
        disabled={isPending}
        className={`flex items-center gap-1.5 px-3 py-1.5 rounded-xl border transition-all active:scale-90 ${activeStyles}`}
      >
        <HeartHandshake size={14} className={isBelieved ? 'fill-current' : ''} />
        <span className="text-[10px] font-black">{count}</span>
      </button>
    );
  }

  return (
    <button
      onClick={handleToggle}
      disabled={isPending}
      className={`group flex items-center gap-3 px-8 py-4 rounded-2xl border font-black uppercase tracking-widest transition-all active:scale-95 ${activeStyles}`}
    >
      {isPending ? (
        <Loader2 size={20} className="animate-spin" />
      ) : (
        <HeartHandshake
          size={20}
          className={isBelieved ? 'animate-pulse fill-current' : ''}
        />
      )}
      <div className="flex flex-col items-start leading-none">
        <span className="text-sm">{isBelieved ? 'I Believe' : 'Support'}</span>
        <span className="text-[9px] opacity-60 mt-0.5">{count} Beliefs</span>
      </div>
    </button>
  );
};

const ProofDetailView: React.FC<{
  proof: ProofDetailsResponse;
  onBack: () => void;
  onUpdate: (ulid: string, patch: Partial<ProofDetailsResponse>) => void;
}> = ({ proof, onBack, onUpdate }) => {
  return (
    <div className="fixed inset-0 z-[100] bg-[#05000a] overflow-y-auto animate-in slide-in-from-bottom duration-500">
      <div className={`sticky top-0 z-[110] px-6 py-4 flex justify-between items-center border-b backdrop-blur-md ${proof.is_believed ? 'bg-purple-900/30 border-purple-500/40' : 'bg-[#05000a]/80 border-white/5'
        }`}>
        <button onClick={onBack} className="p-2 -ml-2 text-slate-400 hover:text-white transition-colors">
          <ArrowLeft size={28} />
        </button>
        {proof.is_believed && (
          <div className="flex items-center gap-2 px-3 py-1 bg-purple-500 border border-purple-400 rounded-full shadow-[0_0_15px_rgba(168,85,247,0.5)]">
            <HeartHandshake size={14} className="text-white fill-current" />
            <span className="text-[10px] font-black uppercase text-white">You Believe</span>
          </div>
        )}
        <button className="p-2 text-slate-400"><Share2 size={20} /></button>
      </div>

      <div className="relative h-[35vh] w-full">
        {proof.photo_urls?.[0] ? (
          <img src={proof.photo_urls[0]} className="w-full h-full object-cover opacity-50" alt="" />
        ) : (
          <div className="w-full h-full bg-gradient-to-b from-purple-900/20 to-transparent" />
        )}
        <div className="absolute inset-0 bg-gradient-to-t from-[#05000a] to-transparent" />
        <div className="absolute bottom-6 left-8 right-8 max-w-2xl mx-auto">
          <span className="text-amber-400 flex items-center gap-1 text-xs font-black italic">
            <Star size={14} fill="currentColor" /> +{proof.xp_reward} XP
          </span>
          <h1 className="text-4xl md:text-6xl font-black text-white uppercase italic">{proof.quest_title}</h1>
        </div>
      </div>

      <div className="max-w-2xl mx-auto px-6 pb-40">
        <div className="flex items-center justify-between py-8 border-b border-white/5">
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 rounded-2xl border bg-white/5 border-white/10 flex items-center justify-center">
              <UserIcon size={20} className="text-purple-500" />
            </div>
            <div>
              <p className="text-lg font-bold text-white">{proof.username}</p>
              <p className="text-[10px] text-slate-500 font-black uppercase mt-1.5">Verified Achievement</p>
            </div>
          </div>
          <div className="text-right">
            <div className="text-emerald-400 flex items-center gap-1 font-black text-[10px] uppercase">
              <CheckCircle2 size={14} /> {proof.status}
            </div>
            <div className="text-slate-500 text-[10px] font-bold mt-1">{proof.beliefs_count} Beliefs</div>
          </div>
        </div>

        {proof.proof_text && <p className="mt-12 text-2xl font-medium text-slate-200 italic">“{proof.proof_text}”</p>}

        {proof.voice_urls && proof.voice_urls[0] && (
          <CyberAudioPlayer src={proof.voice_urls[0]} />
        )}

        <div className="grid grid-cols-1 gap-6 mt-12">
          {proof.photo_urls?.map((url, i) => (
            <img key={i} src={url} alt="" className={`rounded-[2rem] border w-full shadow-2xl transition-all duration-700 ${proof.is_believed ? 'border-purple-500/50' : 'border-white/10'
              }`} />
          ))}
        </div>

        <div className={`mt-20 py-12 border rounded-[3rem] flex flex-col items-center gap-6 transition-all ${proof.is_believed ? 'bg-purple-500/10 border-purple-500/30' : 'bg-white/[0.02] border-white/5'
          }`}>
          <BelieveButton
            proofUlid={proof.ulid}
            isBelieved={proof.is_believed}
            count={proof.beliefs_count}
            onToggle={(s, c) => onUpdate(proof.ulid, { is_believed: s, beliefs_count: c })}
          />
        </div>
      </div>
    </div>
  );
};

const Feed: React.FC = () => {
  const [feed, setFeed] = useState<ProofDetailsResponse[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [selectedUlid, setSelectedUlid] = useState<string | null>(null);

  const fetchFeed = useCallback(async () => {
    setIsLoading(true);
    const res = await commands.getProofFeed(20, 0);
    if (res.status === 'ok') setFeed(res.data.items || []);
    setIsLoading(false);
  }, []);

  const updateProof = useCallback((ulid: string, patch: Partial<ProofDetailsResponse>) => {
    setFeed(prev => prev.map(p => (p.ulid === ulid ? { ...p, ...patch } : p)));
  }, []);

  useEffect(() => { fetchFeed(); }, [fetchFeed]);

  const selectedProof = feed.find(p => p.ulid === selectedUlid);

  return (
    <Layout title="Community Feed" isOverlayActive={!!selectedUlid}>
      <header className="mb-10 flex justify-between items-end">
        <div>
          <div className="flex items-center gap-2 text-purple-500 mb-2">
            <Sparkles size={16} />
            <span className="text-[10px] font-black uppercase tracking-[0.4em]">Activity</span>
          </div>
          <h1 className="text-5xl font-black italic uppercase text-white leading-none">Victories</h1>
        </div>
        <button onClick={fetchFeed} className={isLoading ? 'animate-spin text-purple-500' : 'text-slate-400'}>
          <RefreshCcw size={24} />
        </button>
      </header>

      {isLoading ? (
        <div className="space-y-6 animate-pulse">
          <div className="h-72 bg-white/5 rounded-[2.5rem]" />
          <div className="h-72 bg-white/5 rounded-[2.5rem]" />
        </div>
      ) : (
        feed.map(entry => (
          <div
            key={entry.ulid}
            onClick={() => setSelectedUlid(entry.ulid)}
            className="mb-6 border rounded-[2.5rem] bg-[#11051a] border-white/5 p-7 cursor-pointer hover:border-white/10 transition-colors"
          >
            <h3 className="text-2xl font-black uppercase italic text-white mb-4">{entry.quest_title}</h3>
            {entry.photo_urls?.[0] && (
              <div className="aspect-video rounded-2xl overflow-hidden border border-white/5 mb-6">
                <img src={entry.photo_urls[0]} className="w-full h-full object-cover" alt="" />
              </div>
            )}
            <div className="flex justify-between items-center border-t border-white/5 pt-4">
              <span className="text-[10px] text-slate-500 font-bold uppercase">
                {new Date(entry.created_at).toLocaleDateString()}
              </span>
              <BelieveButton
                proofUlid={entry.ulid}
                isBelieved={entry.is_believed}
                count={entry.beliefs_count}
                variant="compact"
                onToggle={(s, c) => updateProof(entry.ulid, { is_believed: s, beliefs_count: c })}
              />
            </div>
          </div>
        ))
      )}

      {selectedProof && (
        <ProofDetailView
          proof={selectedProof}
          onBack={() => setSelectedUlid(null)}
          onUpdate={updateProof}
        />
      )}
    </Layout>
  );
};

export default Feed;
