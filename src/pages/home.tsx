
import React, { useState } from 'react';
import TopAppBar from '../components/TopAppBar';
import NavigationDrawer from '../components/NavigationDrawer';

const Home: React.FC = () => {
  const [isDrawerOpen, setIsDrawerOpen] = useState<boolean>(false);

  return (
    <div className="min-h-screen bg-[#05050a] flex p-4 font-sans text-slate-200">
      <TopAppBar title='Timur Borisov' onMenuClick={() => setIsDrawerOpen(true)}></TopAppBar>
      <NavigationDrawer
        isOpen={isDrawerOpen}
        onClose={() => setIsDrawerOpen(false)}
      />


    </div>
  );
}

export default Home;
