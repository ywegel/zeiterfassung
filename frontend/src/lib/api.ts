import type { Region } from "./regions";

export interface StopTimerResponse {
	duration: number;
}

export interface CurrentlyActiveResponse {
	region: Region | null;
	duration: number | null;
}

export async function fetchCurrentlyActive(): Promise<CurrentlyActiveResponse> {
	const response = await fetch("/api/currently_active", {
		method: "GET",
		headers: { "Content-Type": "application/json" },
	});

	if (!response.ok) {
		throw new Error("Failed to fetch currently active timer");
	}

	return response.json();
}

export async function startTimer(region: Region): Promise<void> {
	const response = await fetch(`/api/${region}/start`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
	});

	if (!response.ok) {
		throw new Error(`Failed to start timer for ${region}`);
	}
}

export async function stopTimer(region: Region): Promise<StopTimerResponse> {
	const response = await fetch(`/api/${region}/stop`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
	});

	if (!response.ok) {
		throw new Error(`Failed to stop timer for ${region}`);
	}

	return response.json();
}
