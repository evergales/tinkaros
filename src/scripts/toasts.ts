import { toasts } from "svelte-toasts";
import type { ToastType } from "svelte-toasts/types/common";

export default function newToast(type?: ToastType, title?: string, description?: string, duration?: number) {
    toasts.add({
        type: type ?? "info",
        title: title ?? "info",
        description: description ?? "no further information provided",
        duration: duration ?? 0,
        placement: "bottom-right"
    })
}