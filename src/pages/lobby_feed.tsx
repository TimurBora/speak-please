import React, { useState, useEffect, useCallback } from 'react';
import { RefreshCcw, Plus, Sparkles, Loader2, ChevronLeft } from 'lucide-react';
import Layout from '../components/Layout';
import LobbyItem from '../components/LobbyItem';
import CreateLobbySheet from '../components/CreateLobbySheet';
import LobbyChat from '../components/LobbyChat'; // Новый компонент
import { commands, LobbyFeedItem, LobbyDto } from '../bindings';

const Lobbies: React.FC = () => {
  const [isCreateOpen, setIsCreateOpen] = useState(false);
  const [items, setItems] = useState<LobbyFeedItem[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  const [activeLobby, setActiveLobby] = useState<LobbyDto | null>(null);

  const fetchLobbies = useCallback(async () => {
    setIsLoading(true);
    try {
      const response = await commands.getAllLobbies();
      if (response.status === 'ok') {
        setItems(response.data.items);
      }
    } catch (err) {
      console.error("Fetch lobbies failed:", err);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchLobbies();
  }, [fetchLobbies]);

  const handleAction = async (lobby: LobbyDto, isMember: boolean) => {
    if (isMember) {
      setActiveLobby(lobby);
    } else {
      try {
        const res = await commands.joinLobby(lobby.ulid);
        if (res.status === 'ok') {
          await fetchLobbies();
          setActiveLobby(lobby);
        }
      } catch (err) {
        console.error("Join failed", err);
      }
    }
  };

  if (activeLobby) {
    return (
      <Layout title={activeLobby.name} showBottomBar={false}>
        <div className="flex flex-col h-full">
          <button
            onClick={() => setActiveLobby(null)}
            className="flex items-center gap-2 text-purple-400 font-black uppercase text-[10px] tracking-widest mb-6 hover:text-white transition-colors"
          >
            <ChevronLeft size={14} /> Back to Grid
          </button>
          <LobbyChat lobby={activeLobby} onBack={() => setActiveLobby(null)} />
        </div>
      </Layout>
    );
  }

  return (
    <Layout title="Lobbies" isOverlayActive={isCreateOpen}>
      <header className="mb-8 flex justify-between items-end px-1">
        <div className="flex flex-col gap-1">
          <div className="flex items-center gap-2 text-purple-500">
            <Sparkles size={14} />
            <span className="text-[10px] font-black uppercase tracking-[0.3em]">Neural Network</span>
          </div>
          <h1 className="text-3xl font-black text-white tracking-tight italic uppercase">
            All Lobbies
          </h1>
        </div>

        <div className="flex gap-2">
          <button
            onClick={fetchLobbies}
            className="p-3 bg-white/5 border border-white/10 rounded-2xl text-slate-400 active:scale-90 transition-all hover:bg-white/10"
          >
            <RefreshCcw size={20} className={isLoading ? 'animate-spin' : ''} />
          </button>
          <button
            onClick={() => setIsCreateOpen(true)}
            className="p-3 bg-purple-600 border border-purple-400 rounded-2xl text-white shadow-[0_0_15px_rgba(168,85,247,0.3)] active:scale-90 transition-all hover:bg-purple-500"
          >
            <Plus size={20} />
          </button>
        </div>
      </header>

      {isLoading && items.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-20 opacity-50">
          <Loader2 className="animate-spin text-purple-500 mb-4" size={40} />
          <span className="text-[10px] font-black uppercase tracking-widest">Scanning Grid...</span>
        </div>
      ) : (
        <div className="space-y-4 pb-10">
          {items.map((item) => (
            <LobbyItem
              key={item.lobby.ulid}
              lobby={item.lobby}
              isMember={item.is_member}
              onAction={() => handleAction(item.lobby, item.is_member)}
            />
          ))}
        </div>
      )}

      <CreateLobbySheet
        isOpen={isCreateOpen}
        onClose={() => setIsCreateOpen(false)}
        onCreated={() => {
          setIsCreateOpen(false);
          fetchLobbies();
        }}
      />
    </Layout>
  );
};

export default Lobbies;
