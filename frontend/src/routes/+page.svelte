<script lang="ts">
	import { onMount } from "svelte";

	enum Region {
		Aa1 = "aa1",
		Aa2 = "aa2",
		Aa3 = "aa3",
		Ac1 = "ac1",
		Ac2 = "ac2",
		Ac3 = "ac3",
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
			Active timer: {activeRegion.toUpperCase()}, Duration: {currentDuration} seconds
		</div>
	{/if}
	<div class="grid w-full max-w-md grid-cols-2 grid-rows-3 gap-4">
		<button
			class="rounded-lg py-8 text-2xl font-bold transition-colors duration-200 {activeRegion ===
			Region.Aa1
				? 'bg-blue-500 text-white'
				: 'bg-gray-200 text-gray-800'}"
			onclick={() => toggleTimer(Region.Aa1)}
		>
			AA1
		</button>
		<button
			class="rounded-lg py-8 text-2xl font-bold transition-colors duration-200 {activeRegion ===
			Region.Aa2
				? 'bg-green-500 text-white'
				: 'bg-gray-200 text-gray-800'}"
			onclick={() => toggleTimer(Region.Aa2)}
		>
			AA2
		</button>
		<button
			class="rounded-lg py-8 text-2xl font-bold transition-colors duration-200 {activeRegion ===
			Region.Aa3
				? 'bg-red-500 text-white'
				: 'bg-gray-200 text-gray-800'}"
			onclick={() => toggleTimer(Region.Aa3)}
		>
			AA3
		</button>
		<button
			class="rounded-lg py-8 text-2xl font-bold transition-colors duration-200 {activeRegion ===
			Region.Ac1
				? 'bg-yellow-500 text-white'
				: 'bg-gray-200 text-gray-800'}"
			onclick={() => toggleTimer(Region.Ac1)}
		>
			AC1
		</button>
		<button
			class="rounded-lg py-8 text-2xl font-bold transition-colors duration-200 {activeRegion ===
			Region.Ac2
				? 'bg-purple-500 text-white'
				: 'bg-gray-200 text-gray-800'}"
			onclick={() => toggleTimer(Region.Ac2)}
		>
			AC2
		</button>
		<button
			class="rounded-lg py-8 text-2xl font-bold transition-colors duration-200 {activeRegion ===
			Region.Ac3
				? 'bg-orange-500 text-white'
				: 'bg-gray-200 text-gray-800'}"
			onclick={() => toggleTimer(Region.Ac3)}
		>
			AC3
		</button>
	</div>
	{#if lastStopped}
		<div class="mt-6 text-lg text-gray-800">
			Last stopped: {lastStopped.region.toUpperCase()}, Duration: {lastStopped.duration} seconds
		</div>
	{/if}
</div>
