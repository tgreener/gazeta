# gazeta

A command-line tool for interacting with the [Nextcloud News](https://github.com/nextcloud/news) API. Currently exports your feeds as OPML for import into a local RSS reader. More operations planned.

Named after the humble newspaper kiosk. It routes things.

## Features

- Fetches feeds and folders from Nextcloud News
- Exports to OPML with folder structure preserved
- Designed as a pipe-friendly CLI tool \u2014 more subcommands coming

## Installation

```bash
git clone https://github.com/youruser/gazeta
cd gazeta
cargo build --release
cp target/release/gazeta ~/.local/bin/
```

## Configuration

Create `~/.config/gazeta/config.toml`:

```toml
username = "youruser"
password = "yourapppassword"   # use a Nextcloud app password, not your real one
domain   = "https://your.nextcloud.com"
# output = "/custom/path/feeds.opml"   # optional, default shown below
```

The default output path is `~/.local/share/gazeta/gazeta.opml`.

## Usage

```bash
# export feeds to OPML
gazeta export
```

### Pairing with bulletty

```bash
gazeta export && bulletty import ~/.local/share/gazeta/gazeta.opml && bulletty update
```

Or set it as a `before_tui` hook in your bulletty config:

```toml
[hooks]
before_tui = "gazeta export && bulletty import ~/.local/share/gazeta/gazeta.opml && bulletty update"
```

## Roadmap

- [ ] Read state sync
- [ ] Mark items read/unread
- [ ] Starred items
- [ ] Pipe-friendly output modes (JSON, plain URLs)

## License

GPLv3. See [LICENSE](LICENSE).
