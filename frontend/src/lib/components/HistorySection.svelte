<script lang="ts">
	import { onMount } from "svelte";
	import { type DailySummary, fetchDailyHistory } from "$lib/api.js";
	import { generateDailySummaryCsv } from "$lib/csv.js";

	let history: DailySummary[] = [];
	let error: string | null = null;
	let csvContent: string = "";

	// Refresh CSV, when history changes
	$: csvContent = history.length > 0 ? generateDailySummaryCsv(history) : "";

	onMount(async () => {
		try {
			history = await fetchDailyHistory();
		} catch (err) {
			error = "Failed to load history data";
			console.error(err);
		}
	});

	function copyCsvToClipboard() {
		if (!csvContent) return;

		navigator.clipboard.writeText(csvContent).then(
			() => alert("CSV copied to clipboard!"),
			(err) => {
				console.error("Failed to copy CSV:", err);
				alert("Failed to copy CSV");
			}
		);
	}
</script>

<div id="history-section" class="bg-gray-200 p-4">
	{#if error}
		<p class="text-lg text-red-600">{error}</p>
	{:else if history.length === 0}
		<p class="text-lg text-gray-800">No history data available.</p>
	{:else}
		<div class="flex flex-col gap-4">
			<!-- Copy-Button -->
			<div class="mx-auto max-w-2xl">
				<button
					on:click={copyCsvToClipboard}
					class="rounded-lg bg-green-600 px-6 py-2 font-semibold text-white shadow transition hover:bg-green-700"
				>
					Copy today's summary as CSV
				</button>
			</div>

			<!-- Tabel -->
			<div class="overflow-x-auto">
				<table class="mx-auto w-full max-w-2xl rounded-lg bg-white shadow-md">
					<thead>
						<tr class="bg-blue-600 text-white">
							<th class="px-4 py-3 text-left font-semibold">Region</th>
							<th class="px-4 py-3 text-left font-semibold"
								>Summed Duration (seconds)</th
							>
						</tr>
					</thead>
					<tbody>
						{#each history as entry (entry.region)}
							<tr class="border-b hover:bg-gray-50">
								<td class="px-4 py-2">{entry.region}</td>
								<td class="px-4 py-2">{entry.summed_duration}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	{/if}
</div>
