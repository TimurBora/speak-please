import React from 'react';
import {
  ChevronRight,
  Clock,
  CheckCircle2,
  AlertCircle,
  Sparkles
} from 'lucide-react';

export type TaskStatus = 'todo' | 'pending' | 'completed' | 'rejected';

interface Task {
  id: string;
  title: string;
  points: number;
  status: TaskStatus;
}

interface DailyItemProps {
  id: string;
  title: string;
  points: number;
  status: TaskStatus;
  onClick: (id: string) => void;
}

const DailyItem: React.FC<DailyItemProps> = ({ id, title, points, status, onClick }) => {
  const statusConfig = {
    todo: {
      icon: (
        <div className="w-5 h-5 rounded-full border-2 border-purple-500/30 
          group-hover:border-purple-400 group-hover:shadow-[0_0_15px_rgba(168,85,247,0.6)] 
          group-hover:bg-purple-500/20 transition-all duration-500"
        />
      ),
      container: `bg-gradient-to-br from-[#1e0034] to-[#12001d] 
        border-purple-500/10 hover:border-purple-500/40 
        cursor-pointer hover:scale-[1.01] hover:shadow-purple-500/10`,
      titleColor: "text-slate-200 group-hover:text-white"
    },
    pending: {
      icon: <Clock className="w-5 h-5 text-cyan-400 animate-spin-slow" />,
      container: "bg-cyan-500/5 border-cyan-500/20 cursor-default opacity-90",
      titleColor: "text-slate-300"
    },
    completed: {
      icon: <CheckCircle2 className="w-5 h-5 text-emerald-400 shadow-[0_0_10px_rgba(52,211,153,0.3)]" />,
      container: "bg-emerald-500/5 border-emerald-500/10 cursor-default opacity-60",
      titleColor: "text-slate-500 italic line-through"
    },
    rejected: {
      icon: <AlertCircle className="w-5 h-5 text-rose-500" />,
      container: "bg-rose-500/5 border-rose-500/20 cursor-pointer hover:bg-rose-500/10",
      titleColor: "text-rose-100/80"
    }
  };

  const config = statusConfig[status];

  return (
    <div
      onClick={() => (status === 'todo' || status === 'rejected') && onClick(id)}
      className={`w-full relative group flex items-center justify-between p-5 mb-4 rounded-2xl border transition-all duration-500 ${config.container}`}
    >
      <div className="flex items-center gap-4 flex-1">
        <div className="flex-shrink-0">{config.icon}</div>
        <div className="flex flex-col gap-0.5">
          <h3 className={`font-semibold text-[15px] tracking-tight transition-colors ${config.titleColor}`}>
            {title}
          </h3>
          <div className="flex items-center gap-2">
            <span className={`text-[10px] font-bold px-2 py-0.5 rounded-md border ${status === 'completed' ? 'border-emerald-500/20 text-emerald-500/50' :
                status === 'pending' ? 'border-cyan-500/20 text-cyan-400' :
                  status === 'rejected' ? 'border-rose-500/20 text-rose-400' :
                    'border-purple-500/20 text-purple-400'
              }`}>
              {status.toUpperCase()}
            </span>
          </div>
        </div>
      </div>

      <div className="flex items-center gap-3">
        <div className="flex items-center gap-1.5 bg-black/20 px-3 py-1.5 rounded-xl border border-white/5">
          <Sparkles size={12} className={status === 'completed' ? 'text-slate-600' : 'text-purple-400'} />
          <span className={`text-xs font-black ${status === 'completed' ? 'text-slate-600' : 'text-white'}`}>
            {points}
          </span>
        </div>
        {(status === 'todo' || status === 'rejected') && (
          <ChevronRight className="w-5 h-5 text-slate-700 group-hover:text-purple-400 transform group-hover:translate-x-1 transition-all" />
        )}
      </div>
    </div>
  );
};

interface DailyListProps {
  tasks: Task[];
  onTaskClick: (id: string) => void;
}

const DailyList: React.FC<DailyListProps> = ({ tasks, onTaskClick }) => {
  return (
    <div className="w-full">
      {tasks.map(task => (
        <DailyItem
          key={task.id}
          {...task}
          onClick={onTaskClick}
        />
      ))}
    </div>
  );
}

export default DailyList;
