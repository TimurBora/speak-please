import React, { useState, useEffect } from 'react';
import Layout from '../components/Layout';
import DailyList from '../components/DailyList';
import ProofSheet from '../components/ProofSheet';
import { useTaskStore } from '../stores/taskStore';

import { QuestDto } from '../bindings';
import { useAuthStore } from '../stores/authStore';

const Home: React.FC = () => {
  const [selectedTaskId, setSelectedTaskId] = useState<string | null>(null);
  const { tasks, isLoading, error, fetchTasks, getTaskById } = useTaskStore();
  const { userSession } = useAuthStore();

  useEffect(() => {
    fetchTasks();
  }, [fetchTasks]);

  const activeTask = selectedTaskId ? getTaskById(selectedTaskId) : null;
  const isUIOverlayActive = !!selectedTaskId;

  const username = userSession?.username;

  return (
    <Layout title={username} isOverlayActive={isUIOverlayActive}>
      <div className="mb-6 flex justify-between items-end px-1">
        <div className="flex flex-col gap-1">
          <h2 className="text-[10px] uppercase tracking-[0.3em] text-purple-500 font-black">
            Daily Quests
          </h2>
          <p className="text-2xl font-black text-white tracking-tight">
            Today's Focus
          </p>
        </div>

        {isLoading && (
          <span className="text-xs text-purple-400 animate-pulse font-bold uppercase">
            Syncing...
          </span>
        )}
      </div>

      {error && (
        <div className="bg-red-500/10 border border-red-500/20 text-red-400 p-4 rounded-2xl mb-6 text-sm flex justify-between items-center">
          <span>{error}</span>
          <button onClick={() => fetchTasks()} className="font-bold underline">Retry</button>
        </div>
      )}

      <DailyList tasks={tasks} onTaskClick={setSelectedTaskId} />

      <ProofSheet
        task={activeTask ? ({
          ulid: activeTask.quest.ulid,
          title: activeTask.quest.title,
          points: activeTask.quest.xp_reward,
          status: activeTask.status.toLowerCase()
        } as any) : null}
        isOpen={!!selectedTaskId}
        onClose={() => setSelectedTaskId(null)}
        onSubmit={() => setSelectedTaskId(null)}
      />
    </Layout>
  );
};

export default Home;
