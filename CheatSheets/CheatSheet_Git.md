# Git and Github cheatsheet

## Github CLI
* Install Github CLI: https://cli.github.com/
* Downlod all repositories of an organization
```
gh repo list <organizationName> --limit 1000 | while read -r repo _; do
  gh repo clone "$repo" "$repo"
done
```