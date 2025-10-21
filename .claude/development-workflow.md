# BIP-Keychain Development Workflow

**For**: Claude Code and Human Developers
**Methodology**: Kiro Specs + TDD (RED-GREEN-REFACTOR)
**Project**: bip-keychain-core

## Overview

This project follows **Kiro Specs** specification-driven development with strict **TDD (Test-Driven Development)** using the RED-GREEN-REFACTOR cycle.

## Kiro Specs Structure

This project uses Kiro's three-phase workflow:

```
spec/
├── requirements.md  - User stories in EARS notation
├── design.md        - Technical architecture, sequence diagrams
└── tasks.md         - Implementation plan with TDD tracking
```

### 1. Requirements (requirements.md)

Write user stories in **EARS notation**:
- **WHEN** [trigger condition]
- **THE SYSTEM SHALL** [requirement]
- **AND** [additional requirements]

**Example**:
```
**WHEN** a user provides a JSON entity and parent entropy,
**THE SYSTEM SHALL** hash the entity using the configured hash function,
**AND** convert the first 4 bytes to a uint32 BIP-32 child index.
```

### 2. Design (design.md)

Document:
- System architecture diagrams
- Module design
- Data flow
- Sequence diagrams
- Error handling strategy
- Security considerations

### 3. Tasks (tasks.md)

Track implementation with **TDD phases**:

```markdown
## Task: Implement HMAC-SHA-512 hashing

**TDD Phase**: GREEN
**Status**: Complete

### Subtasks

- [x] **RED**: Write test for HMAC-SHA-512 with known vector (commit: abc123)
- [x] **GREEN**: Implement HMAC-SHA-512 to pass test (commit: def456)
- [x] **REFACTOR**: Extract helper functions (commit: ghi789)
```

## TDD Workflow (RED-GREEN-REFACTOR)

### Phase 1: RED - Write Failing Test

**Before writing any implementation code**, write a test that:
1. Describes the expected behavior
2. Uses a known test vector or property
3. **FAILS** because the functionality doesn't exist yet

**Commit Message**:
```
test: add failing test for HMAC-SHA-512 hashing

RED phase: Test expects hash_entity() to produce known output
for test vector, but function doesn't exist yet.

Test vector from: [source]

TDD-Phase: RED
Related: spec/tasks.md#hmac-sha-512
```

**Update tasks.md**: Mark subtask RED phase complete with commit hash

### Phase 2: GREEN - Make Test Pass

Write the **minimum code** necessary to make the test pass:
1. Implement the functionality
2. Run tests until they pass
3. Don't optimize yet

**Commit Message**:
```
feat: implement HMAC-SHA-512 hashing

GREEN phase: Implement hash_entity() using hmac + sha2 crates.
Test now passes with known vector.

TDD-Phase: GREEN
TDD-Previous: abc123 (RED)
Related: spec/tasks.md#hmac-sha-512
```

**Update tasks.md**: Mark subtask GREEN phase complete with commit hash

### Phase 3: REFACTOR - Improve Code

**While keeping tests green**, improve the code:
1. Extract functions
2. Remove duplication
3. Improve readability
4. Add documentation

**Commit Message**:
```
refactor: extract canonical_json helper

REFACTOR phase: Extract JSON canonicalization into reusable
helper function. Tests still pass.

TDD-Phase: REFACTOR
TDD-Previous: def456 (GREEN)
Related: spec/tasks.md#hmac-sha-512
```

**Update tasks.md**: Mark subtask REFACTOR phase complete with commit hash

## Development Cycle

```
┌─────────────────────────────────────────────────┐
│ 1. Read requirements.md & design.md            │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 2. Pick next task from tasks.md                │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 3. RED: Write failing test                     │
│    - Commit with "test: ..." message           │
│    - Update tasks.md with RED commit hash      │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 4. Verify test fails (important!)              │
│    Run: cargo test                              │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 5. GREEN: Implement functionality               │
│    - Commit with "feat: ..." message           │
│    - Update tasks.md with GREEN commit hash    │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 6. Verify test passes                           │
│    Run: cargo test                              │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 7. REFACTOR: Improve code (if needed)           │
│    - Commit with "refactor: ..." message       │
│    - Update tasks.md with REFACTOR commit      │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 8. Verify tests still pass                      │
│    Run: cargo test                              │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 9. Mark task complete in tasks.md              │
│    Move to next task → repeat from step 2      │
└─────────────────────────────────────────────────┘
```

## Commit Message Format

