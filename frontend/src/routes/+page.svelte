<script lang="ts">
	import { onMount } from "svelte";

	enum Region {
		North = "north",
		East = "east",
		West = "west",
		South = "south",
	}

	let activeRegion: Region | null = $state(null);

	let lastStopped: { region: Region; duration: number } | null = $state(null);

	let currentDuration: number | null = $state(null);

	onMount(async () => {
		try {
			const response = await fetch("/api/currently_active", {
				method: "GET",
				headers: { "Content-Type": "application/json" },
			});

			if (!response.ok) {
				throw new Error("Failed to fetch currently active timer");
			}

			const data = await response.json();
			if (data.region !== null && data.duration !== null) {
				activeRegion = data.region as Region;
				currentDuration = data.duration;
			}
		} catch (error) {
			console.error("Error fetching currently active timer:", error);
			alert("Failed to fetch active timer. Please try again.");
		}
	});

	async function startTimer(region: Region) {
		if (activeRegion !== null) {
			alert("Another timer is already running. Stop it first.");
			return;
		}

		try {
			const response = await fetch(`/api/${region}/start`, {
				method: "POST",
				headers: { "Content-Type": "application/json" },
			});

			if (!response.ok) {
				throw new Error(`Failed to start timer for ${region}`);
			}

			activeRegion = region;
		} catch (error) {
			console.error("Error starting timer:", error);
			alert("Failed to start timer. Please try again.");
		}
	}

	async function stopTimer(region: Region) {
		try {
			const response = await fetch(`/api/${region}/stop`, {
				method: "POST",
				headers: { "Content-Type": "application/json" },
			});

			if (!response.ok) {
				throw new Error(`Failed to stop timer for ${region}`);
			}

			const data = await response.json();
			console.log("Stop response:", data);
			lastStopped = { region, duration: data.duration };
			activeRegion = null;
		} catch (error) {
			console.error("Error stopping timer:", error);
			alert("Failed to stop timer. Please try again.");
		}
	}

	function toggleTimer(region: Region) {
		if (activeRegion === region) {
			stopTimer(region);
		} else {
			startTimer(region);
		}
		currentDuration = null;
	}
</script>

<div class="flex min-h-screen flex-col items-center justify-center bg-gray-100 p-4">
	{#if activeRegion && currentDuration !== null}
		<div class="mb-6 text-center text-lg text-gray-800">
			Active timer: {activeRegion.charAt(0).toUpperCase() + activeRegion.slice(1)}, Duration: {currentDuration}
			seconds
		</div>
	{/if}
	<div class="grid w-full max-w-md grid-cols-2 gap-4">
		<button
			class="rounded-lg py-8 text-2xl font-bold transition-colors duration-200 {activeRegion ===
			Region.North
				? 'bg-blue-500 text-white'
				: 'bg-gray-200 text-gray-800'}"
			onclick={() => toggleTimer(Region.North)}
		>
			North
		</button>
		<button
			class="rounded-lg py-8 text-2xl font-bold transition-colors duration-200 {activeRegion ===
			Region.East
				? 'bg-green-500 text-white'
				: 'bg-gray-200 text-gray-800'}"
			onclick={() => toggleTimer(Region.East)}
		>
			East
		</button>
		<button
			class="rounded-lg py-8 text-2xl font-bold transition-colors duration-200 {activeRegion ===
			Region.West
				? 'bg-red-500 text-white'
				: 'bg-gray-200 text-gray-800'}"
			onclick={() => toggleTimer(Region.West)}
		>
			West
		</button>
		<button
			class="rounded-lg py-8 text-2xl font-bold transition-colors duration-200 {activeRegion ===
			Region.South
				? 'bg-yellow-500 text-white'
				: 'bg-gray-200 text-gray-800'}"
			onclick={() => toggleTimer(Region.South)}
		>
			South
		</button>
	</div>
	{#if lastStopped}
		<div class="mt-6 text-lg text-gray-800">
			Last stopped: {lastStopped.region.charAt(0).toUpperCase() +
				lastStopped.region.slice(1)}, Duration: {lastStopped.duration} seconds
		</div>
	{/if}
</div>
