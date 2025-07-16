import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

const createTrackingStore = () => {
  const { subscribe, set, update } = writable({
    isTracking: false,
    currentProject: null,
    currentWindow: null,
    sessionDuration: 0,
    idleTime: 0,
    lastActivity: null,
    isLoading: false,
    error: null
  });

  let pollingInterval = null;

  return {
    subscribe,
    
    async initialize() {
      try {
        update(state => ({ ...state, isLoading: true }));
        const status = await invoke('get_tracking_status');
        set({
          isTracking: status.is_tracking,
          currentProject: status.current_project,
          currentWindow: status.current_window,
          sessionDuration: status.session_duration,
          idleTime: status.idle_time,
          lastActivity: status.last_activity,
          isLoading: false,
          error: null
        });
      } catch (error) {
        console.error('Failed to initialize tracking store:', error);
        update(state => ({ ...state, isLoading: false, error: error.message }));
      }
    },

    async startTracking() {
      try {
        update(state => ({ ...state, isLoading: true }));
        await invoke('start_tracking');
        await this.refreshStatus();
      } catch (error) {
        console.error('Failed to start tracking:', error);
        update(state => ({ ...state, isLoading: false, error: error.message }));
      }
    },

    async stopTracking() {
      try {
        update(state => ({ ...state, isLoading: true }));
        await invoke('stop_tracking');
        await this.refreshStatus();
      } catch (error) {
        console.error('Failed to stop tracking:', error);
        update(state => ({ ...state, isLoading: false, error: error.message }));
      }
    },

    async refreshStatus() {
      try {
        const status = await invoke('get_tracking_status');
        update(state => ({
          ...state,
          isTracking: status.is_tracking,
          currentProject: status.current_project,
          currentWindow: status.current_window,
          sessionDuration: status.session_duration,
          idleTime: status.idle_time,
          lastActivity: status.last_activity,
          isLoading: false,
          error: null
        }));
      } catch (error) {
        console.error('Failed to refresh tracking status:', error);
        update(state => ({ ...state, error: error.message }));
      }
    },

    startPolling() {
      if (pollingInterval) {
        clearInterval(pollingInterval);
      }
      
      pollingInterval = setInterval(() => {
        this.refreshStatus();
      }, 2000); // Poll every 2 seconds
    },

    stopPolling() {
      if (pollingInterval) {
        clearInterval(pollingInterval);
        pollingInterval = null;
      }
    },

    clearError() {
      update(state => ({ ...state, error: null }));
    }
  };
};

export const trackingStore = createTrackingStore();