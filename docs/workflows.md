# Workflow

This repo has github workflow that uses tags as CI system to publish builds.

You can create tags with:

```bash
git push -a v0.1.0 # Creates new tag on current branch head
git push --tags # pushes tag to github, this should trigger the workflow process
```

* push a tag in the form of `v[0-9]+.[0-9]+.[0-9]+*` (e.g. `v1.1.42`) to trigger the flow
