import ky from "ky";
import { session } from "./auth";

export const api = ky.extend({
  prefixUrl: import.meta.env.VITE_API_ENDPOINT,
  hooks: {
    afterResponse: [
      (_req, _opts, res) => res.status === 401 && session.set(null),
    ],
  },
});
