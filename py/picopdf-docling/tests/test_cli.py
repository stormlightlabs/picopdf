from picopdf_docling.cli import main


def test_protocol_version(capsys):
    assert main(["--protocol-version"]) == 0
    captured = capsys.readouterr()
    assert captured.out == "1\n"
    assert captured.err == ""
