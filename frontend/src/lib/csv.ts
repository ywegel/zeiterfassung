import type { DailySummary } from "./api.js";

/**
 * Converts a DailySummary List into a CSV-String.
 */
export function generateDailySummaryCsv(data: DailySummary[]): string {
	const header = ["Region", "Summed Duration (seconds)"].join(",");

	const rows = data.map((entry) => `${entry.region},${entry.summed_duration}`);

	return [header, ...rows].join("\n");
}
