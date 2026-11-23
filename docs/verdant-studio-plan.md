# Verdant Studio: implementation plan (v4)

*Fatelines project · 23 November 2026 · evidence in [verdant-studio-context.md](verdant-studio-context.md)*

Nothing has been created in any repository yet.

Every item carries its citation under D12:
- **Std** is a standard;
- **Req** is a stakeholder requirement, which here is a decision of Alex's;
- **Der** is a derivation. "Der 2" means it reuses a Prism or Verdant pattern; "Der 3" means it is extrapolated.

Anything without a citation is not in this plan.

---

## 1. Goal and rules

| Item | Cites |
|---|---|
| `afflom/verdant-studio` is a video-game Lifecycle Management model, built only in Prism. It uses Verdant Engine as its only engine, applies its own lifecycle model to itself, and ships as one application on its GitHub Pages. | Req, Alex 16:10 and 16:43 |
| **Settlement.** The first rule that applies settles a question: (1) a standard applies, so implement it; (2) Prism or Verdant has a pattern, so reuse it unchanged; (3) neither exists, so extrapolate a canonical schema, prove it locally and cite the nearest authority. | Req, Alex 16:54 |
| **Mechanical development.** All work goes through the repo's API. Machine-readable standards are generated. Inference only writes encodings where no machine-readable form exists, and those encodings then enter like any other standard. | Req, Alex 17:18 |
| **Admission.** A construct enters only with one `Citation`. A construct is a type, field, constructor, operation, relation, value, check, or an item of this plan. | Req, Alex 18:03 |

**The three kinds of citation.**

| Kind | What it names | Resolves against |
|---|---|---|
| `Std` | The catalog row and the authority entry that define the construct | `authorities.toml`, a sha256-pinned corpus |
| `Req` | An ISO 29148 stakeholder requirement, with its origin and evidence (§4b) | The requirements register |
| `Der` | The settlement rule used, its inputs (each already admitted), and the check or theorem, which is re-run | `ids.toml` and `ledger.toml` |

**How admission is enforced.**

| Mechanism | Cites |
|---|---|
| Every declaration is built by `admit : Citation → Decl`. `Citation` is an inductive with exactly three constructors, and its arguments are register IDs. An uncited declaration does not elaborate, and lexicon terms (P13) enter the same way. | Der 2, LexLean `inductive{}` |
| **Well-founded.** A `Der` can cite only constructs that are already admitted, and an inductive has no cycles. So every chain ends in a `Std` or a `Req`, and Lean proves this structurally. | Der, from the inductive |
| `ingest-*` and `propose-encoding` are the only entry points, and the citation is a required argument. `check` gives a typed refusal when a citation does not resolve. The seal refuses anything else. | Der, D8 + P5 + P14 |
| A `Der` or a local name is refused if an admitted standard has an `exactMatch` concept for it. | Req, settlement rule 1 + Std SKOS |
| When an authority is re-pinned and a term is dropped, every construct that cites it is refused at the next `check`. | Der, P10 + constraint loop |

## 2. Decisions

All decisions are Alex's, made on 23 November 2026, and each is cited as Req.

| # | Decision |
|---|---|
| D1 | Verdant Engine is the only engine. Verdant Studio covers its whole public API. |
| D2 | **Oracles.** Use an external oracle, or failing that an official machine-readable form. Otherwise use a locally proved encoding, reported separately. |
| D3 | Pin exact engine releases. Each release already ships the `.holo`, four crates, a sparse index, a ghcr.io OCI bundle and a Zenodo record. The latest is v2.19.0. |
| D4 | Use template contract 2 (`prism-code`). |
| D5 | Qortara adds no overhead. Nothing is adopted. |
| D6 | The model comes first. A part is served only when it is relevant and verified. |
| D7 | Admitted ratchet checks are implemented in Prism inside Verdant Studio. `prism-fidelity` is not imported. |
| D8 | Development is mechanical and goes only through the API. |
| D9 | Every parameter and UI component is defined in Prism and traced to a stakeholder requirement. |
| D10 | Stakeholders are PROV-O producers and consumers. Each requirement has one origin: Identity, Refinement or Constraint. The KPI is the latency of the refinement loop. |
| D11 | Models cross-classify each other, and relevance and differentiation are computed from that. |
| D12 | The admission rule (§1). |

