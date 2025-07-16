<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { appWindow } from '@tauri-apps/api/window';
  import { trackingStore } from './lib/stores/tracking';
  import { projectsStore } from './lib/stores/projects';
  import { settingsStore } from './lib/stores/settings';
  import './app.css';
  
  import Dashboard from './lib/components/Dashboard.svelte';
  import Sidebar from './lib/components/Sidebar.svelte';
  import TopBar from './lib/components/TopBar.svelte';
  import StatusBar from './lib/components/StatusBar.svelte';
  
  let currentView = 'dashboard';
  let isLoading = true;
  
  onMount(async () => {
    try {
      // Initialize stores
      await Promise.all([
        trackingStore.initialize(),
        projectsStore.initialize(),
        settingsStore.initialize()
      ]);
      
      // Start tracking status polling
      trackingStore.startPolling();
      
      // Auto-start tracking if enabled
      const settings = await settingsStore.getSettings();
      if (settings.auto_start === 'true') {
        await invoke('start_tracking');
      }
      
    } catch (error) {
      console.error('Failed to initialize app:', error);
    } finally {
      isLoading = false;
    }
  });
  
  function handleViewChange(event) {
    currentView = event.detail.view;
  }
  
  // Handle window events
  onMount(() => {
    const unlisten = appWindow.listen('tauri://close-requested', () => {
      // Hide window instead of closing
      appWindow.hide();
    });
    
    return unlisten;
  });
</script>

<main class="flex h-screen bg-gray-50">
  {#if isLoading}
    <div class="flex items-center justify-center w-full h-full">
      <div class="flex items-center space-x-4">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
        <span class="text-lg font-medium text-gray-600">Loading...</span>
      </div>
    </div>
  {:else}
    <!-- Sidebar -->
    <Sidebar {currentView} on:viewChange={handleViewChange} />
    
    <!-- Main Content -->
    <div class="flex-1 flex flex-col">
      <!-- Top Bar -->
      <TopBar />
      
      <!-- Content Area -->
      <div class="flex-1 overflow-hidden">
        {#if currentView === 'dashboard'}
          <Dashboard />
        {:else if currentView === 'projects'}
          <div class="p-6">
            <h1 class="text-2xl font-bold text-gray-900">Projects</h1>
            <p class="text-gray-600 mt-2">Manage your projects and time tracking rules.</p>
          </div>
        {:else if currentView === 'reports'}
          <div class="p-6">
            <h1 class="text-2xl font-bold text-gray-900">Reports</h1>
            <p class="text-gray-600 mt-2">View detailed reports and analytics.</p>
          </div>
        {:else if currentView === 'settings'}
          <div class="p-6">
            <h1 class="text-2xl font-bold text-gray-900">Settings</h1>
            <p class="text-gray-600 mt-2">Configure your timesheet tracker preferences.</p>
          </div>
        {/if}
      </div>
      
      <!-- Status Bar -->
      <StatusBar />
    </div>
  {/if}
</main>