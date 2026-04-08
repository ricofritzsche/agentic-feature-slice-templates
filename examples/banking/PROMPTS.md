# PROMPTS

### 1) Init Project

```
Use $project-bootstrap to scaffold the first working version of a Rust HTTP service for a project called Banking Feature Slices Example.

Goal:
Create only the bootstrap foundation, not the business features yet.
Requirements:
* standard Cargo project
* explicit environment-based config
* structured startup logging
* GET /health endpoint
* one smoke test
* README with exact run and verify steps
* no speculative abstractions
* no generic folders such as services, repositories, helpers, utils, shared, common, or domain
```

### 2) Generate the first self-contained feature slice "Open Account"


```
Use $feature-slice-rust and $factstore-usage to implement the feature slice: open_account.

Before writing code:
- inspect the factstore repository referenced by the factstore-usage skill
- use its current API and crate names as the source of truth
- inspect `specs/open_account.md` and treat it as the authoritative feature contract
- treat the spec’s HTTP interface, success responses, failure statuses, error codes, and business rules as complete
- do not invent behavior outside the spec
- do not introduce undocumented HTTP statuses, undocumented error codes, undocumented response fields, or undocumented duplicate handling
- do not guess or wrap factstore behind a CRUD abstraction

Goal:
Implement the open_account feature end to end in the existing Rust service so that the real HTTP behavior matches `specs/open_account.md` exactly.

Feature scope:
- add the required HTTP endpoint for account opening
- validate the request locally inside the feature
- generate the account_id inside the feature
- append the intended fact(s) through factstore where needed
- return exactly the success payload shape defined in `specs/open_account.md`
- implement exactly the failure behavior defined in `specs/open_account.md`
- keep reads, writes, and boundary code explicit
- keep the implementation small, local, and runnable

Strict contract rules:
- the implemented endpoint must match the spec’s method and path exactly
- the implemented success status must match the spec exactly
- the implemented failure statuses must match the spec exactly
- do not return any extra HTTP status not named in the spec
- do not return any extra error code not named in the spec
- do not add deduplication or idempotency behavior that the spec does not define
- do not omit any field from the success payload that the spec defines
- if the spec defines normalization, trimming, casing, age rules, or date rules, implement them exactly
- if internal implementation details fail at runtime, do not invent external HTTP contract behavior outside the spec

Structure rules:
- keep the feature under `src/features/open_account/`
- use a small internal split when the feature contains multiple concerns
- boundary files may use local names such as `http_handler.rs`, `request.rs`, or `response.rs`
- pure files should use behavior-specific names only when real pure logic exists
- append/load/persist files are shell
- do not use file names based on HTTP verbs
- do not create files or folders named service, manager, repository, helper, utils, shared, common, core, or domain

Constraints:
- no CRUD repository abstraction
- no mutable table as source of truth
- no cross-feature dependency by default
- no speculative abstractions
- keep startup and config changes minimal and explicit
- keep factstore usage direct and visible at the feature boundary
- follow the exact business rules, failure behavior, and test cases from `specs/open_account.md`

Testing requirements:
- add HTTP tests for every explicit failure path named in the spec
- add HTTP tests for every explicit error code named in the spec
- add a success-path test that asserts the full response shape, not only selected fields
- add a repeat-request test if the spec defines non-idempotent behavior
- do not stop at a few representative tests; cover the explicit contract

Acceptance criteria:
- `cargo check` passes
- `cargo test` passes
- `cargo fmt --check` passes
- `cargo clippy -- -D warnings` passes
- the feature works through the existing service bootstrap
- the implemented HTTP behavior matches `specs/open_account.md` exactly
- factstore usage stays direct and explicit
- no undocumented HTTP statuses exist
- no undocumented error codes exist
- the full success payload shape is tested
- every explicit spec-listed failure path is tested

At the end, report briefly:
- which files were added or changed
- how the feature is split internally
- which files are shell and which are pure, if any
- how the implemented endpoint matches the spec
- which tests map to which spec cases
```

### Audit Prompt for "Open Account"

