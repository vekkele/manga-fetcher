
import SearchSvelte from "src/pages/Search.svelte";
import TitleSvelte from "src/pages/Title.svelte";
import type { SvelteComponent } from "svelte";
import { derived, writable } from "svelte/store";

export enum Route {
  Search = 'search',
  Title = 'title',
}

type Routes = {
  [key: string]: typeof SvelteComponent
}

const routes: Routes = {
  [Route.Search]: SearchSvelte,
  [Route.Title]: TitleSvelte,
}

function createRoutes() {
  const store = writable(Route.Search);

  return {
    store,
    routes,
    navigate: (route: Route) => store.set(route),
  }
}

export const router = createRoutes();

export const page = derived(router.store, $route => routes[$route]);
