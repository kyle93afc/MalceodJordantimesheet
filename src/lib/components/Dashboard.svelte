<script>
  import { onMount } from 'svelte';
  import { trackingStore } from '../stores/tracking';
  import { projectsStore } from '../stores/projects';
  import { formatDuration } from '../utils/time';
  import { Clock, TrendingUp, Calendar, Activity } from 'lucide-svelte';
  
  let todaySummary = null;
  let recentEntries = [];
  
  $: currentProject = $trackingStore.currentProject;
  $: sessionDuration = $trackingStore.sessionDuration;
  $: isTracking = $trackingStore.isTracking;
  
  onMount(async () => {
    const today = new Date().toISOString().split('T')[0];
    todaySummary = await projectsStore.getDailySummary(today);
    
    // Get recent entries
    const entries = await projectsStore.getTimeEntries();
    recentEntries = entries.slice(0, 10);
  });
  
  $: totalTodayDuration = todaySummary?.total_duration || 0;
  $: projectCount = todaySummary?.project_breakdown?.length || 0;
</script>

<div class="p-6 space-y-6">
  <!-- Header -->
  <div>
    <h1 class="text-2xl font-bold text-gray-900">Dashboard</h1>
    <p class="text-gray-600 mt-1">Track your time and monitor productivity</p>
  </div>
  
  <!-- Stats Cards -->
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
    <!-- Current Session -->
    <div class="card">
      <div class="flex items-start justify-between">
        <div>
          <p class="text-sm text-gray-500">Current Session</p>
          <p class="text-2xl font-bold text-gray-900 mt-1">
            {formatDuration(sessionDuration)}
          </p>
          {#if currentProject}
            <p class="text-sm text-gray-600 mt-1">{currentProject.name}</p>
          {/if}
        </div>
        <div class="p-2 bg-primary-50 rounded-lg">
          <Clock class="h-5 w-5 text-primary-600" />
        </div>
      </div>
    </div>
    
    <!-- Today's Total -->
    <div class="card">
      <div class="flex items-start justify-between">
        <div>
          <p class="text-sm text-gray-500">Today's Total</p>
          <p class="text-2xl font-bold text-gray-900 mt-1">
            {formatDuration(totalTodayDuration)}
          </p>
          <p class="text-sm text-gray-600 mt-1">{projectCount} projects</p>
        </div>
        <div class="p-2 bg-green-50 rounded-lg">
          <Calendar class="h-5 w-5 text-green-600" />
        </div>
      </div>
    </div>
    
    <!-- Weekly Average -->
    <div class="card">
      <div class="flex items-start justify-between">
        <div>
          <p class="text-sm text-gray-500">Weekly Average</p>
          <p class="text-2xl font-bold text-gray-900 mt-1">7h 24m</p>
          <p class="text-sm text-green-600 mt-1">+12% from last week</p>
        </div>
        <div class="p-2 bg-blue-50 rounded-lg">
          <TrendingUp class="h-5 w-5 text-blue-600" />
        </div>
      </div>
    </div>
    
    <!-- Active Projects -->
    <div class="card">
      <div class="flex items-start justify-between">
        <div>
          <p class="text-sm text-gray-500">Active Projects</p>
          <p class="text-2xl font-bold text-gray-900 mt-1">{$projectsStore.projects.length}</p>
          <p class="text-sm text-gray-600 mt-1">This month</p>
        </div>
        <div class="p-2 bg-purple-50 rounded-lg">
          <Activity class="h-5 w-5 text-purple-600" />
        </div>
      </div>
    </div>
  </div>
  
  <!-- Today's Breakdown -->
  {#if todaySummary && todaySummary.project_breakdown.length > 0}
    <div class="card">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">Today's Project Breakdown</h2>
      <div class="space-y-3">
        {#each todaySummary.project_breakdown as item}
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div 
                class="w-3 h-3 rounded-full" 
                style="background-color: {item.project.color || '#6366f1'}"
              ></div>
              <span class="font-medium text-gray-900">{item.project.name}</span>
            </div>
            <span class="text-gray-600">{formatDuration(item.duration)}</span>
          </div>
        {/each}
      </div>
    </div>
  {/if}
  
  <!-- Recent Entries -->
  {#if recentEntries.length > 0}
    <div class="card">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">Recent Time Entries</h2>
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>Project</th>
              <th>Window</th>
              <th>Duration</th>
              <th>Time</th>
            </tr>
          </thead>
          <tbody>
            {#each recentEntries as entry}
              <tr>
                <td class="font-medium">{entry.project_id}</td>
                <td class="text-gray-600 truncate max-w-xs">{entry.window_title}</td>
                <td>{formatDuration(entry.duration_seconds)}</td>
                <td class="text-gray-500">
                  {new Date(entry.start_time).toLocaleTimeString()}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  {/if}
</div>