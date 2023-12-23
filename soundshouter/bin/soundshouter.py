#!/usr/bin/env python3

import click
from pathlib import Path

from soundshouter.lib.index import import_sounds
from soundshouter.lib.db import init_schema, truncate_db
from soundshouter.lib.settings import get_settings


@click.group()
def commands():
    pass


@click.command()
@click.argument('path', type=click.Path())
def initdb(path):
    """Initialize the database schema and import sounds"""
    click.echo('Initialized the database schema')
    truncate_db()
    init_schema()
    click.echo('Scanning for sound files ...')
    sounds = import_sounds(Path(path).absolute())


@click.command()
def update_db():
    click.echo('Dropped the database')


@click.command()
def server():
    """Run the api server"""
    import uvicorn
    from soundshouter.server.main import app
    uvicorn.run(app, port=5000, log_level="info")


commands.add_command(server)
commands.add_command(initdb)
commands.add_command(update_db)


def main():
    commands()


if __name__ == '__main__':
    main()
