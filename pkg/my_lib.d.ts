/* tslint:disable */
/* eslint-disable */

export class SimulationParams {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  pop_size: number;
  nb_generations: number;
  crossover_rate: number;
  mutation_rate: number;
  elite_rate: number;
}

export function run_from_web(map: string, pop_size: number, nb_generations: number, crossover_rate: number, mutation_rate: number, elite_rate: number): string[];

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_get_simulationparams_crossover_rate: (a: number) => number;
  readonly __wbg_get_simulationparams_elite_rate: (a: number) => number;
  readonly __wbg_get_simulationparams_mutation_rate: (a: number) => number;
  readonly __wbg_get_simulationparams_nb_generations: (a: number) => number;
  readonly __wbg_get_simulationparams_pop_size: (a: number) => number;
  readonly __wbg_set_simulationparams_crossover_rate: (a: number, b: number) => void;
  readonly __wbg_set_simulationparams_elite_rate: (a: number, b: number) => void;
  readonly __wbg_set_simulationparams_mutation_rate: (a: number, b: number) => void;
  readonly __wbg_set_simulationparams_nb_generations: (a: number, b: number) => void;
  readonly __wbg_set_simulationparams_pop_size: (a: number, b: number) => void;
  readonly __wbg_simulationparams_free: (a: number, b: number) => void;
  readonly run_from_web: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
