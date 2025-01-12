# Rust Template Benchmark

This benchmark tries to compare different template engines in Rust.
We consider both precompiled and dynamic engines, which of course have wildly different runtime
performances.

**Beware:** One engine might be faster than the other.
But if the other engine is more ergonomical to work with,
e.g. because you have no re-compilation times, then it might still be the better fit for you.
Only you can decide which characteristics of a template engine
(runtime speed, simplicity of programming, …) are important to you.

Currently, we compare eleven different template engines.
All of them are well maintained, mature and perform HTML escaping automatically.

| Crate          | Docs                                            | Repo                               | Code         | Recent<br>Downloads                  | Github<br>Stars                              | Contrib-<br>utors                            | Recent<br>Commits                          |
| -------------- | ----------------------------------------------- | ---------------------------------- | -------------| ------------------------------------ | -------------------------------------------- | -------------------------------------------- | ------------------------------------------ |
| [askama]       | [![][docs-img-askama]][docs-askama]             | [![][img-repo]][repo-askama]       | pre-compiled | [![][dl-askama]][askama]             | [![][stars-askama]][repo-askama]             | [![][contr-askama]][repo-askama]             | [![][act-askama]][repo-askama]             |
| [handlebars]   | [![][docs-img-handlebars]][docs-handlebars]     | [![][img-repo]][repo-handlebars]   | interpreted  | [![][dl-handlebars]][handlebars]     | [![][stars-handlebars]][repo-handlebars]     | [![][contr-handlebars]][repo-handlebars]     | [![][act-handlebars]][repo-handlebars]     |
| [horrorshow]   | [![][docs-img-horrorshow]][docs-horrorshow]     | [![][img-repo]][repo-horrorshow]   | pre-compiled | [![][dl-horrorshow]][horrorshow]     | [![][stars-horrorshow]][repo-horrorshow]     | [![][contr-horrorshow]][repo-horrorshow]     | [![][act-horrorshow]][repo-horrorshow]     |
| [markup]       | [![][docs-img-markup]][docs-markup]             | [![][img-repo]][repo-markup]       | pre-compiled | [![][dl-markup]][markup]             | [![][stars-markup]][repo-markup]             | [![][contr-markup]][repo-markup]             | [![][act-markup]][repo-markup]             |
| [maud]         | [![][docs-img-maud]][docs-maud]                 | [![][img-repo]][repo-maud]         | pre-compiled | [![][dl-maud]][maud]                 | [![][stars-maud]][repo-maud]                 | [![][contr-maud]][repo-maud]                 | [![][act-maud]][repo-maud]                 |
| [minijinja]    | [![][docs-img-minijinja]][docs-minijinja]       | [![][img-repo]][repo-minijinja]    | interpreted  | [![][dl-minijinja]][minijinja]       | [![][stars-minijinja]][repo-minijinja]       | [![][contr-minijinja]][repo-minijinja]       | [![][act-minijinja]][repo-minijinja]       |
| [rinja]        | [![][docs-img-rinja]][docs-rinja]               | [![][img-repo]][repo-rinja]        | pre-compiled | [![][dl-rinja]][rinja]               | [![][stars-rinja]][repo-rinja]               | [![][contr-rinja]][repo-rinja]               | [![][act-rinja]][repo-rinja]               |
| [ructe]        | [![][docs-img-ructe]][docs-ructe]               | [![][img-repo]][repo-ructe]        | pre-compiled | [![][dl-ructe]][ructe]               | [![][stars-ructe]][repo-ructe]               | [![][contr-ructe]][repo-ructe]               | [![][act-ructe]][repo-ructe]               |
| [sailfish]     | [![][docs-img-sailfish]][docs-sailfish]         | [![][img-repo]][repo-sailfish]     | pre-compiled | [![][dl-sailfish]][sailfish]         | [![][stars-sailfish]][repo-sailfish]         | [![][contr-sailfish]][repo-sailfish]         | [![][act-sailfish]][repo-sailfish]         |
| [tera]         | [![][docs-img-tera]][docs-tera]                 | [![][img-repo]][repo-tera]         | interpreted  | [![][dl-tera]][tera]                 | [![][stars-tera]][repo-tera]                 | [![][contr-tera]][repo-tera]                 | [![][act-tera]][repo-tera]                 |
| [tinytemplate] | [![][docs-img-tinytemplate]][docs-tinytemplate] | [![][img-repo]][repo-tinytemplate] | interpreted  | [![][dl-tinytemplate]][tinytemplate] | [![][stars-tinytemplate]][repo-tinytemplate] | [![][contr-tinytemplate]][repo-tinytemplate] | [![][act-tinytemplate]][repo-tinytemplate] |
| [upon] | [![][docs-img-upon]][docs-upon] | [![][img-repo]][repo-upon] | interpreted  | [![][dl-upon]][upon] | [![][stars-upon]][repo-upon] | [![][contr-upon]][repo-upon] | [![][act-upon]][repo-upon] |

