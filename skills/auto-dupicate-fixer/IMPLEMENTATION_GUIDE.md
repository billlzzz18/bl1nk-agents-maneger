# Guardian Implementation Guide

How to use Guardian (Auto Duplicate Fixer) as a persona-driven skill.

---

## 🚀 Quick Start

### Before Each Session
```bash
# 1. Read Guardian's journal for context
cat .guardian.md

# 2. Activate Guardian persona
# (This means: think like Guardian for this session)

# 3. Start profiling
npm run dry-run <project-path>
```

### During Session
```bash
# Follow Guardian's Daily Process:
# 🔍 PROFILE → ⚡ SELECT → 🔧 OPTIMIZE → ✅ VERIFY → 🎁 PRESENT
```

---

## 📋 Workflow Example

### Session: Consolidating Duplicate Utilities

```bash
# 1️⃣ PROFILE - Hunt for architectural smells
node scripts/detect.js /path/to/project --format=json --min-tokens=50
# Output: .duplicate-report.json

# 2️⃣ SELECT - Evaluate options
# Guardian reads: .duplicate-report.json + .keep-remove-map.json
# Scores candidates by: usage × test coverage × clarity
# Selects: src/utils/format.ts (8 usages, 52 tests) vs src/helpers/formatter.ts (1 usage, 0 tests)

# 3️⃣ OPTIMIZE - Implement safely
node scripts/refactor.js /path/to/project
# Changes:
#   - Merged content into src/utils/format.ts
#   - Updated 5 import statements
#   - Deleted src/helpers/formatter.ts
#   - Updated barrel files

# 4️⃣ VERIFY - Run full validation
bash scripts/validate.sh /path/to/project
# Checks: tests, types, lint, build

# 5️⃣ PRESENT - Create commit with impact
# Guardian generates:
#   Title: "🛡️ Guardian: Consolidated duplicate format utilities"
#   Body: Impact metrics + verification checklist
```

---

## 🎯 Guardian's Decision Process

### Scoring Matrix

```
File A: src/utils/format.ts
  - Usage count: 8 files → +10 points
  - Test coverage: 52 tests (92%) → +10 points
  - Clear merge target: Yes → +10 points
  - Naming convention: ✓ follows src/utils/ pattern → +5 points
  - TOTAL: 35 points (✅ KEEPER)

File B: src/helpers/formatter.ts
  - Usage count: 1 file → +0 points
  - Test coverage: 0 tests (0%) → -10 points (red flag)
  - Clear merge target: Yes → +10 points
  - Naming convention: ✗ helpers/ is deprecated → +0 points
  - TOTAL: 0 points (❌ TO DELETE)

Decision: Keep A, Delete B
```

---

## ⚙️ Configuration

Guardian works out-of-the-box for most projects, but you can customize:

### `.guardianrc.json` (Optional)
```json
{
  "targetLanguages": ["ts", "js", "tsx", "jsx", "py"],
  "minDuplicationThreshold": 80,
  "minTestCoveragePercent": 50,
  "excludeDirs": ["node_modules", ".git", "dist"],
  "scoringWeights": {
    "usageCount": 3,
    "testCoverage": 3,
    "clearDecision": 2,
    "namingConvention": 1
  }
}
```

---

## 📊 Measurement & Impact

Guardian documents impact in commits:

### Metrics Guardian Tracks

```
Files Before/After:
  - Reduced: 5 files → 3 files (-40%)
  - Consolidated: 2 duplicates → 1 source

Code Lines:
  - Total LOC: 1,200 → 900 (-25%)
  - Duplicate LOC: 300 → 0 (-100%)

Import Standardization:
  - Paths fixed: 12
  - Import cycles resolved: 2

Test Coverage:
  - Maintained: 92% → 92%
  - New coverage gaps: 0
```

---

## 🔧 Usage Patterns

### Pattern 1: Daily Cleanup
```bash
# Run every morning (safe, incremental)
npm run daily /path/to/project
```

### Pattern 2: Full Audit
```bash
# Run when code feels messy
npm run aggressive /path/to/project
```

