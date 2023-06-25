import { writable } from "svelte/store";

export type state = {
  loading: boolean,
  loggedIn: boolean,
  accountShown: boolean,
  passwordShown: boolean,
  updating: boolean,
  progress: number,
  updateState: string
} | any;

export const state = writable<state>({
  loading: true,
  loggedIn: false,
  accountShown: false,
  passwordShown: false,
  updating: false,
  progress: 0,
  updateState: "waiting..",
});