Please see also [*Rust web framework comparison*].

## Benchmark Results

The benchmarking was done on Github's action runners, on an AMD-64 Linux system.
The absolute performance can vary, but the relative performance of crate *X* to *Y* should be stable.

[![](https://rinja-rs.github.io/template-benchmark/results.svg)](https://rinja-rs.github.io/template-benchmark/)

More information can be found in: <https://rinja-rs.github.io/template-benchmark/>.

The "big table" benchmark generates a 100×100 cell HTML table.
The "teams" benchmark contains a list of four sport teams and their scores.
The former test contains only raw data, but a lot of it.
The latter test includes texts that (might) need escaping, and "if" conditions.

To run the benchmarks on your machine, clone the repo, and execute `cargo benchmark`.
The console will contain performance numbers,
and the file `target/criterion/report/index.html` will contain more information.
Run `./generate-table.py` to generate the table you see above.

[*Rust web framework comparison*]: <https://github.com/flosse/rust-web-framework-comparison>
[img-repo]: <https://img.shields.io/badge/-repo-f8f8f8?style=flat-square&logo=github&logoColor=black>

[askama]: <https://crates.io/crates/askama/>
[handlebars]: <https://crates.io/crates/handlebars/>
[horrorshow]: <https://crates.io/crates/horrorshow/>
[markup]: <https://crates.io/crates/markup/>
[maud]: <https://crates.io/crates/maud/>
[minijinja]: <https://crates.io/crates/minijinja/>
[rinja]: <https://crates.io/crates/rinja/>
[ructe]: <https://crates.io/crates/ructe/>
[sailfish]: <https://crates.io/crates/sailfish/>
[tera]: <https://crates.io/crates/tera/>
[tinytemplate]: <https://crates.io/crates/tinytemplate/>
[upon]: <https://crates.io/crates/upon/>

[docs-askama]: <https://docs.rs/askama/latest/askama>
[docs-handlebars]: <https://docs.rs/handlebars/latest/handlebars>
[docs-horrorshow]: <https://docs.rs/horrorshow/latest/horrorshow>
[docs-markup]: <https://docs.rs/markup/latest/markup>
[docs-maud]: <https://docs.rs/maud/latest/maud>
[docs-minijinja]: <https://docs.rs/minijinja/latest/minijinja>
[docs-rinja]: <https://docs.rs/rinja/latest/rinja>
[docs-ructe]: <https://docs.rs/ructe/latest/ructe>
[docs-sailfish]: <https://docs.rs/sailfish/latest/sailfish>
[docs-tera]: <https://docs.rs/tera/latest/tera>
[docs-tinytemplate]: <https://docs.rs/tinytemplate/latest/tinytemplate>
[docs-upon]: <https://docs.rs/upon/latest/upon>

[docs-img-askama]: <https://img.shields.io/docsrs/askama?label=&style=flat-square>
[docs-img-handlebars]: <https://img.shields.io/docsrs/handlebars?label=&style=flat-square>
[docs-img-horrorshow]: <https://img.shields.io/docsrs/horrorshow?label=&style=flat-square>
[docs-img-markup]: <https://img.shields.io/docsrs/markup?label=&style=flat-square>
[docs-img-maud]: <https://img.shields.io/docsrs/maud?label=&style=flat-square>
[docs-img-minijinja]: <https://img.shields.io/docsrs/minijinja?label=&style=flat-square>
[docs-img-rinja]: <https://img.shields.io/docsrs/rinja?label=&style=flat-square>
[docs-img-ructe]: <https://img.shields.io/docsrs/ructe?label=&style=flat-square>
[docs-img-sailfish]: <https://img.shields.io/docsrs/sailfish?label=&style=flat-square>
[docs-img-tera]: <https://img.shields.io/docsrs/tera?label=&style=flat-square>
[docs-img-tinytemplate]: <https://img.shields.io/docsrs/tinytemplate?label=&style=flat-square>
[docs-img-upon]: <https://img.shields.io/docsrs/upon?label=&style=flat-square>

[dl-askama]: <https://img.shields.io/crates/dr/askama?label=&color=f8f8f8&style=flat-square>
[dl-handlebars]: <https://img.shields.io/crates/dr/handlebars?label=&color=f8f8f8&style=flat-square>
[dl-horrorshow]: <https://img.shields.io/crates/dr/horrorshow?label=&color=f8f8f8&style=flat-square>
[dl-markup]: <https://img.shields.io/crates/dr/markup?label=&color=f8f8f8&style=flat-square>
[dl-maud]: <https://img.shields.io/crates/dr/maud?label=&color=f8f8f8&style=flat-square>
[dl-minijinja]: <https://img.shields.io/crates/dr/minijinja?label=&color=f8f8f8&style=flat-square>
[dl-rinja]: <https://img.shields.io/crates/dr/rinja?label=&color=f8f8f8&style=flat-square>
[dl-ructe]: <https://img.shields.io/crates/dr/ructe?label=&color=f8f8f8&style=flat-square>
[dl-sailfish]: <https://img.shields.io/crates/dr/sailfish?label=&color=f8f8f8&style=flat-square>
[dl-tera]: <https://img.shields.io/crates/dr/tera?label=&color=f8f8f8&style=flat-square>
[dl-tinytemplate]: <https://img.shields.io/crates/dr/tinytemplate?label=&color=f8f8f8&style=flat-square>
[dl-upon]: <https://img.shields.io/crates/dr/upon?label=&color=f8f8f8&style=flat-square>

[stars-askama]: <https://img.shields.io/github/stars/djc/askama?label=&color=f8f8f8&style=flat-square>
[stars-handlebars]: <https://img.shields.io/github/stars/sunng87/handlebars-rust?label=&color=f8f8f8&style=flat-square>
[stars-horrorshow]: <https://img.shields.io/github/stars/Stebalien/horrorshow-rs?label=&color=f8f8f8&style=flat-square>
[stars-markup]: <https://img.shields.io/github/stars/utkarshkukreti/markup.rs?label=&color=f8f8f8&style=flat-square>
[stars-maud]: <https://img.shields.io/github/stars/lambda-fairy/maud?label=&color=f8f8f8&style=flat-square>
[stars-minijinja]: <https://img.shields.io/github/stars/mitsuhiko/minijinja?label=&color=f8f8f8&style=flat-square>
[stars-rinja]: <https://img.shields.io/github/stars/rinja-rs/rinja?label=&color=f8f8f8&style=flat-square>
[stars-ructe]: <https://img.shields.io/github/stars/kaj/ructe?label=&color=f8f8f8&style=flat-square>
[stars-sailfish]: <https://img.shields.io/github/stars/rust-sailfish/sailfish?label=&color=f8f8f8&style=flat-square>
[stars-tera]: <https://img.shields.io/github/stars/Keats/tera?label=&color=f8f8f8&style=flat-square>
[stars-tinytemplate]: <https://img.shields.io/github/stars/bheisler/TinyTemplate?label=&color=f8f8f8&style=flat-square>
[stars-upon]: <https://img.shields.io/github/stars/rossmacarthur/upon?label=&color=f8f8f8&style=flat-square>

[contr-askama]: <https://img.shields.io/github/contributors/djc/askama?label=&color=f8f8f8&style=flat-square>
[contr-handlebars]: <https://img.shields.io/github/contributors/sunng87/handlebars-rust?label=&color=f8f8f8&style=flat-square>
[contr-horrorshow]: <https://img.shields.io/github/contributors/Stebalien/horrorshow-rs?label=&color=f8f8f8&style=flat-square>
[contr-markup]: <https://img.shields.io/github/contributors/utkarshkukreti/markup.rs?label=&color=f8f8f8&style=flat-square>
[contr-maud]: <https://img.shields.io/github/contributors/lambda-fairy/maud?label=&color=f8f8f8&style=flat-square>
[contr-minijinja]: <https://img.shields.io/github/contributors/mitsuhiko/minijinja?label=&color=f8f8f8&style=flat-square>
[contr-rinja]: <https://img.shields.io/github/contributors/rinja-rs/rinja?label=&color=f8f8f8&style=flat-square>
[contr-ructe]: <https://img.shields.io/github/contributors/kaj/ructe?label=&color=f8f8f8&style=flat-square>
[contr-sailfish]: <https://img.shields.io/github/contributors/rust-sailfish/sailfish?label=&color=f8f8f8&style=flat-square>
[contr-tera]: <https://img.shields.io/github/contributors/Keats/tera?label=&color=f8f8f8&style=flat-square>
[contr-tinytemplate]: <https://img.shields.io/github/contributors/bheisler/TinyTemplate?label=&color=f8f8f8&style=flat-square>
[contr-upon]: <https://img.shields.io/github/contributors/rossmacarthur/upon?label=&color=f8f8f8&style=flat-square>

[act-askama]: <https://img.shields.io/github/commit-activity/y/djc/askama?label=&color=f8f8f8&style=flat-square>
[act-handlebars]: <https://img.shields.io/github/commit-activity/y/sunng87/handlebars-rust?label=&color=f8f8f8&style=flat-square>
[act-horrorshow]: <https://img.shields.io/github/commit-activity/y/Stebalien/horrorshow-rs?label=&color=f8f8f8&style=flat-square>
[act-markup]: <https://img.shields.io/github/commit-activity/y/utkarshkukreti/markup.rs?label=&color=f8f8f8&style=flat-square>
[act-maud]: <https://img.shields.io/github/commit-activity/y/lambda-fairy/maud?label=&color=f8f8f8&style=flat-square>
[act-minijinja]: <https://img.shields.io/github/commit-activity/y/mitsuhiko/minijinja?label=&color=f8f8f8&style=flat-square>
[act-rinja]: <https://img.shields.io/github/commit-activity/y/rinja-rs/rinja?label=&color=f8f8f8&style=flat-square>
[act-ructe]: <https://img.shields.io/github/commit-activity/y/kaj/ructe?label=&color=f8f8f8&style=flat-square>
[act-sailfish]: <https://img.shields.io/github/commit-activity/y/rust-sailfish/sailfish?label=&color=f8f8f8&style=flat-square>
[act-tera]: <https://img.shields.io/github/commit-activity/y/Keats/tera?label=&color=f8f8f8&style=flat-square>
[act-tinytemplate]: <https://img.shields.io/github/commit-activity/y/bheisler/TinyTemplate?label=&color=f8f8f8&style=flat-square>
[act-upon]: <https://img.shields.io/github/commit-activity/y/rossmacarthur/upon?label=&color=f8f8f8&style=flat-square>

[repo-askama]: <https://github.com/djc/askama>
[repo-handlebars]: <https://github.com/sunng87/handlebars-rust>
[repo-horrorshow]: <https://github.com/Stebalien/horrorshow-rs>
[repo-markup]: <https://github.com/utkarshkukreti/markup.rs>
[repo-maud]: <https://github.com/lambda-fairy/maud>
[repo-minijinja]: <https://github.com/mitsuhiko/minijinja>
[repo-rinja]: <https://github.com/rinja-rs/rinja>
[repo-ructe]: <https://github.com/kaj/ructe>
[repo-sailfish]: <https://github.com/rust-sailfish/sailfish>
[repo-tera]: <https://github.com/Keats/tera>
[repo-tinytemplate]: <https://github.com/bheisler/TinyTemplate>
[repo-upon]: <https://github.com/rossmacarthur/upon>
