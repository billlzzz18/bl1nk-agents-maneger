# Auto Duplicate File Fixer Skill

ตรวจจับและแก้ไขไฟล์ซ้ำในโปรเจค TypeScript, JavaScript, Python อย่างอัตโนมัติ

## Quick Start

```bash
# Check only (dry-run)
npm run dry-run /path/to/project

# Daily fix (safe, incremental)
npm run daily /path/to/project

# Aggressive fix (larger patches)
npm run aggressive /path/to/project
```

## What It Does

1. **Detect**: หาไฟล์ซ้ำเหมือน, โค้ดซ้ำ, โครงสร้างซ้ำ
2. **Decide**: เลือกไฟล์ไหนให้เก็บ อิงจาก test coverage > import count > type definitions
3. **Refactor**: แก้ไข imports, ลบไฟล์ซ้ำ, อัพเดท barrel files
4. **Validate**: รัน tests, lint, typecheck, build
5. **Deploy**: สร้าง commit หรือ PR อัตโนมัติ

## Configuration

Supports:
- TypeScript (.ts, .tsx)
- JavaScript (.js, .jsx)
- Python (.py)

## Outputs

- `.duplicate-report.json` - ผลจากการ detect
- `.keep-remove-map.json` - decision mapping
- `.refactor-log.json` - changes applied
- `.validation-log.json` - test results
- `.deploy-log.json` - deployment status
- `.pipeline-log.json` - full execution log

## Error Handling

- Auto rollback on test failures
- Dry-run mode to preview changes
- Detailed error logs for debugging

## Requirements

- Node.js >= 16
- jscpd (installed via npm)
- git (for commits)
- pytest (for Python projects)

## Patterns

### TS/JS Naming Convention
```
src/
  utils/          # shared utilities
  helpers/        # avoid! merge with utils
  services/       # business logic
  hooks/          # React hooks
  components/     # UI components
```

### Python Pattern
```
src/
  common/         # shared functions
  core/           # core logic
  utils.py        # avoid duplicate utils
```

Duplicates across directories get merged/removed automatically.
