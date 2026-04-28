# yaml-gear

A fast YAML parser and stringifier powered by Rust + WebAssembly.

## Install

```bash
npm install yaml-gear@alpha
```

## Usage

`await init()` は不要です。そのまま呼び出せます。

```js
import { parseYaml, parseAllYaml, stringifyYaml } from 'yaml-gear';

const obj = await parseYaml(`
name: Alice
age: 30
active: true
scores:
  - 100
  - 95
  - 87
`);
console.log(obj);
// => { name: 'Alice', age: 30, active: true, scores: [100, 95, 87] }

const yaml = await stringifyYaml({ name: 'Alice', age: 30 });
console.log(yaml);
// => "name: Alice\nage: 30\n"
```

### マルチドキュメント

`---` で区切られた複数ドキュメントを配列として取得できます。

```js
const docs = await parseAllYaml(`
---
name: Alice
---
name: Bob
`);
console.log(docs);
// => [{ name: 'Alice' }, { name: 'Bob' }]
```

### ブラウザ

```html
<script type="module">
  import { parseYaml, stringifyYaml } from 'https://esm.sh/yaml-gear@alpha';
  const obj = await parseYaml('name: Alice\nage: 30\n');
  console.log(obj); // => { name: 'Alice', age: 30 }
</script>
```

### エラーハンドリング

```js
try {
  await parseYaml('invalid: : yaml');
} catch (e) {
  console.error(e.message);
}
```