### RED Phase
```
test: [description of test]

RED phase: [Why this test will fail]

TDD-Phase: RED
Related: spec/tasks.md#[task-id]
```

### GREEN Phase
```
feat: [what was implemented]

GREEN phase: [How it makes tests pass]

TDD-Phase: GREEN
TDD-Previous: [RED commit hash]
Related: spec/tasks.md#[task-id]
```

### REFACTOR Phase
```
refactor: [what was improved]

REFACTOR phase: [What changed, tests still pass]

TDD-Phase: REFACTOR
TDD-Previous: [GREEN commit hash]
Related: spec/tasks.md#[task-id]
```

## Test Vector Sources

For cryptographic implementations, use known test vectors from:
- **BIP-32**: Official test vectors in BIP document
- **BIP-85**: Test vectors from reference implementations
- **BLAKE2**: https://github.com/BLAKE2/BLAKE2/tree/master/testvectors
- **HMAC-SHA-512**: NIST test vectors

## Example Task Entry in tasks.md

```markdown
## Task: Implement Multi-Hash Support

**TDD Phase**: GREEN
**Status**: In Progress

**Requirements**: US-1 from requirements.md
**Design**: See design.md#hash-module

### Subtasks

#### HMAC-SHA-512
- [x] **RED**: Write test with NIST vector (commit: abc123)
- [x] **GREEN**: Implement using hmac crate (commit: def456)
- [x] **REFACTOR**: Extract canonicalize_json (commit: ghi789)

#### BLAKE2b
- [x] **RED**: Write test with libsodium vector (commit: jkl012)
- [x] **GREEN**: Implement using alkali crate (commit: mno345)
- [ ] **REFACTOR**: Optimize for large entities

#### SHA-256
- [ ] **RED**: Write test with NIST vector
- [ ] **GREEN**: Implement using sha2 crate
- [ ] **REFACTOR**: TBD

### Acceptance Criteria
- [ ] All three hash functions implemented
- [ ] Tests pass with known vectors
- [ ] Performance < 10ms per hash
```

## Claude Code Instructions

When working on this project:

1. **Always check spec/ directory first**
   - Read requirements.md for context
   - Read design.md for implementation guidance
   - Read tasks.md for current progress

2. **Follow TDD strictly**
   - Write tests BEFORE implementation
   - Commit after each phase (RED, GREEN, REFACTOR)
   - Update tasks.md with commit hashes

3. **Use test vectors**
   - Find official test vectors for crypto operations
   - Document source in test comments

4. **Update tasks.md immediately**
   - Mark phases complete with commit hashes
   - Move to next subtask only after REFACTOR

5. **Commit message discipline**
   - Use "test:", "feat:", "refactor:" prefixes
   - Include TDD-Phase metadata
   - Reference tasks.md task ID

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_hmac_sha512

# Run with output
cargo test -- --nocapture

# Run tests for specific module
cargo test hash::
```

## References

- **Kiro Specs**: https://kiro.dev/docs/specs/
- **Kiro Steering**: https://kiro.dev/docs/steering/
- **TDD Best Practices**: https://martinfowler.com/bliki/TestDrivenDevelopment.html
- **BIP-Keychain**: https://github.com/akarve/bip-keychain

## Quick Start for New Developers

1. Read `spec/requirements.md` - understand what we're building
2. Read `spec/design.md` - understand how we're building it
3. Read `spec/tasks.md` - see what's done and what's next
4. Pick a task marked "pending"
5. Follow TDD cycle (RED → GREEN → REFACTOR)
6. Update tasks.md as you go

## Directory Structure

```
bip-keychain-core/
├── .claude/
│   └── development-workflow.md  # This file
├── spec/
│   ├── requirements.md          # EARS user stories
│   ├── design.md                # Technical design
│   └── tasks.md                 # TDD task tracking
├── nickel/
│   ├── src/keychain.ncl         # Nickel schema definitions
│   └── examples/                # Nickel examples
├── src/
│   ├── lib.rs                   # Library entry point
│   ├── entity.rs                # Entity type definitions
│   ├── hash.rs                  # Multi-hash support
│   ├── derivation.rs            # BIP-keychain algorithm
│   ├── bip32_wrapper.rs         # BIP-32 wrapper
│   ├── output.rs                # Key formatting
│   ├── error.rs                 # Error types
│   ├── cli.rs                   # CLI logic
│   └── bin/bip-keychain.rs      # CLI entry point
├── tests/
│   └── integration_tests.rs     # Integration tests
├── Cargo.toml                   # Dependencies
└── README.md                    # Project overview
```
