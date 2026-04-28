/**
 * Parse a YAML string into a JavaScript value.
 * @param input - YAML string to parse
 * @throws {Error} If the YAML is invalid
 */
export declare function parseYaml(input: string): Promise<unknown>;

/**
 * Parse a YAML string containing multiple documents into an array.
 * @param input - YAML string with one or more documents separated by `---`
 * @throws {Error} If the YAML is invalid
 */
export declare function parseAllYaml(input: string): Promise<unknown[]>;

/**
 * Stringify a JavaScript value into a YAML string.
 * @param value - Value to stringify
 * @throws {Error} If the value cannot be stringified
 */
export declare function stringifyYaml(value: unknown): Promise<string>;