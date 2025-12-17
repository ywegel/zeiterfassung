import { describe, expect, it } from "vitest";
import { generateDailySummaryCsv } from "$lib/csv.ts";
import type { DailySummary } from "$lib/api.ts";
import { Region } from "$lib/regions.ts";

describe("generateDailySummaryCsv", () => {
	it("generates correct CSV with header and multiple rows", () => {
		const data: DailySummary[] = [
			{ region: Region.Aa1, summed_duration: 3600 },
			{ region: Region.Aa2, summed_duration: 1800 },
			{ region: Region.Aa3, summed_duration: 7200 },
			{ region: Region.Ac1, summed_duration: 0 },
		];

		const result = generateDailySummaryCsv(data);

		const expected = `Region,Summed Duration (seconds)
			Aa1,3600
			Aa2,1800
			Aa3,7200
			Ac1,0`;

		expect(result).toBe(expected);
	});

	it("handles empty array gracefully (header only)", () => {
		const data: DailySummary[] = [];

		const result = generateDailySummaryCsv(data);

		const expected = "Region,Summed Duration (seconds)";

		expect(result).toBe(expected);
	});

	it("preserves order of regions as provided", () => {
		const data: DailySummary[] = [
			{ region: Region.Ac1, summed_duration: 100 },
			{ region: Region.Aa3, summed_duration: 200 },
			{ region: Region.Aa1, summed_duration: 300 },
			{ region: Region.Aa2, summed_duration: 400 },
		];

		const result = generateDailySummaryCsv(data);

		const lines = result.split("\n");
		expect(lines[1]).toBe("Ac1,100");
		expect(lines[2]).toBe("Aa3,200");
		expect(lines[3]).toBe("Aa1,300");
		expect(lines[4]).toBe("Aa2,400");
	});
});
