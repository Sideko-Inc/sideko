#!/usr/bin/env python3
import subprocess
import argparse
import sys
import re
from pathlib import Path
from typing import List, Tuple


def run_command(command: str, check: bool = True) -> str:
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


def show_git_diff(files: List[Path]) -> None:
    """Show git diff for the given files"""
    diff_output = run_command(f"git diff {' '.join(str(f) for f in files)}")
    if diff_output:
        print("\n=== Changes to be committed ===")
        print(diff_output)
        print("=" * 30)
    else:
        print("\nNo changes detected in files")


def confirm_action(prompt: str) -> bool:
    """Ask for user confirmation"""
    response = input(f"\n{prompt} (y/N): ").lower()
    return response == "y"


def find_members_section(content: str) -> Tuple[int, int, List[str]]:
    """
    Find the members section in the TOML content and return its location and current members.
    Returns (start_line_idx, end_line_idx, current_members)
    """
    lines = content.splitlines()
    members_start = None
    members_end = None
    current_members = []
    in_workspace = False

    for i, line in enumerate(lines):
        stripped = line.strip()

        # Track if we're in the workspace section
        if stripped == "[workspace]":
            in_workspace = True
            continue

        # If we hit another section, stop looking
        if stripped.startswith("[") and stripped != "[workspace]":
            in_workspace = False
            continue

        if in_workspace and "members" in line and "=" in line:
            members_start = i
            bracket_count = line.count("[") - line.count("]")

            # Handle single-line case
            if bracket_count == 0:
                members_end = i
                # Parse single-line members
                members_str = line.split("=")[1].strip()
                current_members = [
                    m.strip(' "[]') for m in members_str.split(",") if m.strip()
                ]
                break

            # Multi-line case
            current_line = i + 1
            while current_line < len(lines) and bracket_count > 0:
                bracket_count += lines[current_line].count("[") - lines[
                    current_line
                ].count("]")
                if '"' in lines[current_line]:
                    member = lines[current_line].strip().strip('",')
                    if member:
                        current_members.append(member)
                current_line += 1
            members_end = current_line - 1
            break

    if members_start is None:
        raise ValueError("Could not find members section in workspace.toml")

    return members_start, members_end, current_members


def update_workspace_toml(delete: bool) -> Path:
    """Update the workspace.toml file to delete/restore sideko-py"""
    workspace_path = Path("Cargo.toml")
    if not workspace_path.exists():
        print("Error: Cargo.toml not found")
        sys.exit(1)

    content = workspace_path.read_text()
    try:
        # Find the members section and parse current members
        members_start, members_end, current_members = find_members_section(content)

        # Update members list
        if delete:
            new_members = [m for m in current_members if m != "sideko-py"]
        else:
            if "sideko-py" not in current_members:
                new_members = current_members + ["sideko-py"]
            else:
                new_members = current_members

        # Reconstruct the members section
        lines = content.splitlines()
        original_indent = " " * (
            len(lines[members_start]) - len(lines[members_start].lstrip())
        )

        if len(new_members) <= 2:  # Use single-line format for short lists
            members_str = ", ".join(f'"{m}"' for m in new_members)
            members_line = f"{original_indent}members = [{members_str}]"
            new_lines = (
                lines[:members_start] + [members_line] + lines[members_end + 1 :]
            )
        else:  # Use multi-line format
            new_lines = (
                lines[:members_start]
                + [f"{original_indent}members = ["]
                + [f'{original_indent}    "{m}",' for m in new_members]
                + [f"{original_indent}]"]
                + lines[members_end + 1 :]
            )

        # Write new content, preserving original newline
        new_content = "\n".join(new_lines)
        if content.endswith("\n"):
            new_content += "\n"

        workspace_path.write_text(new_content)
        return workspace_path

    except Exception as e:
        print(f"Error updating workspace.toml: {e}")
        sys.exit(1)


def update_version(file_path: Path, new_version: str) -> Path:
    """Update version in a Cargo.toml file"""
    if not file_path.exists():
        print(f"Error: {file_path} not found")
        sys.exit(1)

    content = file_path.read_text()
    new_content = re.sub(
        r'version = "[^"]+"', f'version = "{new_version}"', content, count=1
    )

    # Write new content
    file_path.write_text(new_content)
    return file_path


def get_current_version() -> str:
    """Get the current version from Cargo.toml"""
    cargo_path = Path("sideko/Cargo.toml")
    if not cargo_path.exists():
        print("Error: sideko/Cargo.toml not found")
        sys.exit(1)

    content = cargo_path.read_text()
    version_match = re.search(r'version = "([^"]+)"', content)
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

    current_version = get_current_version()
    if not args.version:
        raise Exception(f"Must specify version. Current is: {current_version}")

    version = args.version
    print(f"\nBumping version from {current_version} to {version}")

    # Track all modified files
    modified_files = []

    # Update versions in both Cargo.toml files
    modified_files.append(update_version(Path("sideko/Cargo.toml"), version))
    modified_files.append(update_version(Path("sideko-py/Cargo.toml"), version))

    # Delete sideko-py from workspace
    modified_files.append(update_workspace_toml(delete=True))

    # Update the generated CLI docs
    run_command("cd docs && cargo run && cd ..")
    modified_files.append(Path("docs/CLI.md"))

    # Show all changes
    show_git_diff(modified_files)

    if not confirm_action("Do the changes look correct?"):
        print("Aborting on user request")
        # Restore from git
        run_command("git restore " + " ".join(str(f) for f in modified_files))
        sys.exit(1)

    modified_files.append(Path("Cargo.lock"))
    print(f"\nStarting release process for version {version}")

    # 1. Initial commit and push changes
    print("\nStep 1: Committing and pushing changes...")
    run_command("git add " + " ".join(str(f) for f in modified_files))
    run_command(f'git commit -m "chore: prepare release {version}"')
    run_command("git push")

    # 2. Create and push tag
    if confirm_action(
        f"Run and push tag? git tag v{version} && git push origin v{version}"
    ):
        run_command(f"git tag v{version}")
        run_command(f"git push origin v{version}")

    # 3. Restore sideko-py
    print("\nStep 3: Restoring sideko-py to workspace...")
    update_workspace_toml(delete=False)
    show_git_diff([Path("Cargo.toml")])
    if not confirm_action("Does this look correct?"):
        print("Aborting on user request")
        run_command("git restore Cargo.toml")
        sys.exit(1)

    # 4. Push changes
    print("\nStep 4: Pushing updated Cargo.toml...")
    run_command("git add Cargo.toml Cargo.lock")
    run_command('git commit -m "chore: restore sideko-py for pip release"')
    run_command("git push")

    # 5. Trigger GitHub workflow
    print("\nStep 5: Triggering GitHub workflow...")
    if confirm_action("Trigger GitHub workflow?"):
        run_command("gh workflow run release-py.yml")

    # 6. Remove sideko-py again
    print("\nStep 6: Removing sideko-py from workspace...")
    update_workspace_toml(delete=True)
    show_git_diff([Path("Cargo.toml")])
    if not confirm_action("Does this look correct?"):
        print("Aborting on user request")
        run_command("git restore Cargo.toml")
        sys.exit(1)

    # 7. Final push
    print("\nStep 7: Pushing final changes...")
    run_command("git add Cargo.toml Cargo.lock")
    run_command(
        'git commit -m "chore: remove sideko-py from workspace for dist release"'
    )
    run_command("git push")

    print("\nRelease process completed.")


if __name__ == "__main__":
    main()