```
# Audit Prompt — open_account

Audit the implemented open_account feature strictly against `specs/open_account.md`.

Before auditing:
- inspect `specs/open_account.md` and treat it as the complete external contract for this feature
- inspect the real changed source code and tests only
- inspect the factstore repository referenced by the factstore-usage skill if needed to verify correct direct usage
- do not summarize intentions, likely goals, or architectural preferences
- report only what is actually true in the code and tests
- treat any undocumented HTTP status, undocumented error code, undocumented response field, undocumented normalization rule, or undocumented duplicate behavior as drift

Audit points:

## 1. HTTP contract exactness
Check whether the implementation matches the spec exactly for:
- method
- path
- request shape
- success status
- success payload shape
- failure statuses

Fail this point if:
- any extra HTTP status is returned
- any documented HTTP status is missing
- any documented response field is missing
- any undocumented response field is added

## 2. Validation and normalization exactness
Check whether the implementation enforces exactly the rules defined in the spec for:
- required fields
- blank-after-trim checks
- max lengths
- date parsing
- past-date rule
- age rule
- country-code normalization
- any other normalization or formatting rules in the spec

Fail this point if:
- a rule is missing
- a rule is implemented differently
- a rule exists in code but not in spec

## 3. Business rules exactness
Check whether the implementation matches the spec exactly for:
- account_id generation inside the feature
- account status on creation
- account currency
- created_at behavior
- one account per successful request
- zero hidden duplicate detection if the spec says the feature is not idempotent
- no out-of-scope behavior added

Fail this point if:
- any business rule is omitted
- any extra business behavior is introduced

## 4. Failure behavior exactness
Check every documented failure case from `specs/open_account.md`:
- correct status code
- correct error code
- correct branch condition

Fail this point if:
- any documented failure code is missing
- any undocumented failure code exists
- any documented failure status is mapped incorrectly
- internal failures are exposed through undocumented external contract behavior

## 5. Idempotency and duplication policy exactness
Check whether the implementation matches the spec exactly for:
- non-idempotent repeat behavior
- absence of hidden deduplication
- repeated valid requests creating multiple accounts when the spec says they should

Fail this point if:
- any deduplication exists that the spec does not define
- repeat behavior differs from the spec

## 6. Write-side behavior and factstore usage
Check whether:
- the intended fact(s) only are appended
- factstore usage is direct and explicit
- no CRUD wrapper or repository abstraction was introduced
- no mutable-table source-of-truth drift was introduced
- reads, writes, and boundary code stay explicit

Fail this point if:
- factstore is hidden behind a generic abstraction
- feature behavior is implemented through mutable CRUD source-of-truth drift
- the appended fact shape or append path contradicts the feature contract

## 7. Feature structure
Check whether:
- the feature stays local under `src/features/open_account/`
- the internal split is small and coherent
- forbidden generic technical roles were avoided
- shell and pure responsibilities are separated clearly where that is actually useful

Fail this point if:
- the feature is spread into generic shared buckets
- forbidden technical-role files or abstractions appear
- the feature boundary becomes unclear

## 8. Test coverage against the written contract
Audit the tests against the explicit spec, not against likely intent.

Check whether tests exist for:
- happy path
- full success payload shape
- malformed JSON
- wrong field type
- invalid date format
- every explicit 422 validation code
- repeat-request behavior
- any edge cases explicitly named in the spec

Fail this point if:
- a documented path has no test
- the full success payload is not asserted
- tests only partially cover the written contract

## 9. Drift and overreach
Check whether the implementation:
- adds behavior not present in the spec
- omits behavior required by the spec
- adds speculative abstractions
- adds unnecessary startup/config changes
- introduces cross-feature dependencies without need

Fail this point if any such drift exists.

Output format:

1. Pass/fail per audit point
2. Exact files or code shapes that are problematic, if any
3. Exact mismatches against `specs/open_account.md`
4. A contract coverage matrix with:
   - each documented success path
   - each documented failure path
   - each documented error code
   - whether it is implemented
   - whether it is tested
5. Missing tests or weak coverage
6. Minimal fixes required before the feature is done
```
