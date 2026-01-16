import React, { useState, useEffect } from 'react';
import TopAppBar from '../components/TopAppBar';
import NavigationDrawer from '../components/NavigationDrawer';
import DailyList from '../components/DailyList';
import BottomAppBar from '../components/BottomAppBar';
import ProofSheet from '../components/ProofSheet';

// Импортируем сгенерированные биндинги (путь может отличаться)
import { commands } from '../bindings';

type TaskStatus = 'todo' | 'pending' | 'completed' | 'rejected';

interface Task {
  id: string;
  title: string;
  points: number;
  status: TaskStatus;
}

const Home: React.FC = () => {
  const [isDrawerOpen, setIsDrawerOpen] = useState<boolean>(false);
  const [selectedTaskId, setSelectedTaskId] = useState<string | null>(null);

  // Состояния для данных
  const [tasks, setTasks] = useState<Task[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  // --- Функция загрузки данных ---
  const fetchTasks = async () => {
    try {
      setIsLoading(true);
      // Вызываем команду Rust: get_user_quests()
      const result = await commands.getUserQuests();

      // Маппим ответ из Rust (UserQuestStatusResponse) в наш интерфейс Task
      if (result.status === "ok") {
        const mappedTasks: Task[] = result.data.map((q) => ({
          id: q.quest.ulid, // Используем ULID квеста как ID
          title: q.quest.title,
          points: q.quest.reward_points || 0,
          // Преобразуем Enum из Rust (SCREAMING_SNAKE_CASE) в UI статус
          status: mapRustStatusToUi(q.status)
        }));

        setTasks(mappedTasks);
      } else {
        setError(result.error.message);
      }
    } catch (err) {
      console.error("Failed to fetch quests:", err);
      setError("Connection error");
    } finally {
      setIsLoading(false);
    }
  };

  // Вспомогательная функция для маппинга статусов
  const mapRustStatusToUi = (status: any): TaskStatus => {
    switch (status) {
      case 'NOT_STARTED':
      case 'IN_PROGRESS': return 'todo';
      case 'IN_PENDING': return 'pending';
      case 'COMPLETED': return 'completed';
      case 'FAILED': return 'rejected';
      default: return 'todo';
    }
  };

  // --- Автоматический вызов при монтировании ---
  useEffect(() => {
    fetchTasks();
  }, []);

  // --- Логика ---
  const activeTaskForSheet = tasks.find(t => t.id === selectedTaskId) || null;
  const isUIOverlayActive = !!selectedTaskId || isDrawerOpen;

  const handleTaskSubmit = async (data: any) => {
    console.log("Данные пруфа отправлены на сервер:", data);
    // Здесь можно вызвать команду для завершения квеста, а затем обновить список:
    // await commands.completeQuest(...)
    // await fetchTasks();
    setSelectedTaskId(null);
  };

  return (
    <div className="min-h-screen bg-[#0d0018] relative overflow-hidden flex flex-col font-sans text-slate-200">

      {/* Background Blobs */}
      <div className="absolute top-[-10%] left-[-10%] w-[70%] h-[50%] bg-purple-900/15 rounded-full blur-[120px] pointer-events-none" />
      <div className="absolute bottom-[20%] right-[-10%] w-[60%] h-[40%] bg-indigo-900/10 rounded-full blur-[100px] pointer-events-none" />

      <TopAppBar
        title="Timur Borisov"
        onMenuClick={() => setIsDrawerOpen(true)}
      />

      <NavigationDrawer
        isOpen={isDrawerOpen}
        onClose={() => setIsDrawerOpen(false)}
      />

      <main className="flex-1 relative z-10 px-4 pt-24 pb-32 overflow-y-auto scrollbar-hide">
        <div className="mb-6 flex justify-between items-end px-1">
          <div className="flex flex-col gap-1">
            <h2 className="text-[10px] uppercase tracking-[0.3em] text-purple-500 font-black">
              Daily Quests
            </h2>
            <p className="text-2xl font-black text-white tracking-tight">Today's Focus</p>
          </div>
          {isLoading && <span className="text-xs text-purple-400 animate-pulse">Updating...</span>}
        </div>

        {error && (
          <div className="bg-red-500/10 border border-red-500/20 text-red-400 p-3 rounded-xl mb-4 text-sm">
            {error}. <button onClick={fetchTasks} className="underline">Retry</button>
          </div>
        )}

        <div className="space-y-1">
          <DailyList
            tasks={tasks}
            onTaskClick={(id) => setSelectedTaskId(id)}
          />
        </div>
      </main>

      <BottomAppBar isHidden={isUIOverlayActive} />

      <ProofSheet
        task={activeTaskForSheet}
        isOpen={!!selectedTaskId}
        onClose={() => setSelectedTaskId(null)}
        onSubmit={handleTaskSubmit}
      />
    </div>
  );
};

export default Home;
