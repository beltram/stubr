# IDE completion


A json schema is also maintained [here](schemas/stubr.schema.json) to provide completion in IDE. It just contains
completion for features implemented in stubr and should alleviate you from a bit of pain when writing json from scratch.

<details>
<summary><b>IntelliJ</b></summary>

* Go to `Settings > Languages & Frameworks > Schemas & DTDs > JSON Schema Mappings`
* Add a mapping (click on the upper `+`)
* Then supply the following
    * name: `stubr`
    * Schema file or URL: `https://raw.githubusercontent.com/beltram/stubr/main/schemas/stubr.schema.json`
    * Schema version: `JSON Schema version 7`
    * File path pattern: `stubs/*.json` (and `mappings/*.json` if you want to use it for original wiremock stubs)
* Then `Apply`

</details>


<details>
<summary><b>VsCode</b></summary>

* Open workspace settings (File > Preferences > Settings)
* Add the following under the property `json.schemas`
```json
"json.schemas": [{"fileMatch": ["stubs/*.json", "mappings/*.json"], "url": "https://raw.githubusercontent.com/beltram/stubr/main/schemas/stubr.schema.json"}]
```

</details>