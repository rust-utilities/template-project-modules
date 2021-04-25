# Template Project Modules
[heading__top]:
  #template-project-modules
  "&#x2B06; Template that provides examples of using project modules"


Template that provides examples of using project modules


## [![Byte size of Template Project Modules][badge__main__template_project_modules__source_code]][template_project_modules__main__source_code] [![Open Issues][badge__issues__template_project_modules]][issues__template_project_modules] [![Open Pull Requests][badge__pull_requests__template_project_modules]][pull_requests__template_project_modules] [![Latest commits][badge__commits__template_project_modules__main]][commits__template_project_modules__main]


---


- [:arrow_up: Top of Document][heading__top]

- [:building_construction: Requirements][heading__requirements]

- [:zap: Quick Start][heading__quick_start]

- [&#x1F9F0; Usage][heading__usage]

- [&#x1F5D2; Notes][heading__notes]

- [:chart_with_upwards_trend: Contributing][heading__contributing]

  - [:trident: Forking][heading__forking]
  - [:currency_exchange: Sponsor][heading__sponsor]

- [:card_index: Attribution][heading__attribution]

- [:balance_scale: Licensing][heading__license]


---



## Requirements
[heading__requirements]:
  #requirements
  "&#x1F3D7; Prerequisites and/or dependencies that this project needs to function properly"


This repository requires [Rust][rust_home] language/compiler to build from source. As of last update to this ReadMe file, the recommended method of installing Rust is via the official installer script...


```Bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


To use the `generate` command-line option the `cargo-generate` crate needs to be installed...


```Bash
cargo install cargo-generate
```


______


## Quick Start
[heading__quick_start]:
  #quick-start
  "&#9889; Perhaps as easy as one, 2.0,..."


This repository is a template intended to assist with setting up new Rust applications that use internal modules.


Examples are provided for directory, file, and shared modules, as well as how to `use` each within various scopes.


```Bash
cargo generate --name 'your-application-name'\
               --git https://github.com/rust-utilities/template-project-modules.git
```


If this template is wanted as a favorite then add it to the _`$CARGO_HOME/cargo-generate`_ configuration file, eg...


```Conf
[favorite.project-modules]
description = "Template for application scoped modules"
git = "https://github.com/rust-utilities/template-project-modules.git"
branch = "main"
```


... Then the following command may be used for generating new projects from this template...


```Bash
cargo generate project-modules your-application-name
```


______


## Usage
[heading__usage]:
  #usage
  "&#x1F9F0; How to utilize this repository"


Generate a new project using this template, then change the current working directory, and either `build` or `run` via `cargo` command...


```Bash
cargo generate project-modules your-application-name

cd your-application-name

cargo run
```


... Customize the files within the `src/` directory and enjoy!


______


## Notes
[heading__notes]:
  #notes
  "&#x1F5D2; Additional things to keep in mind when developing"


This repository may not be feature complete and/or fully functional, Pull Requests that add features or fix bugs are certainly welcomed.


______


## Contributing
[heading__contributing]:
  #contributing
  "&#x1F4C8; Options for contributing to template-project-modules and rust-utilities"


Options for contributing to template-project-modules and rust-utilities


---


### Forking
[heading__forking]:
  #forking
  "&#x1F531; Tips for forking template-project-modules"


Start making a [Fork][template_project_modules__fork_it] of this repository to an account that you have write permissions for.


- Add remote for fork URL. The URL syntax is _`git@github.com:<NAME>/<REPO>.git`_...


```Bash
cd ~/git/hub/rust-utilities/template-project-modules

git remote add fork git@github.com:<NAME>/template-project-modules.git
```


- Commit your changes and push to your fork, eg. to fix an issue...


```Bash
cd ~/git/hub/rust-utilities/template-project-modules


git commit -F- <<'EOF'
:bug: Fixes #42 Issue


**Edits**


- `<SCRIPT-NAME>` script, fixes some bug reported in issue
EOF


git push fork main
```


> Note, the `-u` option may be used to set `fork` as the default remote, eg. _`git push -u fork main`_ however, this will also default the `fork` remote for pulling from too! Meaning that pulling updates from `origin` must be done explicitly, eg. _`git pull origin main`_


- Then on GitHub submit a Pull Request through the Web-UI, the URL syntax is _`https://github.com/<NAME>/<REPO>/pull/new/<BRANCH>`_


> Note; to decrease the chances of your Pull Request needing modifications before being accepted, please check the [dot-github](https://github.com/rust-utilities/.github) repository for detailed contributing guidelines.


---


### Sponsor
  [heading__sponsor]:
  #sponsor
  "&#x1F4B1; Methods for financially supporting rust-utilities that maintains template-project-modules"


Thanks for even considering it!


Via Liberapay you may <sub>[![sponsor__shields_io__liberapay]][sponsor__link__liberapay]</sub> on a repeating basis.


Regardless of if you're able to financially support projects such as template-project-modules that rust-utilities maintains, please consider sharing projects that are useful with others, because one of the goals of maintaining Open Source repositories is to provide value to the community.


______


## Attribution
[heading__attribution]:
  #attribution
  "&#x1F4C7; Resources that where helpful in building this project so far."


- [GitHub -- `github-utilities/make-readme`](https://github.com/github-utilities/make-readme)

- [Lib RS -- `cargo-generate`](https://lib.rs/crates/cargo-generate)

- [Matrix -- Rust (Lang) -- `altum_videtur`](https://matrix.to/#/!fsEJmDUHIdYFfFRTSH:jki.re/$uOtX7fI-wvryDSjNCQLmdeFJYvg2mDWTgB4cKkN5wUg?via=jki.re&via=matrix.org&via=privacytools.io)

- [Matrix -- Rust (lang) -- `tanriol`](https://matrix.to/#/!fsEJmDUHIdYFfFRTSH:jki.re/$GDXY6iKjiCoPY0JFFcw5-iag4CLz3SJN02WY3DoVWNs?via=jki.re&via=matrix.org&via=privacytools.io)


______


## License
[heading__license]:
  #license
  "&#x2696; Legal side of Open Source"


```
Template that provides examples of using project modules
Copyright (C) 2021 S0AndS0

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, version 3 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```


For further details review full length version of [AGPL-3.0][branch__current__license] License.



[branch__current__license]:
  /LICENSE
  "&#x2696; Full length version of AGPL-3.0 License"


[badge__commits__template_project_modules__main]:
  https://img.shields.io/github/last-commit/rust-utilities/template-project-modules/main.svg

[commits__template_project_modules__main]:
  https://github.com/rust-utilities/template-project-modules/commits/main
  "&#x1F4DD; History of changes on this branch"


[template_project_modules__community]:
  https://github.com/rust-utilities/template-project-modules/community
  "&#x1F331; Dedicated to functioning code"


[issues__template_project_modules]:
  https://github.com/rust-utilities/template-project-modules/issues
  "&#x2622; Search for and _bump_ existing issues or open new issues for project maintainer to address."

[template_project_modules__fork_it]:
  https://github.com/rust-utilities/template-project-modules/
  "&#x1F531; Fork it!"

[pull_requests__template_project_modules]:
  https://github.com/rust-utilities/template-project-modules/pulls
  "&#x1F3D7; Pull Request friendly, though please check the Community guidelines"

[template_project_modules__main__source_code]:
  https://github.com/rust-utilities/template-project-modules/
  "&#x2328; Project source!"

[badge__issues__template_project_modules]:
  https://img.shields.io/github/issues/rust-utilities/template-project-modules.svg

[badge__pull_requests__template_project_modules]:
  https://img.shields.io/github/issues-pr/rust-utilities/template-project-modules.svg

[badge__main__template_project_modules__source_code]:
  https://img.shields.io/github/repo-size/rust-utilities/template-project-modules


[rust_home]:
  https://www.rust-lang.org/
  "Home page for Rust language"

[rust_github]:
  https://github.com/rust-lang
  "Source code for Rust on GitHub"

[sponsor__shields_io__liberapay]:
  https://img.shields.io/static/v1?logo=liberapay&label=Sponsor&message=rust-utilities

[sponsor__link__liberapay]:
  https://liberapay.com/rust-utilities
  "&#x1F4B1; Sponsor developments and projects that rust-utilities maintains via Liberapay"

