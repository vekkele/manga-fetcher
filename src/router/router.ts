
import SearchSvelte from "src/pages/Search.svelte";
import TitleSvelte from "src/pages/Title.svelte";
import type { SvelteComponent } from "svelte";
import { writable } from "svelte/store";

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
  const { subscribe, set } = writable(Route.Search);

  return {
    subscribe,
    routes,
    navigate: (route: Route) => set(route),
  }
}

const router = createRoutes();

export default router