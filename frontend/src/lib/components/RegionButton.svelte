<script lang="ts">
	import { Region } from "$lib/regions.ts";
	import { appState } from "$lib/state.svelte.ts";

	type Props = {
		region: Region;
		onToggle: (region: Region) => void;
	};

	const { region, onToggle }: Props = $props();

	function getActiveColor(region: Region): string {
		switch (region) {
			case Region.Aa1:
				return "bg-blue-500";
			case Region.Aa2:
				return "bg-green-500";
			case Region.Aa3:
				return "bg-red-500";
			case Region.Ac1:
				return "bg-yellow-500";
			case Region.Ac2:
				return "bg-purple-500";
			case Region.Ac3:
				return "bg-orange-500";
			default:
				return "bg-gray-500";
		}
	}

	const isActive = $derived(appState.activeRegion === region);
</script>

<button
	class="rounded-lg py-8 text-2xl font-bold transition-colors duration-200 {isActive
		? getActiveColor(region) + ' text-white'
		: 'bg-gray-200 text-gray-800'}"
	onclick={() => onToggle(region)}
>
	{region.toUpperCase()}
</button>
