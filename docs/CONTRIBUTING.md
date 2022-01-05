# Contributing

Contributions are absolutely, positively welcome and encouraged! Contributions
come in many forms. You could:

  1. Submit a feature request or bug report as an [issue].
  2. Ask for improved documentation as an [issue].
  3. Comment on [issues that require feedback].
  4. Contribute code via [pull requests].
  5. Propose a pair-programming session to [alepez](https://devand.dev/chat/alepez)

## Setting up your local development environment

Project is currently built with `rustc 1.56.0` and tested on x86_64 Linux.

To build the project with `egui` feature enabled, you also need to install a gcc
compiler and libxcb.

On a fresh Ubuntu 20.04 you just need to install:

```shell
sudo apt-get install -y build-essential libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

For detailed informations on prerequisites, you can refer to [CICD GitHub
Workflow].

[issue]: https://github.com/alepez/provola/issues
[issues that require feedback]: https://github.com/alepez/provola/issues?q=is%3Aissue+is%3Aopen+label%3A%22feedback+wanted%22
[pull requests]: https://github.com/alepez/provola/pulls
[CICD GitHub Workflow]: https://github.com/alepez/provola/blob/main/.github/workflows/cicd.yml
