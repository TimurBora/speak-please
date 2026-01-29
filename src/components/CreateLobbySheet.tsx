import React, { useState } from 'react';
import { X, Sparkles, Hash, AlignLeft, Send } from 'lucide-react';
import { commands } from '../bindings';

interface CreateLobbySheetProps {
  isOpen: boolean;
  onClose: () => void;
  onCreated: () => void;
}

const CreateLobbySheet: React.FC<CreateLobbySheetProps> = ({ isOpen, onClose, onCreated }) => {
  const [name, setName] = useState('');
  const [topic, setTopic] = useState('');
  const [description, setDescription] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async () => {
    if (!name || !topic) return;
    setIsSubmitting(true);
    try {
      await commands.createLobby(name, topic, description);
      setName(''); setTopic(''); setDescription('');
      onCreated();
      onClose();
    } catch (err) {
      console.error("Lobby creation failed:", err);
    } finally {
      setIsSubmitting(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-[200] flex items-center justify-center px-4">
      <div className="absolute inset-0 bg-[#05000a]/90 backdrop-blur-sm" onClick={onClose} />
      <div className="relative w-full max-w-lg bg-[#11051a] border border-purple-500/30 rounded-[2.5rem] p-8 animate-in slide-in-from-bottom duration-500">
        <div className="flex justify-between items-start mb-8">
          <div>
            <div className="flex items-center gap-2 text-purple-500 mb-1">
              <Sparkles size={14} />
              <span className="text-[10px] font-black uppercase tracking-[0.2em]">New Space</span>
            </div>
            <h2 className="text-3xl font-black italic uppercase text-white">Create Lobby</h2>
          </div>
          <button onClick={onClose} className="p-2 bg-white/5 rounded-full text-slate-400"><X size={24} /></button>
        </div>

        <div className="space-y-6">
          <div className="space-y-2">
            <label className="text-[10px] font-black uppercase tracking-widest text-slate-500 ml-2">Lobby Name</label>
            <div className="relative">
              <input
                value={name} onChange={(e) => setName(e.target.value)}
                placeholder="The Void..."
                className="w-full bg-black/40 border border-white/10 rounded-2xl py-4 px-6 text-white focus:border-purple-500/50 outline-none transition-all"
              />
            </div>
          </div>

          <div className="space-y-2">
            <label className="text-[10px] font-black uppercase tracking-widest text-slate-500 ml-2">Topic</label>
            <div className="relative flex items-center">
              <Hash size={16} className="absolute left-6 text-purple-500" />
              <input
                value={topic} onChange={(e) => setTopic(e.target.value)}
                placeholder="Gaming, Tech, Art..."
                className="w-full bg-black/40 border border-white/10 rounded-2xl py-4 pl-14 pr-6 text-white focus:border-purple-500/50 outline-none transition-all"
              />
            </div>
          </div>

          <div className="space-y-2">
            <label className="text-[10px] font-black uppercase tracking-widest text-slate-500 ml-2">Description (Optional)</label>
            <textarea
              value={description} onChange={(e) => setDescription(e.target.value)}
              rows={3}
              className="w-full bg-black/40 border border-white/10 rounded-2xl py-4 px-6 text-white focus:border-purple-500/50 outline-none transition-all resize-none"
            />
          </div>

          <button
            disabled={isSubmitting || !name || !topic}
            onClick={handleSubmit}
            className="w-full py-5 bg-purple-600 border border-purple-400 rounded-2xl text-white font-black uppercase tracking-widest flex items-center justify-center gap-3 active:scale-95 disabled:opacity-50 transition-all shadow-[0_0_30px_rgba(168,85,247,0.2)]"
          >
            {isSubmitting ? 'Initializing...' : <>Manifest Lobby <Send size={18} /></>}
          </button>
        </div>
      </div>
    </div>
  );
};

export default CreateLobbySheet;