## 3. Patterns reused unchanged (Der 2)

| # | Pattern | Source |
|---|---|---|
| P1 | One model is the single source, and documents are generated from it. | Template R1 |
| P2 | Claim levels stay distinct: cited, proved here, or measured. | R2 |
| P3 | A capability starts as an ID, a scenario and a failing check. | R3 |
| P4 | A `build` claim names the theorems that the Lean kernel proves in the same run. | Contract 2 |
| P5 | A typed refusal is answered by a replayed acceptance vector. | R5 |
| P6 | An oracle is the authority's official corpus, committed as a sha256 artifact. PrismPM `standards/` corpora are ingested by revision and hash. | R6 `contract.artifact`; PrismPM |
| P7 | Content addressing: proofs are cached by κ, and a build is skipped when κ is unchanged. | prism-code; vTensor `kappa-cache` |
| P8 | Generated code is bound to its attestation, from the point the engine is linked. | vTensor `*-kernel.lock` |
| P9 | Standards enter as locked catalogs: a corpus, rows binding standard to facet, and an owner per row. | verdant-engine `catalogs/` |
| P10 | Registers are monotone: closed questions stay closed and limits only tighten. | verdant-engine ratchet, `fidelity.toml` |
| P11 | Every quantity cites an authority entry, and every module has a stratum. | verdant-engine ratchet |
| P12 | A gate counts once a planted defect has failed it, and that defect is recorded once. | Template `AGENTS.md` |
| P13 | A standard's terms become closed-lexicon entries. | LexLean |
| P14 | The repo is one API: an operation is an MCP tool, and a catalog is a read-only resource. The protocol core is linked unchanged. | `verdant mcp`; lexlean-mcp |

## 4. The API and the seed

| Item | Cites |
|---|---|
| The API is the set of operations defined in the model, dispatched through the built holo. It is served over `prism-code` now and over MCP (`UOR-Foundation/MCP`) once that is linked. Both transports call the same operations. | Der 2, P14 + D4 |
| **The one host effect.** The host fetches a URL that the model pins, and `check` refuses a sha256 mismatch. Every other step is pure. | Der 2, R6 |

**The seed** is the only change made outside the API. It is written once in LexLean.

| Step | Cites |
|---|---|
| 1. `prism-code init` | Der 2, D4 |
| 2a. Types for Standard and Authority | Std: PrismPM `authority-binding/1` and PROV-O `Agent` |
| 2b. Type for Oracle | Der 2, P6 |
| 2c. Type for Facet (domain × phase) | Std: ISO 25964-1 facet, over the index axes |
| 2d. Type for Owner | Std: PROV-O `wasAttributedTo` and the 42010 stakeholder |
| 2e. Type for Verdict: external, local or none | Req, D2 |
| 2f. Type for Stratum | Der 2, engine ratchet (D7) |
| 2g. Types for Stakeholder, Concern, Viewpoint and View | Std, ISO 42010 (`prism.arch` lexicon) |
| 2h. Requirement type, with its origin | Std, ISO 29148, plus Req, D10 |
| 2i. Types for ConceptScheme and the mapping relations | Std: W3C SKOS and ISO 25964-2 |
| 2j. Citation type | Req, D12 |
| 3. The operations `ingest-authority`, `ingest-index`, `propose-encoding`, `check`, `build` and `publish`. The names are not normative. | Der: D8, P9, R6 and the prism-code commands |
| 4. JSON Schema 2020-12 translator | Der: P9 catalogs are JSON, and D2 says to use the official machine form |
| 5. **Checks.** Each citation resolves (D12). Each module has a stratum (P11). Registers are monotone (P10). | Der, D7 |
| 6. One workflow: `prism-code build`, then PrismPM's `publish-pages` | Der 2, D4, plus Req: one Pages app |

