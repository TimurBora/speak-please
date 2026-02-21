import React, { useState, useRef, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { X, Mic, Square, Trash2, Play, ShieldCheck, Plus, Sparkles, AlertTriangle } from 'lucide-react';
import { useTaskStore } from '../stores/taskStore';
import { QuestDto } from '../bindings';
import { startRecording, stopRecording } from "tauri-plugin-mic-recorder-api";
import { convertFileSrc } from '@tauri-apps/api/core';
import { readFile } from '@tauri-apps/plugin-fs';

const useTauriAudioRecorder = () => {
  const [isRecording, setIsRecording] = useState(false);
  const [recordingTime, setRecordingTime] = useState(0);
  const [audioResult, setAudioResult] = useState<{ url: string; uint8Array: Uint8Array } | null>(null);
  const [permissionError, setPermissionError] = useState<string | null>(null);
  const timerInterval = useRef<number | null>(null);

  const start = async () => {
    try {
      await startRecording();
      setIsRecording(true);
      setRecordingTime(0);
      setAudioResult(null);

      timerInterval.current = window.setInterval(() => {
        setRecordingTime((prev) => prev + 1);
      }, 1000);
    } catch (err) {
      console.error('[Audio] Start error:', err);
      setPermissionError('Microphone access failed');
    }
  };

  const stop = async () => {
    try {
      const filePath = await stopRecording();
      if (timerInterval.current) clearInterval(timerInterval.current);
      setIsRecording(false);
      console.log("РЕАЛЬНЫЙ ПУТЬ К ФАЙЛУ:", filePath);

      if (filePath) {
        const uint8Array = await readFile(filePath);

        const assetUrl = convertFileSrc(filePath);

        setAudioResult({
          url: assetUrl,
          uint8Array
        });
      }
    } catch (err) {
      console.error('[Audio] Stop error:', err);
      setIsRecording(false);
    }
  };

  const clear = () => setAudioResult(null);

  return { isRecording, recordingTime, audioResult, permissionError, start, stop, clear };
};

const fileToUint8Array = async (file: File | Blob): Promise<Uint8Array> => {
  const arrayBuffer = await file.arrayBuffer();
  return new Uint8Array(arrayBuffer);
};

interface ProofSheetProps {
  task: QuestDto | null;
  isOpen: boolean;
  onClose: () => void;
  onSubmit: () => void;
}

const ProofSheet: React.FC<ProofSheetProps> = ({ task, isOpen, onClose, onSubmit }) => {
  const [text, setText] = useState('');
  const [images, setImages] = useState<{ file: File; preview: string }[]>([]);
  const [audioNotes, setAudioNotes] = useState<{ url: string; data: Uint8Array }[]>([]);

  const { submitProof, isLoading: isSubmitting, error } = useTaskStore();

  const {
    isRecording,
    recordingTime,
    audioResult,
    start: startTauriRecording,
    stop: stopTauriRecording,
    clear: clearCurrentAudio
  } = useTauriAudioRecorder();

  useEffect(() => {
    if (audioResult) {
      setAudioNotes(prev => [...prev, { url: audioResult.url, data: audioResult.uint8Array }]);
      clearCurrentAudio();
    }
  }, [audioResult, clearCurrentAudio]);

  useEffect(() => {
    return () => {
      images.forEach(img => URL.revokeObjectURL(img.preview));
    };
  }, [images]);

  if (!task) return null;

  const handleImageChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(e.target.files || []);
    const newImages = files.map(file => ({
      file,
      preview: URL.createObjectURL(file)
    }));
    setImages(prev => [...prev, ...newImages]);
  };

  const removeImage = (index: number) => {
    URL.revokeObjectURL(images[index].preview);
    setImages(prev => prev.filter((_, i) => i !== index));
  };

  const removeAudio = (index: number) => {
    setAudioNotes(prev => prev.filter((_, i) => i !== index));
  };

  const resetLocalState = () => {
    setText('');
    images.forEach(img => URL.revokeObjectURL(img.preview));
    setImages([]);
    setAudioNotes([]);
  };

  const handleClose = () => {
    if (isSubmitting) return;
    resetLocalState();
    onClose();
  };

  const handleSubmit = async () => {
    if (!task) return;

    const photoBytes = await Promise.all(images.map(img => fileToUint8Array(img.file)));
    const voiceBytes = audioNotes.map(a => a.data);

    const payload = {
      proof_text: text.trim() || null,
      photo_count: images.length,
      voice_count: audioNotes.length,
    };

    const result = await submitProof(task.ulid, payload, photoBytes, voiceBytes);

    if (result) {
      resetLocalState();
      onSubmit();
    }
  };

  const formatTime = (seconds: number) => {
    const m = Math.floor(seconds / 60);
    const s = seconds % 60;
    return `${m}:${s < 10 ? '0' : ''}${s}`;
  };

  return (
    <AnimatePresence>
      {isOpen && (
        <>
          <motion.div
            initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}
            onClick={handleClose}
            className="fixed inset-0 bg-[#05000a]/80 z-[60] backdrop-blur-xl"
          />
          <motion.div
            initial={{ y: "100%" }} animate={{ y: 0 }} exit={{ y: "100%" }}
            transition={{ type: "spring", damping: 25, stiffness: 200 }}
            className="fixed bottom-0 left-0 right-0 z-[70] bg-[#0d0018] border-t border-purple-500/20 rounded-t-[40px] p-6 pb-10 shadow-[0_-20px_50px_rgba(0,0,0,0.5)] max-h-[92vh] overflow-y-auto scrollbar-hide"
          >
            <div className="w-12 h-1.5 bg-purple-900/40 rounded-full mx-auto mb-6" />

            <div className="flex justify-between items-start mb-8">
              <div className="flex flex-col gap-1">
                <div className="flex items-center gap-2">
                  <Sparkles size={14} className="text-purple-400 animate-pulse" />
                  <span className="text-purple-400 text-[10px] font-black uppercase tracking-[0.3em]">Quest Completion</span>
                </div>
                <h2 className="text-2xl font-extrabold text-white tracking-tight">{task.title}</h2>
              </div>
              <button
                onClick={handleClose}
                className="p-2.5 bg-purple-950/40 border border-white/5 rounded-2xl text-slate-400 hover:text-white transition-colors"
              >
                <X size={20} />
              </button>
            </div>

            {error && (
              <div className="mb-6 bg-rose-500/10 border border-rose-500/20 rounded-2xl p-4 animate-in fade-in slide-in-from-top-2">
                <div className="flex items-center gap-3">
                  <AlertTriangle className="w-5 h-5 text-rose-500" />
                  <div>
                    <p className="text-rose-400 text-[10px] font-black uppercase tracking-wider">Protocol Error</p>
                    <p className="text-rose-200/70 text-xs">{error}</p>
                  </div>
                </div>
              </div>
            )}

            <div className="space-y-8">
              <div className="relative">
                <textarea
                  value={text}
                  onChange={(e) => setText(e.target.value)}
                  placeholder="Describe your triumph..."
                  className="w-full bg-purple-950/20 border border-white/5 rounded-[28px] p-6 text-slate-200 focus:outline-none focus:border-purple-500/50 h-32 resize-none transition-all placeholder:text-slate-600 shadow-inner"
                />
              </div>

              <div className="grid grid-cols-2 gap-6">
                <div className="space-y-3">
                  <span className="text-[10px] font-black uppercase tracking-widest text-slate-500 ml-1">Visual Proof</span>
                  <div className="grid grid-cols-1 gap-3">
                    {images.map((img, idx) => (
                      <motion.div key={img.preview} layout initial={{ opacity: 0, scale: 0.9 }} animate={{ opacity: 1, scale: 1 }} className="relative h-24 rounded-2xl overflow-hidden group border border-white/10">
                        <img src={img.preview} alt="proof" className="w-full h-full object-cover" />
                        <button onClick={() => removeImage(idx)} className="absolute top-2 right-2 p-1.5 bg-red-500/80 backdrop-blur-md rounded-lg text-white opacity-0 group-hover:opacity-100 transition-opacity">
                          <Trash2 size={12} />
                        </button>
                      </motion.div>
                    ))}
                    <label className="h-24 flex flex-col items-center justify-center border-2 border-dashed border-purple-500/20 rounded-2xl bg-purple-950/10 cursor-pointer hover:bg-purple-500/5 hover:border-purple-500/40 transition-all text-purple-400/50 group">
                      <Plus size={24} className="group-hover:scale-110 transition-transform" />
                      <input type="file" multiple accept="image/*" onChange={handleImageChange} className="hidden" />
                    </label>
                  </div>
                </div>

                <div className="space-y-3">
                  <span className="text-[10px] font-black uppercase tracking-widest text-slate-500 ml-1">Audio Logs</span>
                  <div className="space-y-3">
                    {audioNotes.map((audio, idx) => (
                      <motion.div key={audio.url} initial={{ opacity: 0, x: 20 }} animate={{ opacity: 1, x: 0 }} className="flex items-center gap-3 bg-purple-950/20 p-2 rounded-xl border border-white/5">
                        <div
                          className="w-8 h-8 bg-purple-500/20 rounded-lg flex items-center justify-center text-purple-400 cursor-pointer"
                          onClick={() => {
                            const player = new Audio(audio.url);
                            player.play();
                          }}
                        >
                          <Play size={14} fill="currentColor" />
                        </div>
                        <span className="text-[9px] font-mono text-slate-500 truncate flex-1">LOG_{idx + 1}</span>
                        <button onClick={() => removeAudio(idx)} className="p-1.5 text-slate-600 hover:text-red-400">
                          <Trash2 size={14} />
                        </button>
                      </motion.div>
                    ))}

                    <button
                      onClick={isRecording ? stopTauriRecording : startTauriRecording}
                      className={`w-full h-24 rounded-2xl border-2 border-dashed flex flex-col items-center justify-center gap-2 transition-all ${isRecording
                        ? 'bg-red-500/10 border-red-500 text-red-500 shadow-[0_0_20px_rgba(239,68,68,0.2)]'
                        : 'bg-purple-950/10 border-purple-500/20 text-purple-400/50 hover:border-purple-500/40'
                        }`}
                    >
                      {isRecording ? (
                        <>
                          <div className="flex items-center gap-2">
                            <div className="w-2 h-2 bg-red-500 rounded-full animate-ping" />
                            <span className="font-mono text-sm font-bold">{formatTime(recordingTime)}</span>
                          </div>
                          <Square size={16} fill="currentColor" />
                        </>
                      ) : (
                        <>
                          <Mic size={24} />
                          <span className="text-[9px] font-bold uppercase tracking-tighter">Voice Rec</span>
                        </>
                      )}
                    </button>
                  </div>
                </div>
              </div>

              <div className="pt-4">
                <button
                  disabled={isSubmitting || isRecording}
                  onClick={handleSubmit}
                  className="group relative w-full h-16 rounded-2xl overflow-hidden transition-all active:scale-[0.97] disabled:opacity-40 disabled:grayscale"
                >
                  <div className={`absolute inset-0 bg-gradient-to-r ${error ? 'from-rose-600 to-rose-500' : 'from-purple-600 via-fuchsia-500 to-purple-600'} animate-gradient-x`} />

                  <div className="absolute inset-[2px] bg-[#0d0018] group-hover:bg-transparent transition-colors rounded-[14px] flex items-center justify-center gap-3">
                    <ShieldCheck className={`w-6 h-6 transition-transform duration-500 ${isSubmitting ? 'animate-spin' : 'group-hover:scale-110 text-fuchsia-400'}`} />
                    <span className="text-sm font-black uppercase tracking-[0.2em] text-white">
                      {isSubmitting ? 'Synchronizing...' : 'Complete Quest'}
                    </span>
                  </div>
                  <div className="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity blur-xl bg-fuchsia-500/30 -z-10" />
                </button>
              </div>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
};

export default ProofSheet;
