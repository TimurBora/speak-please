import React from 'react';
import { List } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

interface TopAppBarProps {
  title: string;
  onMenuClick?: () => void;
}

const TopAppBar: React.FC<TopAppBarProps> = ({ title, onMenuClick }) => {
  const navigate = useNavigate();

  return (
    <header className="fixed top-0 left-0 right-0 z-50 w-full h-16 bg-[#0f111a]/80 backdrop-blur-md flex items-center justify-between px-4">
      <div className="flex items-center gap-3">

        <div className="flex items-center">
          <button onClick={onMenuClick} className="p-2 hover:bg-[#161925] rounded-full transition-colors text-slate-400">
            <List className="w-5 h-5" />
          </button>
        </div>


        <h1 className="text-lg font-medium tracking-tight text-slate-200">
          {title}
        </h1>
      </div>

    </header>
  );
};

export default TopAppBar;
