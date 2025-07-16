<script>
  import { trackingStore } from '../stores/tracking';
  import { Play, Pause, User } from 'lucide-svelte';
  
  $: isTracking = $trackingStore.isTracking;
  $: currentProject = $trackingStore.currentProject;
  
  async function toggleTracking() {
    if (isTracking) {
      await trackingStore.stopTracking();
    } else {
      await trackingStore.startTracking();
    }
  }
</script>

<header class="h-16 bg-white border-b border-gray-200 px-6 flex items-center justify-between">
  <!-- Current Project Info -->
  <div class="flex items-center space-x-4">
    <div>
      <p class="text-sm text-gray-500">Current Project</p>
      <p class="font-semibold text-gray-900">
        {currentProject ? currentProject.name : 'No project selected'}
      </p>
    </div>
  </div>
  
  <!-- Actions -->
  <div class="flex items-center space-x-4">
    <!-- Track/Pause Button -->
    <button
      class="flex items-center space-x-2 px-4 py-2 rounded-md font-medium transition-colors
             {isTracking 
               ? 'bg-red-100 text-red-700 hover:bg-red-200' 
               : 'bg-green-100 text-green-700 hover:bg-green-200'}"
      on:click={toggleTracking}
    >
      {#if isTracking}
        <Pause class="h-5 w-5" />
        <span>Pause Tracking</span>
      {:else}
        <Play class="h-5 w-5" />
        <span>Start Tracking</span>
      {/if}
    </button>
    
    <!-- User Menu -->
    <button class="p-2 rounded-full hover:bg-gray-100">
      <User class="h-5 w-5 text-gray-600" />
    </button>
  </div>
</header>