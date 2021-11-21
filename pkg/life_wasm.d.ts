/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
*/
export function greet(name: string): void;
/**
*/
export enum Cell {
  Dead,
  Alive,
}
/**
*/
export class Universe {
  free(): void;
/**
* @param {number} x
* @param {number} y
*/
  toggle(x: number, y: number): void;
/**
* @returns {number}
*/
  grid(): number;
/**
* @param {number} x
* @param {number} y
* @returns {number}
*/
  get(x: number, y: number): number;
/**
* @returns {number}
*/
  height(): number;
/**
* @returns {number}
*/
  width(): number;
/**
* @returns {string}
*/
  render(): string;
/**
* @returns {number}
*/
  delta(): number;
/**
* @returns {Universe}
*/
  static new(): Universe;
/**
*/
  update(): void;
}
