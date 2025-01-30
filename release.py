#!/usr/bin/env python3
import subprocess
import argparse
import sys
import re
from pathlib import Path


def run_command(command, check=True):
    """Run a shell command and return its output"""
    try:
        result = subprocess.run(
            command,
            check=check,
            text=True,
            shell=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error executing command: {command}")
        print(f"Error output: {e.stderr}")
        sys.exit(1)


def update_workspace_toml(comment_out: bool):
    """Update the workspace.toml file to comment/uncomment sideko-py"""
    workspace_path = Path("Cargo.toml")
    if not workspace_path.exists():
        print("Error: Cargo.toml not found")
        sys.exit(1)

    content = workspace_path.read_text()
    if comment_out:
        new_content = content.replace('"sideko-py",', '# "sideko-py",')
    else:
        new_content = content.replace('# "sideko-py",', '"sideko-py",')

    workspace_path.write_text(new_content)


def get_current_version():
    """Get the current version from Cargo.toml"""
    cargo_toml = Path("sideko/Cargo.toml").read_text()
    version_match = re.search(r'version = "([^"]+)"', cargo_toml)
    if not version_match:
        print("Error: Could not find version in Cargo.toml")
        sys.exit(1)
    return version_match.group(1)


def main():
    parser = argparse.ArgumentParser(
        description="Automate the release process for the CLI"
    )
    parser.add_argument(
        "--version",
        help="Version to release (if not provided, will use current version)",
    )
    args = parser.parse_args()

    # Get version
    version = args.version or get_current_version()

    print(f"Starting release process for version {version}")

    # 1. Comment out sideko-py
    print("Commenting out sideko-py...")
    update_workspace_toml(comment_out=True)

    # 2. Commit and push changes
    print("Committing and pushing changes...")
    run_command("git add Cargo.toml")
    run_command(f'git commit -m "chore: prepare release {version}"')
    run_command("git push")

    # 3. Create and push tag
    print(f"Creating and pushing tag v{version}...")
    run_command(f"git tag v{version}")
    run_command(f"git push origin v{version}")

    # 4. Uncomment sideko-py
    print("Uncommenting sideko-py...")
    update_workspace_toml(comment_out=False)

    # 5. Push changes
    print("Pushing updated Cargo.toml...")
    run_command("git add Cargo.toml")
    run_command('git commit -m "chore: restore sideko-py after release"')
    run_command("git push")

    # 6. Trigger GitHub workflow
    print("Triggering GitHub  workflow...")
    run_command("gh workflow run release-py.yml")

    # 7. Comment out sideko-py again
    print("Commenting out sideko-py again...")
    update_workspace_toml(comment_out=True)

    # 8. Final push
    print("Pushing final changes...")
    run_command("git add Cargo.toml")
    run_command('git commit -m "chore: comment out sideko-py"')
    run_command("git push")

    print("Release process completed successfully!")


if __name__ == "__main__":
    main()
