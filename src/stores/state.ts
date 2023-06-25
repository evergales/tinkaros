import { writable } from "svelte/store";

export type state = {
  loading: boolean,
  updating: boolean,
  progress: number,
  updateState: string,
  settingsShown: boolean
} | any;

export const state = writable<state>({
  loading: true,
  updating: false,
  progress: 0,
  updateState: "waiting..",
  settingsShown: false
});
