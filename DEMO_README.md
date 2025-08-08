# GitHub Comment Generation Demo

This PR demonstrates the fix for issue #603 where Squawk fails to upload large linting reports to GitHub with a 422 "Unprocessable Entity" error.

## Test Migrations

This PR includes test migrations that demonstrate different GitHub comment generation scenarios:

### 1. `001_demo_normal_comment.sql`
- **Purpose**: Demonstrates normal GitHub comment generation
- **Expected Behavior**: Full SQL content displayed with violations
- **Violations**: 4-6 typical migration violations
- **Comment Size**: Within normal limits

### 2. `002_demo_sql_truncation.sql` 
- **Purpose**: Demonstrates SQL truncation functionality
- **Expected Behavior**: SQL content truncated at 50 lines with notice
- **Violations**: Multiple violations detected across all lines
- **Comment Size**: Reduced due to truncation

## Key Features Being Tested

1. **Size Limit Enforcement**: Pre-check comment size before GitHub API calls
2. **Smart Truncation**: Limit SQL preview while preserving all violations
3. **Summary Mode**: For very large reports, switch to summary-only mode
4. **Error Handling**: Better user feedback for size limit issues

## Expected GitHub Comments

Each migration should generate a different style of GitHub comment:

- **Normal comments** for typical cases (< 65K chars)
- **Truncated comments** for medium files (SQL truncated at 50 lines)
- **Summary comments** for very large files (no SQL content, just violations)

## Testing Instructions

1. The GitHub Actions workflow should run Squawk on these migrations
2. Comments should be posted to this PR demonstrating each behavior
3. All comments should be within GitHub's 65,536 character limit
4. All violations should be properly detected and reported

This allows reviewers to see the actual behavior of the fix in a real GitHub environment.
