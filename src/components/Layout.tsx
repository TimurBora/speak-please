import React, { useState } from 'react';
import TopAppBar from './TopAppBar';
import BottomAppBar from './BottomAppBar';
import NavigationDrawer from './NavigationDrawer';

interface LayoutProps {
  children: React.ReactNode;
  title?: string;
  showBottomBar?: boolean;
  isOverlayActive?: boolean;
}

const Layout: React.FC<LayoutProps> = ({
  children,
  title = "Quest",
  showBottomBar = true,
  isOverlayActive = false
}) => {
  const [isDrawerOpen, setIsDrawerOpen] = useState(false);

  return (
    <div className="relative min-h-screen text-white flex flex-col selection:bg-purple-500/30">

      <div className="fixed inset-0 z-0 bg-[#050505] bg-[radial-gradient(circle_at_50%_-20%,#1a0b2e_0%,#050505_80%)]" />

      <div className="fixed inset-0 z-0 pointer-events-none overflow-hidden">
        <div className="absolute top-[-10%] left-[-10%] w-[50%] h-[50%] bg-purple-600/10 blur-[120px] rounded-full opacity-50" />
      </div>

      <div className="relative z-10 flex flex-col min-h-screen">
        <TopAppBar
          title={title}
          onMenuClick={() => setIsDrawerOpen(true)}
        />

        <NavigationDrawer
          isOpen={isDrawerOpen}
          onClose={() => setIsDrawerOpen(false)}
        />

        <main
          className="flex-1 w-full max-w-2xl mx-auto px-4"
          style={{
            paddingTop: 'calc(var(--top-bar-height, 64px) + env(safe-area-inset-top, 25px) + 40px)',
            paddingBottom: showBottomBar
              ? 'calc(var(--bottom-bar-height, 80px) + env(safe-area-inset-bottom, 20px) + 40px)'
              : 'env(safe-area-inset-bottom, 20px)',
          }}
        >
          {children}
        </main>

        {showBottomBar && (
          <BottomAppBar isHidden={isOverlayActive || isDrawerOpen} />
        )}

        <div
          className="fixed bottom-0 left-0 right-0 z-[55] bg-[#0c0616]/90 backdrop-blur-xl border-t border-white/[0.03]"
          style={{ height: 'env(safe-area-inset-bottom)' }}
        />
      </div>
    </div>
  );
};

export default Layout;
