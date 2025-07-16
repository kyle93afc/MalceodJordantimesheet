import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

const createProjectsStore = () => {
  const { subscribe, set, update } = writable({
    projects: [],
    timeEntries: [],
    isLoading: false,
    error: null
  });

  return {
    subscribe,
    
    async initialize() {
      try {
        update(state => ({ ...state, isLoading: true }));
        const projects = await invoke('get_projects');
        const timeEntries = await invoke('get_time_entries');
        
        set({
          projects,
          timeEntries,
          isLoading: false,
          error: null
        });
      } catch (error) {
        console.error('Failed to initialize projects store:', error);
        update(state => ({ ...state, isLoading: false, error: error.message }));
      }
    },

    async refreshProjects() {
      try {
        const projects = await invoke('get_projects');
        update(state => ({ ...state, projects }));
      } catch (error) {
        console.error('Failed to refresh projects:', error);
        update(state => ({ ...state, error: error.message }));
      }
    },

    async getTimeEntries(startDate = null, endDate = null) {
      try {
        update(state => ({ ...state, isLoading: true }));
        const timeEntries = await invoke('get_time_entries', {
          startDate,
          endDate
        });
        update(state => ({ ...state, timeEntries, isLoading: false }));
        return timeEntries;
      } catch (error) {
        console.error('Failed to get time entries:', error);
        update(state => ({ ...state, isLoading: false, error: error.message }));
        return [];
      }
    },

    async getDailySummary(date) {
      try {
        return await invoke('get_daily_summary', { date });
      } catch (error) {
        console.error('Failed to get daily summary:', error);
        update(state => ({ ...state, error: error.message }));
        return null;
      }
    },

    async getProjectStatistics(projectId) {
      try {
        return await invoke('get_project_statistics', { projectId });
      } catch (error) {
        console.error('Failed to get project statistics:', error);
        update(state => ({ ...state, error: error.message }));
        return null;
      }
    },

    async exportData(format, startDate = null, endDate = null, projectIds = null) {
      try {
        return await invoke('export_data', {
          request: {
            format,
            start_date: startDate,
            end_date: endDate,
            project_ids: projectIds
          }
        });
      } catch (error) {
        console.error('Failed to export data:', error);
        update(state => ({ ...state, error: error.message }));
        throw error;
      }
    },

    clearError() {
      update(state => ({ ...state, error: null }));
    }
  };
};

export const projectsStore = createProjectsStore();