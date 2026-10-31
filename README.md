# SigmaOS Neovim

Python tooling for a manifest-driven Neovim distribution. It validates plugin
specifications, prints plugin install plans, checks caller-supplied health
facts (or probes the Neovim version and plugin directory), and generates a
deterministic `init.lua` configuration.

## Side effects and limits

`install-plan` only prints `git clone` argument vectors; it does not clone
plugins. `config-gen` prints generated configuration rather than installing
it. `health` may run `nvim --version` and read the selected plugin directory;
it does not verify plugin signatures or execute plugin code. Treat plugin
manifests as trusted configuration and review generated code before use.

## Development

`make test` exercises parsing, planning, configuration generation, and health
logic with supplied facts. It does not test plugin installation. Inspiration is
drawn from [Aether](https://github.com/omacom/aether.nvim) and
[Lumon](https://github.com/omacom/lumon.nvim); compatibility is not implied.
