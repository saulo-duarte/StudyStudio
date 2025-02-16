import React from 'react';
import { invoke } from '@tauri-apps/api/core';
import { X, Minus, Square } from 'lucide-react';
import { Button } from '@/components/ui/button';

const WindowControls: React.FC = () => {
  const handleMinimize = async () => {
    try {
      await invoke('minimize_window');
    } catch (error) {
    }
  };

  const handleMaximize = async () => {
    try {
      await invoke('maximize_window');
    } catch (error) {
    }
  };

  const handleClose = async () => {
    try {
      await invoke('close_window');
    } catch (error) {
    }
  };

  return (
    <div className='flex items'>
      <Button 
        onClick={handleMinimize} 
        title="Minimaze" 
        variant="ghost"
        >
        <Minus size={20} />
      </Button>
      <Button 
        onClick={handleMaximize} 
        title="Maximaze" 
        variant="ghost"
      >
        <Square size={20} />
      </Button>
      <Button 
        onClick={handleClose} 
        title="Close"
        variant="ghost" 
        className='hover:bg-destructive'>
        <X size={20} />
      </Button>
    </div>
  );
};

export default WindowControls;
