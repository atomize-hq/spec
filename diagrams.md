# spec System Diagrams

This file shows the system at multiple resolutions so the boundaries are easier
to hold in your head.

The main distinction to keep straight is:

- `spec` core is the source-of-truth semantic graph and proof pipeline
- passports and exports are derived read surfaces from that core
- semantic-family analysis is a read-side interpretation layer over checked-in
  corpus units
- corpus is an input set for that analysis layer, not the same thing as the
  unit graph itself
- plans are proposed future graph changes, not current source truth

## Resolution 0: One-screen mental model

```mermaid
flowchart LR
    P["Plans\n(.plan.spec)\nproposed future change"] --> C["spec core\nsource-of-truth semantic system"]
    U["Units\n(.unit.spec, .test.spec)"] --> C
    C --> G["Resolved graph"]
    C --> N["Native code + tests"]
    C --> E["Observed evidence"]
    C --> PP["Passports + export JSON"]
    U --> FA["Family analysis lane\n(read-side)"]
    FA --> FC["Coverage artifact"]
    FC --> FR["Recommendation artifact"]
```

## Resolution 1: Big system layers

This is the cleanest top-level separation.

```mermaid
flowchart TB
    subgraph L3["Experience Layer"]
        X1["Editors / agents / review tools"]
        X2["Inspection / navigation / diff views"]
    end

    subgraph L2["Semantic Source Layer (spec core)"]
        S1["Author semantic units"]
        S2["Validate + normalize"]
        S3["Resolve graph"]
        S4["Generate code + tests"]
        S5["Compile + execute"]
        S6["Collect evidence"]
        S7["Emit derived artifacts"]
    end

    subgraph L1["Planning Layer"]
        P1["Plan artifacts"]
        P2["Acceptance criteria"]
        P3["Proposed graph changes"]
    end

    X1 --> S1
    X2 --> S7
    P1 --> S2
    P2 --> S5
    P3 --> S3
```

## Resolution 2: The write path, what changes repo truth

This is the path that changes the actual semantic system.

```mermaid
flowchart LR
    A["Author\n*.unit.spec / *.test.spec"] --> B["Schema + policy validation"]
    B --> C["Internal IR"]
    C --> D["Resolved semantic graph"]
    D --> E["Lower + generate native code"]
    E --> F["Compile + run tests"]
    F --> G["Observed evidence"]
    G --> H["Passports / evidence artifacts / export JSON"]
```

### What each stage means

| Stage | Purpose | Truth type |
|---|---|---|
| `*.unit.spec`, `*.test.spec` | authored semantic source | authored truth |
| validation + IR | make source strict and machine-operable | normalized truth |
| resolved graph | dependency and test-link structure | structural truth |
| compile + test | native behavior check | observed truth |
| passports / export | machine-readable summaries | derived truth |

## Resolution 3: The read surfaces, what is derived vs proposed

This is the boundary people blur most often.

```mermaid
flowchart TB
    U["Authored units/tests"] --> G["Resolved graph"]
    G --> P["Passports"]
    G --> X["spec export JSON"]
    G --> ST["spec status view"]
    G --> FAM["Family analysis inputs"]

    PL["Plan artifacts"] -. separate layer .-> G
    PL -. does not overwrite .-> U

    EV["Observed build/test evidence"] --> P
    EV --> X
    EV --> ST
```

### Truth ownership table

| Surface | Example | What it is | What it is not |
|---|---|---|---|
| authored source | `examples/ecommerce/units/pricing/apply_tax.unit.spec` | source of truth | derived summary |
| observed proof | `.spec.passport.json`, `.test.evidence.json` | recorded execution result | authored intent |
| structural read model | `spec export`, `spec status` | current graph + proof projection | future plan |
| planning surface | `.plan.spec` | intended future graph delta | current implementation truth |
| family analysis | coverage/recommendation artifacts | interpretation of corpus demand | the semantic graph itself |

## Resolution 4: Where semantic families sit

Semantic families are not the whole semantic system.

They are a narrow analysis lane over the Rust `kind:function` subset.

```mermaid
flowchart TB
    U["Function units in repo"] --> SR["Semantic review"]
    SR -->|supported family key| SF["Promoted semantic families"]
    SR -->|unsupported reason code + fingerprint| UC["Unsupported clusters"]

    UC --> COV["Family coverage artifact"]
    COV --> REC["Family recommendation artifact"]

    MAN["semantic-families/corpus/rust-function.toml"] --> COV
    PKT["semantic-families/<family>/ packet fixtures"] -. proof-only / packet certification .-> SF
```

### Important distinction

| Thing | Role |
|---|---|
| semantic review | classifies a function unit against the current supported subset |
| semantic family | a promoted reusable interpretation packet |
| corpus | the checked-in input set used to study demand and route unsupported shapes |
| coverage artifact | what unsupported/supported demand the corpus currently shows |
| recommendation artifact | what next family, if any, looks promotion-worthy |

## Resolution 5: Close-up of the family-analysis lane

This is the current M27.x lane.

