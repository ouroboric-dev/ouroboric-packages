from ouroboric import INFO, main


def test_main_prints_project_info(capsys):
    main()
    out = capsys.readouterr().out
    assert out == INFO + "\n"
    assert "https://ouroboric.dev" in out
