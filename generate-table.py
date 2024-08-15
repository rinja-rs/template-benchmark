#!/usr/bin/env python3

from dataclasses import dataclass
from datetime import datetime, timezone
from glob import glob
from io import StringIO
from json import load
from shutil import move
from subprocess import check_call, DEVNULL
from pathlib import Path
from tempfile import TemporaryDirectory

STYLE = """
html, body {
    background-color: transparent;
    color: black;
}
html {
    font-size: 62.5%;
}
body {
    font-size: 1.6rem;
    line-height: 1.4em;
    font-family: "DejaVu Sans", sans-serif;
}
th, td {
    padding: 0.6rem;
    border: 0.1rem solid #666;
    background-color: white;
}
td:nth-of-type(3), td:nth-of-type(4) {
    text-align: right;
}
abbr {
    font-weight: normal;
    text-decoration-style: dotted;
    text-decoration-skip-ink: none;
    text-decoration-thickness: 0.1rem;
    text-underline-offset: 0.3rem;
    text-underline-position: from-font;
}
small {
    font-size: 85%;
    color: #666;
}
"""


@dataclass
class Entry:
    point: float
    stderr: float


def null(i):
    if i >= 10:
        return f"{i:.1f}"
    else:
        return f"&#x2007;{i:.1f}"


def main(root_dir: Path):
    groups = {
        "big_table": {},
        "teams": {},
    }
    names = {}
    units = {
        "big_table": ("µs", 1000),
        "teams": ("ns", 1),
    }

    for path in sorted(glob("./target/criterion/* */* */new/", root_dir=root_dir)):
        with open(root_dir / path / "benchmark.json", "rt") as f:
            benchmark = load(f)
        with open(root_dir / path / "estimates.json", "rt") as f:
            estimates = load(f)

        group = benchmark["group_id"].split()[0]
        name, version = benchmark["function_id"].split(maxsplit=1)
        point = estimates["median"]["point_estimate"] / units[group][1]
        stderr = estimates["median"]["standard_error"] / units[group][1]

        names.setdefault(name, [version, 0])[1] += point
        groups[group][name] = Entry(point, stderr)

    names = sorted(
        ((name, version, point) for name, [version, point] in names.items()),
        key=lambda entry: entry[2],
    )

    f = StringIO()
    print("<html>", file=f)
    print("<head>", file=f)
    print('<meta charset="utf-8" />', file=f)
    print("<title>Benchmark</title>", file=f)
    print("<style>", STYLE, "</style>", sep="", file=f)
    print("</head>", file=f)
    print("<body>", file=f)
    print("<table>", file=f)
    print("<thead>", file=f)
    print("<tr>", file=f)
    print("<th>crate</th>", file=f)
    print("<th>version</th>", file=f)
    print('<th>big table <abbr title="microseconds = 10^-6 s">(µs)</abbr></th>', file=f)
    print('<th>teams <abbr title="nanoseconds = 10^-9 s">(ns)<abbr></th>', file=f)
    print("</thead>", file=f)
    print("<tbody>", file=f)
    for name, version, _ in names:
        big_table = groups["big_table"][name]
        teams = groups["teams"][name]
        print("<tr>", file=f)
        print("<td>", name, "</td>", file=f)
        print("<td>", version, "</td>", file=f)
        print(
            "<td>",
            f"{big_table.point:,.1f}",
            " <small>(± ",
            null(big_table.stderr),
            ")</small></td>",
            sep="",
            file=f,
        )
        print(
            "<td>",
            f"{teams.point:,.1f}",
            " <small>(± ",
            null(teams.stderr),
            ")</small></td>",
            sep="",
            file=f,
        )
        print("</tr>", file=f)
    print("</tbody>", file=f)
    print("<tfoot>", file=f)
    print("<tr>", file=f)
    print(
        '<td colspan="2" style="text-align: center"><small>',
        datetime.now(timezone.utc).isoformat().replace("T", " ").split(".")[0],
        "</small></td>",
        file=f,
        sep="",
    )
    print(
        '<td colspan="2" style="text-align: center"><small><em>median; lower is better</em></small></td>',
        file=f,
    )
    print("</tr>", file=f)
    print("</tfoot>", file=f)
    print("</table>", file=f)
    print("</body>", file=f)
    print("</html>", file=f)
    f.seek(0, 0)
    results = f.read()

    with TemporaryDirectory() as tempdir:
        tempdir = Path(tempdir)

        # write html table
        with open(tempdir / "results.html", "wt") as f:
            f.write(results)

        # convert html to svg
        with TemporaryDirectory(dir=tempdir) as cachedir:
            check_call(
                (
                    "/usr/bin/env",
                    "xvfb-run",
                    "wkhtmltoimage",
                    "--format",
                    "svg",
                    "--log-level",
                    "none",
                    "--cache-dir",
                    cachedir,
                    "./results.html",
                    "./results.0.svg",
                ),
                stdin=DEVNULL,
                stdout=DEVNULL,
                cwd=tempdir,
            )

        # crop to content
        check_call(
            (
                "/usr/bin/env",
                "xvfb-run",
                "inkscape",
                "--actions",
                'select-by-selector:rect[x="0"][y="0"]; delete-selection; select-all; fit-canvas-to-selection',
                "./results.0.svg",
                "-o",
                "./results.1.svg",
            ),
            stdin=DEVNULL,
            stdout=DEVNULL,
            cwd=tempdir,
        )

        # reduce file size
        check_call(
            (
                "/usr/bin/env",
                "scour",
                "--create-groups",
                "--remove-titles",
                "--remove-descriptions",
                "--remove-metadata",
                "--remove-descriptive-elements",
                "--enable-comment-stripping",
                "--disable-embed-rasters",
                "--enable-viewboxing",
                "--enable-id-stripping",
                "--shorten-ids",
                "./results.1.svg",
                "./results.2.svg",
            ),
            stdin=DEVNULL,
            stdout=DEVNULL,
            cwd=tempdir,
        )

        # we are done
        move(
            tempdir / "results.2.svg", root_dir / "target" / "criterion" / "results.svg"
        )


if __name__ == "__main__":
    main(Path(__file__).absolute().parent)
