import { writable } from "svelte/store";

type updatePopup = {
  shown: boolean,
  version: string,
  notes: string,
  pub_date: string,
  url: string
}

export type state = {
  loading: boolean,
  loggedIn: boolean,
  accountShown: boolean,
  infoPopupShown: boolean,
  updatePopup: updatePopup,
  passwordShown: boolean,
  updating: boolean,
  progress: number,
  updateState: string
} | any;

export const state = writable<state>({
  loading: true,
  loggedIn: false,
  accountShown: false,
  infoPopupShown: false,
  updatePopup: {
    shown: false,
    version: "",
    notes: "",
    pub_date: "",
    url: ""
  },
  passwordShown: false,
  updating: false,
  progress: 0,
  updateState: "waiting..",
});
