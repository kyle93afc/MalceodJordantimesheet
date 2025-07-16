import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

const createSettingsStore = () => {
  const { subscribe, set, update } = writable({
    settings: {},
    isLoading: false,
    error: null
  });

  return {
    subscribe,
    
    async initialize() {
      try {
        update(state => ({ ...state, isLoading: true }));
        const settings = await invoke('get_settings');
        set({
          settings,
          isLoading: false,
          error: null
        });
      } catch (error) {
        console.error('Failed to initialize settings store:', error);
        update(state => ({ ...state, isLoading: false, error: error.message }));
      }
    },

    async getSettings() {
      try {
        const settings = await invoke('get_settings');
        update(state => ({ ...state, settings }));
        return settings;
      } catch (error) {
        console.error('Failed to get settings:', error);
        update(state => ({ ...state, error: error.message }));
        return {};
      }
    },

    async updateSettings(newSettings) {
      try {
        update(state => ({ ...state, isLoading: true }));
        await invoke('update_settings', { settings: newSettings });
        
        // Refresh settings after update
        const settings = await invoke('get_settings');
        update(state => ({ ...state, settings, isLoading: false }));
      } catch (error) {
        console.error('Failed to update settings:', error);
        update(state => ({ ...state, isLoading: false, error: error.message }));
      }
    },

    async updateSetting(key, value) {
      try {
        const currentSettings = await this.getSettings();
        const newSettings = { ...currentSettings, [key]: value };
        await this.updateSettings(newSettings);
      } catch (error) {
        console.error('Failed to update setting:', error);
        update(state => ({ ...state, error: error.message }));
      }
    },

    clearError() {
      update(state => ({ ...state, error: null }));
    }
  };
};

export const settingsStore = createSettingsStore();