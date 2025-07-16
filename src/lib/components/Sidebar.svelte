<script>
  import { createEventDispatcher } from 'svelte';
  import { Home, FolderOpen, FileText, Settings, BarChart3 } from 'lucide-svelte';
  
  export let currentView = 'dashboard';
  
  const dispatch = createEventDispatcher();
  
  const menuItems = [
    { id: 'dashboard', label: 'Dashboard', icon: Home },
    { id: 'projects', label: 'Projects', icon: FolderOpen },
    { id: 'reports', label: 'Reports', icon: BarChart3 },
    { id: 'settings', label: 'Settings', icon: Settings }
  ];
  
  function handleMenuClick(id) {
    dispatch('viewChange', { view: id });
  }
</script>

<aside class="w-64 bg-white border-r border-gray-200 flex flex-col">
  <!-- Logo -->
  <div class="h-16 flex items-center px-6 border-b border-gray-200">
    <h1 class="text-xl font-bold text-primary-600">Timesheet Tracker</h1>
  </div>
  
  <!-- Navigation -->
  <nav class="flex-1 px-4 py-4">
    <ul class="space-y-1">
      {#each menuItems as item}
        <li>
          <button
            class="w-full flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors
                   {currentView === item.id 
                     ? 'bg-primary-50 text-primary-700' 
                     : 'text-gray-700 hover:bg-gray-100'}"
            on:click={() => handleMenuClick(item.id)}
          >
            <svelte:component this={item.icon} class="mr-3 h-5 w-5" />
            {item.label}
          </button>
        </li>
      {/each}
    </ul>
  </nav>
  
  <!-- Footer -->
  <div class="p-4 border-t border-gray-200">
    <p class="text-xs text-gray-500 text-center">
      Version 2.0.0
    </p>
  </div>
</aside>