**What comes after the seed.**

| Item | Cites |
|---|---|
| Everything after the seed enters through `ingest-*` or `propose-encoding`, including new translators and new operations. | Req, D8 |
| The standards index is ingested first. Its rows are citations, not claims. After ingest, the repository copy is the single source and the project-files copy becomes a mirror. | Der 2, P1 + P2 |

## 4a. The interface follows from stakeholder requirements (D9)

| Layer | Cites |
|---|---|
| Stakeholders, concerns, viewpoints and views | Std, ISO/IEC/IEEE 42010 |
| Stakeholder requirements and traceability. This standard is prose, so it is encoded (D2). | Std, ISO/IEC/IEEE 29148 |
| Context of use and user goals | Std: ISO 9241-210 and 9241-11 (index L01, L02) |
| Quality of views. WCAG is checked against the ACT test vectors. | Std: ISO/IEC 25010 (O01), WCAG 2.2 (M06) |
| Visual parameters | Std, W3C DTCG JSON |

**The chain.** standard → facet → requirement (29148) → concern → viewpoint → view (42010) → element.

| Guarantee | Cites |
|---|---|
| An element's type is indexed by its view, and a view's type by a requirement ID. An element with no chain behind it is a type error. | Der: D9 + D12 mechanism |
| The front end is generated from the model and sealed, so a component added by hand fails the seal. | Der 2: P14 + D8 |
| Every size, colour, label, range or default is a field that cites an authority entry or a requirement. | Der 2: P11 + D9 |
| **The interface traceability theorem.** Every element traces back to a catalog row, and every requirement is realized by at least one view. | Std: 29148 bidirectional traceability |
| The repository owner is the first stakeholder, because Verdant Studio applies its own model to itself. The owner's catalog view on Pages is therefore the first derived view. | Req, Alex 16:43 |

## 4b. Stakeholder model (D10)

| Item | Cites |
|---|---|
| An artifact `wasGeneratedBy` a producing activity, `wasAttributedTo` a role, and is `used` by consuming activities. | Std, W3C PROV-O |
| Player feedback is an `Observation`. | Std, W3C SOSA |
| Creator roles are the credit categories. | Std, IGDA Game Crediting Guidelines (V06) |
| External stakeholders are the outside bodies that index rows name as their authority, such as platform holders, rating boards, regulators, unions, rights holders, esports bodies and preservation bodies. | Der, the index's authority fields |
| Elicitation follows the stakeholder needs process. | Std, ISO/IEC/IEEE 15288 |
| Creators and Players are the primary classes. | Req, Alex |

**The matrix is computed, not written.** Each row is a PROV-O triple: an artifact, the producing role and the consuming roles, with phases taken from the index. The rows are generated when V06 and each domain's catalog are ingested. This plan states only the row schema. (Der: PROV-O + P9)

**The three origins.**

| Origin | Producer | Evidence a requirement must cite | Cites |
|---|---|---|---|
| Identity | Creators | The role, and the artifact that states identity | Req, Alex 17:50 |
| Refinement | Players | SOSA observations | Req, Alex 17:50 + Std SOSA |
| Constraint | Standards | The catalog row and the authority entry | Req, Alex 17:50 + Std 29148 "constraint" |

A requirement without an origin and its evidence cannot be constructed.

**Loops.** Each loop runs over the matrix edges.

| Loop | What happens | Cites |
|---|---|---|
| Identity | Identity → requirements → artifacts → creators review | Req, D10 |
| Refinement | Observation → requirement → change → release → acceptance. Its latency is the KPI. | Req, D10 + Std ISO 15939 |
| Constraint | A new edition → re-pin → re-check → refused artifacts get a requirement | Req, D10 + P10 |
| Defect reports | A verification result against an existing requirement, on a `used` edge | Std, ISO 15288 verification |