### Pattern 3: Preview Mode
```bash
# See what would be cleaned without making changes
npm run dry-run /path/to/project
```

---

## 🛡️ Safety Guarantees

Guardian NEVER:
- ❌ Makes breaking changes
- ❌ Deletes code without updating imports
- ❌ Runs without full test suite passing
- ❌ Merges code with different responsibilities
- ❌ Skips type checking

---

## 📖 Output Files

After Guardian runs, you get:

| File | Purpose |
|------|---------|
| `.duplicate-report.json` | What duplicates were found |
| `.keep-remove-map.json` | Which file stays, which goes |
| `.refactor-log.json` | Exactly what changed |
| `.validation-log.json` | Test/lint/build results |
| `.deploy-log.json` | Commit/PR status |
| `.pipeline-log.json` | Full execution timeline |

Example:
```bash
cat .keep-remove-map.json
# Shows: "keep src/utils/format.ts, remove src/helpers/formatter.ts"

cat .refactor-log.json
# Shows: "updated 5 files, deleted 1 file"

cat .validation-log.json
# Shows: "tests: PASS, types: PASS, lint: PASS, build: PASS"
```

---

## 🚨 When Things Go Wrong

### Test Fails
```bash
# Guardian automatically rolls back
git status
# Should be: "On branch main, working tree clean"

# Check log to see what failed
cat .validation-log.json | grep error
```

### Import Breaking
```bash
# Guardian validates 100% of imports before committing
# If this happens, you found a bug in Guardian

# Report to the skill team with:
cat .refactor-log.json
cat .validation-log.json
```

---

## 📚 Learning from Guardian

Guardian's journal (`.guardian.md`) grows over time:

```bash
# After each successful cleanup, Guardian may add a learning:
cat .guardian.md

# Example entries:
# - "False positive: files look same but serve different purposes"
# - "Import cycles resolved by reordering"
# - "Test coverage divergence indicates stale code"
```

---

## 🎓 Examples by Language

### TypeScript Example
```typescript
// BEFORE: 2 files
// src/utils/format.ts (52 tests)
export function formatDate(d: Date): string { ... }

// src/helpers/formatter.ts (0 tests, stale)
export function formatDate(d: Date): string { ... }

// AFTER: 1 file
// src/utils/format.ts (52 tests, merged)
export function formatDate(d: Date): string { ... }
export function formatTime(d: Date): string { ... }

// Updated imports across codebase
import { formatDate, formatTime } from '@utils/format';
```

### Python Example
```python
# BEFORE: 2 modules
# src/validators.py (with validate_email)
# src/services/user.py (also has validate_email)

# AFTER: 1 module
# src/validators.py (canonical location)
def validate_email(email: str) -> bool:
    """Centralized email validation."""
    ...

# Updated imports
from src.validators import validate_email
```

---

## 🔗 Integration

### With CI/CD
```yaml
# GitHub Actions example
- name: 🛡️ Guardian Cleanup
  run: npm run daily ./src
  
- name: Create PR if changes
  if: success()
  uses: peter-evans/create-pull-request@v4
```

### With Git Hooks
```bash
# .husky/pre-commit
#!/bin/bash
npm run lint:duplicates
```

---

## 📞 Support

If Guardian finds something unexpected:

1. **Check the journal:** `.guardian.md`
   - May already have explanation

2. **Review decision logic:** `.keep-remove-map.json`
   - See why file was chosen for deletion

3. **Check validation:** `.validation-log.json`
   - Ensure all tests passed

4. **Dry-run to preview:** `npm run dry-run`
   - See changes before committing

---

## 💡 Pro Tips

1. **Run weekly:** Keeps architecture fresh
2. **Review commits:** Guardian's messages explain every change
3. **Monitor metrics:** Watch files/LOC decrease over time
4. **Read journal:** Learn patterns specific to your codebase
5. **Trust validation:** If tests pass, Guardian's change is safe

---

## 🎯 Success Signs

✅ Your codebase is cleaner when:
- Fewer files with same responsibility
- No circular dependencies
- Consistent import patterns
- High test coverage maintained
- Build times stable or improving

---

*Last Updated: 2024*
*Guardian Version: 1.0*
