> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Publishing HarnessLens

The release workflow uses PyPI Trusted Publishing. It needs no long-lived API token.

## One-time PyPI setup

Create a pending publisher at <https://pypi.org/manage/account/publishing/> with:

- PyPI project name: `harness-lens`
- GitHub owner: `harness-lens`
- GitHub repository: `harness-lens`
- Workflow filename: `publish-to-pypi.yml`
- Environment name: `pypi`

The pending publisher creates the PyPI project during the first successful release.

## One-time GitHub setup

1. In **Settings > Environments**, create the `pypi` environment.
2. Add required reviewers so every production publish needs approval.
3. Enable private vulnerability reporting in **Settings > Security**.
4. Protect `main`; require pull requests and these status checks:
   - `Python 3.10`
   - `Python 3.14`
   - `Rust quality`
   - `Build distribution`
   - `Analyze (actions)`
   - `Analyze (python)`

Do not create a `PYPI_API_TOKEN` secret. The publish job requests a short-lived OIDC token through its `id-token: write` permission.

## Release

1. Update `version` in `pyproject.toml` and `workspace.package.version` in
   `rust/Cargo.toml`, then refresh `rust/Cargo.lock`.
2. Merge the release commit into `main`.
3. Create a GitHub release tagged with the same version, such as `v0.0.1`.
4. Publish the GitHub release.
5. Approve the `pypi` environment deployment.
6. Verify <https://pypi.org/project/harness-lens/> and run:

```bash
python -m pip install harness-lens
harness-lens --version
```

PyPI versions are immutable. Increment the version before every later release.

The release workflow builds Python 3.10 stable-ABI wheels on Linux, macOS, and
Windows plus one source distribution. Wheel builds use Maturin's PyPI
compatibility check; publication uses the protected `pypi` environment and
short-lived OIDC credentials.