**Features.** A role's features are {produce, consume, give feedback} × its matrix edges. Each is rendered in that role's viewpoint and context of use. (Der: PROV-O `wasGeneratedBy`/`used`, SOSA, 42010 and 9241-210)

## 4c. Cross-classification (D11)

| Item | Cites |
|---|---|
| Each model publishes its parts as a SKOS `ConceptScheme`, which is its P13 lexicon. | Std, W3C SKOS |
| A model classifies another model's parts with `exactMatch`, `broadMatch`, `narrowMatch`, `closeMatch` or `relatedMatch`. | Std, ISO 25964-2 |
| Each scheme is a facet, so each part has a classification vector. | Std, ISO 25964-1 |
| Parts are classified only by other models. | Req, Alex 18:01 ("by all other models") |

**Where mappings come from.** Most need no inference.

| Source | Cites |
|---|---|
| Type and proof dependencies, taken from the LexLean semantic snapshot | Der 2, LexLean |
| PROV-O edges of the matrix | Std, PROV-O |
| Citations | Req, D12 |
| Shared URIs, giving `exactMatch` | Std, SKOS |
| Anything else goes through `propose-encoding`, and `check` requires well-formed SKOS | Req, D8 |

**How relevance and differentiation are computed.**

| Item | Cites |
|---|---|
| Requirement origins are relevant. | Req, D10 |
| A part is relevant if and only if a trace in either direction connects it to an origin. The answer is yes or no. | Std, 29148 bidirectional traceability |
| Two parts linked by a mutual `exactMatch` are one concept and are merged. | Std, SKOS |
| A part that no origin reaches gets no view and is listed in the classification output. | Der, D6 + D9 |
| The relevant set is the least fixed point over a finite powerset. Lean proves termination (decreasing count of unsettled parts) and checks the result with `decide{}`. | Der: Knaster–Tarski, P4 |

## 5. Route per standard

The route follows from what the authority publishes, which `ingest-authority` records as an IANA media type. (Der: D2 + D8)

| Published form | Route | Cites |
|---|---|---|
| Schema or grammar | A translator per meta-format, written once through `propose-encoding` and generated from then on | Req, D8 |
| Official test corpus | Replay as an oracle | Der 2, P6 |
| Binary format with only an executable reference | Engine link, using the engine's oracles. prism-code runs no executables. | Req D1, D3 |
| Prose | Search for an official machine form first. If none exists, encode through `propose-encoding` and report it as locally proved. | Req, D2 |
| No standard | Settlement rule 3 | Req, Alex 16:54 |

## 6. Order of work

| Step | Cites |
|---|---|
| 1. Seed (§4) | Req, D8 |
| 2. Repeat: compute the closure (§4c), then implement any relevant, unverified part whose dependencies are verified, by its §5 route. Dependencies are taken in topological order over type and proof dependencies. | Der: D11 + P3 |
| 3. Serve every part that is relevant and verified. The Pages app shows the current catalog and classification. | Req, D6 |

**Reports.**

| Report | Cites |
|---|---|
| KPI: refinement-loop latency | Req, D10 |
| Verification: externally verified and locally proved, as two separate numbers | Req, D2 |
| Classification: the parts that are not relevant | Req, D11 |

## 7. Risks

| Risk | Cites |
|---|---|
| PrismPM refuses the full `Browser` profile (`PP2011`). It matters only when interactive views become relevant. | PrismPM SPEC |
| `UOR-Foundation/MCP` has not passed MCP's conformance suite. No conformance claim is made until it does. | Der 2, P2 |
| Most inference is spent encoding prose standards. Finding official machine forms first reduces that cost. | Req, D2 + D8 |
