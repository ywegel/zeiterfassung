<script lang="ts">
	import { onMount } from "svelte";
	import RegionButton from "$lib/components/RegionButton.svelte";
	import { Region } from "$lib/regions.ts";
	import { appState } from "$lib/state.svelte.ts";
	import { fetchCurrentlyActive, startTimer, stopTimer } from "$lib/api.ts";

	let regions: Region[] = Object.values(Region);

	async function initializeApp() {
		try {
			const data = await fetchCurrentlyActive();
			if (data.region !== null && data.duration !== null) {
				appState.activeRegion = data.region as Region;
				appState.currentDuration = data.duration;
			}
		} catch (error) {
			console.error("Error fetching currently active timer:", error);
			alert("Failed to fetch active timer. Please try again.");
		}
	}

	async function handleStart(region: Region) {
		if (appState.activeRegion !== null) {
			alert("Another timer is already running. Stop it first.");
			return;
		}

		try {
			await startTimer(region);

			appState.activeRegion = region;
		} catch (error) {
			console.error("Error starting timer:", error);
			alert("Failed to start timer. Please try again.");
		}
	}

	async function handleStop(region: Region) {
		try {
			const data = await stopTimer(region);
			console.log("Stop response:", data);
			appState.lastStopped = { region, duration: data.duration };
			appState.activeRegion = null;
		} catch (error) {
			console.error("Error stopping timer:", error);
			alert("Failed to stop timer. Please try again.");
		}
	}

	function toggleTimer(region: Region) {
		if (appState.activeRegion === region) {
			handleStop(region);
		} else {
			handleStart(region);
		}
		appState.currentDuration = null;
	}

	onMount(initializeApp);
</script>

<div class="flex min-h-screen flex-col items-center justify-center bg-gray-100 p-4">
	{#if appState.activeRegion && appState.currentDuration !== null}
		<div class="mb-6 text-center text-lg text-gray-800">
			Active timer: {appState.activeRegion.toUpperCase()}, Duration: {appState.currentDuration}
			seconds
		</div>
	{/if}

	<div class="grid w-full max-w-md grid-cols-2 grid-rows-3 gap-4">
		{#each regions as region (region)}
			<RegionButton {region} onToggle={toggleTimer} />
		{/each}
	</div>

	{#if appState.lastStopped}
		<div class="mt-6 text-lg text-gray-800">
			Last stopped: {appState.lastStopped.region.toUpperCase()}, Duration: {appState
				.lastStopped.duration} seconds
		</div>
	{/if}
</div>
