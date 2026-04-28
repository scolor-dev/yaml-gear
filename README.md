# YAML-GEAR

## インストール

```bash
npm install yaml-gear@alpha
```

## 使い方

`await init()` は不要です。そのまま呼び出せます。

```js
import { parseYaml, stringifyYaml } from 'yaml-gear';

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

### ブラウザ

```html
<script type="module">
  import { parseYaml, stringifyYaml } from 'https://esm.sh/yaml-gear@alpha';
  const obj = await parseYaml('name: Alice\nage: 30\n');
  console.log(obj);
</script>
```

### Vite / webpack などバンドラー経由

```js
import { parseYaml, stringifyYaml } from 'yaml-gear';
const obj = await parseYaml('name: Alice\nage: 30\n');
console.log(obj); // => { name: 'Alice', age: 30 }
```

### エラーハンドリング

```js
try {
  await parseYaml('invalid: : yaml');
} catch (e) {
  console.error(e.message);
}
```
