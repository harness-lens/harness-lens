# Contributing

Create a focused branch, add tests for behavior changes, and open a pull request against `develop`.

```bash
python -m pip install -e ".[dev]"
python -m pytest
python -m build
python -m twine check --strict dist/*
```

By submitting a contribution, you agree that it will be licensed under the BSD 3-Clause License used by this project.
