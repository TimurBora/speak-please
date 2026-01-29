import type React from "react"
import { useState } from "react"
import {
  ChevronRight,
  Clock,
  CheckCircle2,
  AlertCircle,
  Sparkles,
  Zap,
  Info,
  Target,
  Trophy
} from "lucide-react"
import type { UserQuestStatusResponse, QuestStatus, Complexity } from "../bindings"

interface DailyItemProps {
  task: UserQuestStatusResponse
  onClick: (id: string) => void
}

const complexityConfig: Record<Complexity, { bg: string; text: string; border: string; label: string }> = {
  easy: {
    bg: "bg-emerald-500/20",
    text: "text-emerald-400",
    border: "border-emerald-500/30",
    label: "Easy",
  },
  medium: {
    bg: "bg-amber-500/20",
    text: "text-amber-400",
    border: "border-amber-500/30",
    label: "Medium",
  },
  hard: {
    bg: "bg-rose-500/20",
    text: "text-rose-400",
    border: "border-rose-500/30",
    label: "Hard",
  },
}

const DailyItem: React.FC<DailyItemProps> = ({ task, onClick }) => {
  const [isExpanded, setIsExpanded] = useState(false)
  const { status, quest, current_value } = task
  const complexity = complexityConfig[quest.complexity]
  const progress = quest.target_value > 0 ? (current_value / quest.target_value) * 100 : 0

  const statusConfig: Record<
    QuestStatus,
    {
      icon: React.ReactNode
      container: string
      titleColor: string
      statusText: string
      statusColor: string
      glow?: string
    }
  > = {
    IN_PROGRESS: {
      icon: (
        <div className="relative w-10 h-10 rounded-xl bg-gradient-to-br from-fuchsia-500/20 to-purple-500/20 border border-fuchsia-500/40 flex items-center justify-center">
          <Zap size={18} className="text-fuchsia-400" fill="currentColor" />
          <div className="absolute inset-0 rounded-xl border border-fuchsia-400/20 animate-ping" />
        </div>
      ),
      container: "bg-gradient-to-r from-fuchsia-950/30 to-purple-950/30 border-fuchsia-500/30 shadow-[0_0_30px_rgba(217,70,239,0.1)]",
      titleColor: "text-white",
      statusText: "In Progress",
      statusColor: "text-fuchsia-400",
      glow: "absolute inset-0 bg-gradient-to-r from-fuchsia-500/5 to-transparent rounded-2xl",
    },
    IN_PENDING: {
      icon: (
        <div className="w-10 h-10 rounded-xl bg-cyan-500/10 border border-cyan-500/20 flex items-center justify-center">
          <Clock className="w-5 h-5 text-cyan-400 animate-pulse" />
        </div>
      ),
      container: "bg-cyan-950/20 border-cyan-500/15 opacity-80",
      titleColor: "text-slate-300",
      statusText: "Pending",
      statusColor: "text-cyan-400/70",
    },
    COMPLETED: {
      icon: (
        <div className="w-10 h-10 rounded-xl bg-emerald-500/15 border border-emerald-500/25 flex items-center justify-center">
          <CheckCircle2 className="w-5 h-5 text-emerald-400" />
        </div>
      ),
      container: "bg-emerald-950/15 border-emerald-500/15 opacity-60",
      titleColor: "text-slate-500 line-through",
      statusText: "Done",
      statusColor: "text-emerald-400/60",
    },
    FAILED: {
      icon: (
        <div className="w-10 h-10 rounded-xl bg-rose-500/15 border border-rose-500/25 flex items-center justify-center">
          <AlertCircle className="w-5 h-5 text-rose-400" />
        </div>
      ),
      container: "bg-rose-950/20 border-rose-500/20",
      titleColor: "text-rose-200/90",
      statusText: "Failed",
      statusColor: "text-rose-400/70",
    },
  }

  const config = statusConfig[status]
  const isClickable = status !== "COMPLETED" && status !== "IN_PENDING"

  return (
    <div className={`relative w-full group flex flex-col mb-4 rounded-2xl border backdrop-blur-sm transition-all duration-300 overflow-hidden ${config.container}`}>
      {config.glow && <div className={config.glow} />}

      <div
        className="relative flex items-center justify-between p-4 cursor-pointer"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <div className="flex items-center gap-4 flex-1">
          {config.icon}
          <div className="flex flex-col gap-1">
            <h3 className={`font-bold text-[15px] tracking-tight transition-colors ${config.titleColor}`}>
              {quest.title}
            </h3>
            <div className="flex items-center gap-2">
              <span className={`text-[9px] font-black uppercase tracking-wider px-2 py-0.5 rounded-lg border ${complexity.bg} ${complexity.text} ${complexity.border}`}>
                {complexity.label}
              </span>
              <span className={`text-[10px] font-bold uppercase tracking-wider ${config.statusColor}`}>
                {config.statusText}
              </span>
            </div>
          </div>
        </div>

        <div className="flex items-center gap-3">
          <div className="flex items-center gap-1.5 bg-gradient-to-r from-purple-500/15 to-fuchsia-500/15 px-3 py-1.5 rounded-xl border border-purple-500/20">
            <Sparkles size={12} className="text-fuchsia-400" />
            <span className="text-xs font-black text-purple-200">+{quest.xp_reward}</span>
          </div>
          <button
            onClick={(e) => { e.stopPropagation(); setIsExpanded(!isExpanded); }}
            className={`p-1 rounded-lg hover:bg-white/5 transition-colors ${isExpanded ? 'text-fuchsia-400' : 'text-slate-500'}`}
          >
            <Info size={18} />
          </button>
        </div>
      </div>

      {isExpanded && (
        <div className="px-4 pb-4 pt-0 animate-in fade-in slide-in-from-top-2 duration-300 z-10">
          <div className="h-px bg-white/5 mb-3" />

          {quest.description && (
            <p className="text-sm text-slate-400 leading-relaxed mb-4">
              {quest.description}
            </p>
          )}

          <div className="grid grid-cols-2 gap-3 mb-4">
            <div className="bg-black/40 rounded-xl p-3 border border-white/5">
              <div className="flex items-center gap-2 mb-1">
                <Target size={14} className="text-purple-400" />
                <span className="text-[10px] uppercase text-slate-500 font-bold">Objective</span>
              </div>
              <p className="text-xs text-slate-200 capitalize">{quest.validation_type.replace(/_/g, ' ')}</p>
            </div>

            <div className="bg-black/40 rounded-xl p-3 border border-white/5">
              <div className="flex items-center gap-2 mb-1">
                <Trophy size={14} className="text-fuchsia-400" />
                <span className="text-[10px] uppercase text-slate-500 font-bold">Reward</span>
              </div>
              <p className="text-xs text-slate-200">{quest.xp_reward} Experience Points</p>
            </div>
          </div>

          <div className="bg-black/20 rounded-xl p-3 border border-white/5">
            <div className="flex justify-between items-end mb-2">
              <span className="text-[10px] uppercase text-slate-500 font-bold">Task Progress</span>
              <span className="text-xs font-mono font-bold text-fuchsia-300">{current_value} / {quest.target_value}</span>
            </div>
            <div className="w-full h-2 bg-black/40 rounded-full overflow-hidden border border-white/5">
              <div
                className="h-full bg-gradient-to-r from-purple-500 to-fuchsia-500 transition-all duration-700"
                style={{ width: `${progress}%` }}
              />
            </div>
          </div>

          {isClickable && (
            <button
              onClick={() => onClick(quest.ulid)}
              className="w-full mt-4 py-3 bg-purple-600/20 hover:bg-purple-600/30 border border-purple-500/40 rounded-xl text-xs font-black uppercase tracking-widest text-white transition-all active:scale-[0.98] flex items-center justify-center gap-2"
            >
              Update Progress <ChevronRight size={14} />
            </button>
          )}
        </div>
      )}
    </div>
  )
}

