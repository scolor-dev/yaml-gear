// js/index.d.ts

/**
 * Parse a YAML string into a JavaScript value.
 */
export declare function parseYaml(input: string): Promise<unknown>;

/**
 * Stringify a JavaScript value into a YAML string.
 */
export declare function stringifyYaml(value: unknown): Promise<string>;