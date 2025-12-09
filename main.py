#!/usr/bin/env python3

import typer

app = typer.Typer()

@app.command()
def main():
    print("Hello from rustcode!")


if __name__ == "__main__":
    app()