const DailyList: React.FC<{ tasks: UserQuestStatusResponse[]; onTaskClick: (id: string) => void }> = ({
  tasks,
  onTaskClick,
}) => {
  const sortedTasks = [...tasks].sort((a, b) => {
    const priority: Record<QuestStatus, number> = {
      IN_PROGRESS: 0,
      FAILED: 1,
      IN_PENDING: 2,
      COMPLETED: 3,
    }
    return priority[a.status] - priority[b.status]
  })

  const completedCount = tasks.filter((t) => t.status === "COMPLETED").length
  const totalXP = tasks.reduce((acc, t) => t.status === "COMPLETED" ? acc + t.quest.xp_reward : acc, 0)

  return (
    <div className="w-full">
      {sortedTasks.map((t) => (
        <DailyItem key={t.quest.ulid} task={t} onClick={onTaskClick} />
      ))}

      {tasks.length > 0 && (
        <div className="mt-8 p-6 rounded-3xl bg-gradient-to-b from-purple-500/5 to-transparent border-t border-purple-500/10">
          <div className="flex justify-between items-center mb-4">
            <div className="flex flex-col">
              <span className="text-[10px] font-black uppercase tracking-[0.2em] text-purple-500">Daily Progress</span>
              <span className="text-lg font-black text-white">{Math.round((completedCount / tasks.length) * 100)}% Complete</span>
            </div>
            <div className="text-right">
              <span className="text-[10px] font-black uppercase tracking-[0.2em] text-fuchsia-500">Earned Today</span>
              <div className="flex items-center justify-end gap-1 text-lg font-black text-white">
                <Sparkles size={16} className="text-fuchsia-400" />
                {totalXP} XP
              </div>
            </div>
          </div>

          <div className="w-full h-2 bg-purple-950/40 rounded-full overflow-hidden border border-white/5">
            <div
              className="h-full bg-gradient-to-r from-purple-600 via-fuchsia-500 to-purple-600 bg-[length:200%_100%] animate-gradient-x rounded-full transition-all duration-1000"
              style={{ width: `${(completedCount / tasks.length) * 100}%` }}
            />
          </div>

          <p className="text-center text-[10px] text-slate-500 mt-4 font-bold uppercase tracking-widest">
            {completedCount} of {tasks.length} quests finished
          </p>
        </div>
      )}
    </div>
  )
}

export default DailyList
