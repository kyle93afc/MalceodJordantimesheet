<script>
  import { trackingStore } from '../stores/tracking';
  import { formatDuration } from '../utils/time';
  
  $: sessionDuration = $trackingStore.sessionDuration;
  $: isTracking = $trackingStore.isTracking;
  $: currentWindow = $trackingStore.currentWindow;
</script>

<footer class="h-8 bg-gray-800 text-white text-xs flex items-center px-4">
  <div class="flex items-center space-x-6">
    <!-- Tracking Status -->
    <div class="flex items-center space-x-2">
      <div class="w-2 h-2 rounded-full {isTracking ? 'bg-green-400' : 'bg-gray-400'}"></div>
      <span>{isTracking ? 'Tracking' : 'Not Tracking'}</span>
    </div>
    
    <!-- Session Duration -->
    {#if isTracking && sessionDuration > 0}
      <div class="flex items-center space-x-1">
        <span class="text-gray-400">Session:</span>
        <span>{formatDuration(sessionDuration)}</span>
      </div>
    {/if}
    
    <!-- Current Window -->
    {#if currentWindow}
      <div class="flex items-center space-x-1 flex-1 overflow-hidden">
        <span class="text-gray-400">Window:</span>
        <span class="truncate">{currentWindow}</span>
      </div>
    {/if}
  </div>
</footer>