```mermaid
flowchart LR
    M["Corpus manifest\nsemantic-families/corpus/rust-function.toml"] --> S["Load corpus sources"]
    S --> R["Run semantic review per unit"]
    R --> K1["Supported family hits"]
    R --> K2["Unsupported reason codes + shape fingerprints"]

    K1 --> C["coverage.latest.json"]
    K2 --> C

    C --> Q["Cluster unsupported demand"]
    Q --> H["Leverage counts\nreal / regression / boundary"]
    Q --> O["Overlap-family inference"]
    H --> J["recommendation.latest.json"]
    O --> J
```

### Current family-analysis decision axes

```mermaid
flowchart TB
    A["Unsupported cluster"] --> B["How many real examples?"]
    A --> C["How many promotion-relevant regressions?"]
    A --> D["Does it overlap a known family direction?"]
    A --> E["How hard is the jump?"]

    B --> F["Leverage"]
    C --> F
    D --> G["Overlap confidence"]
    E --> H["Difficulty"]

    F --> I["Promotion readiness"]
    G --> I
    H --> I
```

## Resolution 6: The current confusion, `money/round` vs arithmetic

This is the specific distinction behind the recent corpus discussion.

```mermaid
flowchart LR
    subgraph MR["money/round cluster"]
        MR1["reason_code = unsupported_function_surface"]
        MR2["authored_body_kind = neither"]
        MR3["overlap_family = unknown"]
        MR4["real_example_hits can grow"]
    end

    subgraph AR["arithmetic-shape cluster"]
        AR1["reason_code = unsupported_arithmetic_shape"]
        AR2["overlap_family = function.arithmetic_leaf.monotone_*"]
        AR3["real_example_hits can grow"]
        AR4["readiness can flip with more corpus"]
    end

    MR1 --> MR3
    MR2 --> MR3
    MR4 --> MR3

    AR1 --> AR2
    AR3 --> AR4
    AR2 --> AR4
```

### Why these behave differently

```mermaid
flowchart TB
    X["money/round-like new corpus hit"] --> Y["same unsupported_function_surface cluster"]
    Y --> Z["still overlap_family = unknown"]
    Z --> Z2["still held for unknown_overlap_family"]

    A["new arithmetic-like real example"] --> B["same arithmetic cluster"]
    B --> C["overlap_family already known"]
    C --> D["real_example_hits crosses threshold"]
    D --> E["candidate can become ready"]
```

### Plain-English summary

| Cluster | What more corpus changes | What more corpus does not change |
|---|---|---|
| `money/round` unsupported-function-surface | leverage counts | family direction |
| arithmetic-shape | leverage counts | enough to unlock readiness because direction is already known |

That is why “more corpus” can be real progress for arithmetic but mostly just
louder uncertainty for the current `money/round` shape.

## Resolution 7: How passports relate to all of this

Passports are about per-unit proof and machine-readable state. They are not the
same artifact family as corpus recommendation.

```mermaid
flowchart LR
    U["One unit"] --> T["spec test"]
    T --> E["Observed build + local-test evidence"]
    E --> P["Unit passport"]

    U --> SR["Semantic review"]
    SR --> P

    P --> ST["spec status / spec export projections"]
    P -. may be sampled by .-> C["family coverage analysis"]
```

### Passport vs family-analysis comparison

| Surface | Center of gravity | Granularity | Main question |
|---|---|---|---|
| passport | one unit | per-unit | what do we know about this unit right now? |
| molecule evidence | one interaction test | per-test | what cross-unit behavior was observed? |
| export / status | one library or root | graph slice | what is the current structural + proof picture? |
| family coverage | many function units across sources | corpus slice | what supported and unsupported function demand exists? |
| family recommendation | many clusters | next-family candidate | what family, if any, should be promoted next? |

## Resolution 8: End-to-end map

This is the full map with the major axes on one page.

```mermaid
flowchart TB
    subgraph Authored["Authored source"]
        U["*.unit.spec"]
        T["*.test.spec"]
        PL["*.plan.spec"]
    end

    subgraph Core["spec core"]
        V["Validate + normalize"]
        G["Resolve graph"]
        N["Generate native code + tests"]
        EX["Compile + execute"]
        EV["Collect evidence"]
    end

    subgraph Derived["Derived operational truth"]
        PA["Passports"]
        TE["Molecule evidence"]
        ST["spec status"]
        EXP["spec export"]
    end

    subgraph Family["Function-family analysis lane"]
        CM["Corpus manifest"]
        SR["Semantic review over corpus units"]
        FC["coverage.latest.json"]
        FR["recommendation.latest.json"]
    end

    subgraph Planning["Planning lane"]
        PV["spec plan validate/export"]
    end

    U --> V
    T --> V
    V --> G
    G --> N
    N --> EX
    EX --> EV
    EV --> PA
    EV --> TE
    PA --> ST
    PA --> EXP
    TE --> ST
    TE --> EXP

    U --> SR
    CM --> SR
    SR --> FC
    FC --> FR

    PL --> PV
    PV -. advisory only .-> G
```

## Reading guide

If you want the shortest useful reading order:

1. Resolution 0
2. Resolution 3
3. Resolution 4
4. Resolution 6

That path gives the core distinction:

- the graph system is the semantic source and proof engine
- passports are derived per-unit proof surfaces
- semantic-family analysis is a narrow read-side interpretation lane
- corpus work changes that lane's inputs
- not every family-analysis blocker is actually fixable by more corpus
