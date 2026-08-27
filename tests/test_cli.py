import json

from harness_lens.cli import main
from harness_lens.core import discover


def test_discover_finds_supported_harness_files(tmp_path):
    (tmp_path / "AGENTS.md").write_text("# Root instructions\n", encoding="utf-8")
    nested = tmp_path / "service"
    nested.mkdir()
    (nested / "CLAUDE.md").write_text("# Service instructions\n", encoding="utf-8")
    cursor_rules = tmp_path / ".cursor" / "rules"
    cursor_rules.mkdir(parents=True)
    (cursor_rules / "python.md").write_text("# Python rules\n", encoding="utf-8")
    (tmp_path / "README.md").write_text("# Not a harness\n", encoding="utf-8")

    assert [path.as_posix() for path in discover(tmp_path)] == [
        ".cursor/rules/python.md",
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

    assert output["count"] == 1
    assert output["files"] == [".github/copilot-instructions.md"]
