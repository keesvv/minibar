import "./main.css";
import App from "./App.svelte";
import { addMessages, getLocaleFromNavigator, init } from "svelte-i18n";
import nl from "./i18n/nl.json";

addMessages("nl", nl);
init({
  fallbackLocale: "nl",
  initialLocale: getLocaleFromNavigator(),
});

const app = new App({
  target: document.getElementById("app"),
});

export default app;
