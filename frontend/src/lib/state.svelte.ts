import type { Region } from "./regions.ts";

export type ApplicationState = {
	activeRegion: Region | null;
	currentDuration: number | null;
	lastStopped: { region: Region; duration: number } | null;
};

export const appState: ApplicationState = $state({
	activeRegion: null,
	currentDuration: null,
	lastStopped: null,
});
