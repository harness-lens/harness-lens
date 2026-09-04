# SPDX-License-Identifier: MPL-2.0
# Copyright © 2026 Cristian Camargo Filho

import json

from harness_lens.cli import main
from harness_lens.core import discover, scan


def test_discover_finds_supported_harness_files(tmp_path):
    (tmp_path / "AGENTS.md").write_text("# Root instructions\n", encoding="utf-8")
    nested = tmp_path / "service"
    nested.mkdir()
    (nested / "CLAUDE.md").write_text("# Service instructions\n", encoding="utf-8")
    cursor_rules = tmp_path / ".cursor" / "rules" / "languages"
    cursor_rules.mkdir(parents=True)
    (cursor_rules / "python.md").write_text("# Python rules\n", encoding="utf-8")
    (tmp_path / "README.md").write_text("# Not a harness\n", encoding="utf-8")

    assert [path.as_posix() for path in discover(tmp_path)] == [
        ".cursor/rules/languages/python.md",
        "AGENTS.md",
        "service/CLAUDE.md",
    ]


def test_discover_ignores_generated_directories(tmp_path):
    generated = tmp_path / ".venv"
    generated.mkdir()
    (generated / "AGENTS.md").write_text("# Ignore me\n", encoding="utf-8")

    assert discover(tmp_path) == ()


def test_cli_emits_json(tmp_path, capsys):
    github = tmp_path / ".github"
    github.mkdir()
    (github / "copilot-instructions.md").write_text("# Copilot\n", encoding="utf-8")

    assert main([str(tmp_path), "--json"]) == 0
    output = json.loads(capsys.readouterr().out)

    assert len(output["sources"]) == 1
    assert output["sources"][0]["path"] == ".github/copilot-instructions.md"


def test_scan_exposes_evidence_score_and_observability(tmp_path):
    (tmp_path / "AGENTS.md").write_text("# Instructions\n", encoding="utf-8")

    report = scan(tmp_path)

    assert report["schema_version"] == 1
    assert report["scores"][0]["passed"] is True
    assert report["score_summary"]["quality_mean"] == 1.0
    assert report["score_summary"]["safety_violations"] == 0
    assert report["plugin_executions"][0]["status"] == "completed"
