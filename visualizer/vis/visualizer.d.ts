/* tslint:disable */
/* eslint-disable */
/**
* @param {number} seed
* @param {string} game_type
* @returns {string}
*/
export function gen(seed: number, game_type: string): string;
/**
* @param {string} _input
* @param {string} _output
* @param {number} turn
* @param {string} game_type
* @returns {Ret}
*/
export function vis(_input: string, _output: string, turn: number, game_type: string): Ret;
/**
* @param {string} _input
* @param {string} _output
* @param {string} game_type
* @returns {number}
*/
export function get_max_turn(_input: string, _output: string, game_type: string): number;
/**
*/
export class Ret {
  free(): void;
/**
*/
  err: string;
/**
*/
  score: bigint;
/**
*/
  svg: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly gen: (a: number, b: number, c: number, d: number) => void;
  readonly __wbg_ret_free: (a: number) => void;
  readonly __wbg_get_ret_score: (a: number) => number;
  readonly __wbg_set_ret_score: (a: number, b: number) => void;
  readonly __wbg_get_ret_err: (a: number, b: number) => void;
  readonly __wbg_set_ret_err: (a: number, b: number, c: number) => void;
  readonly __wbg_get_ret_svg: (a: number, b: number) => void;
  readonly __wbg_set_ret_svg: (a: number, b: number, c: number) => void;
  readonly vis: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly get_max_turn: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
