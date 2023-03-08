# Module versioning

* Versioning types
    * Of module dependencies
    * Of module itself

## Of dependencies
* Dependency types
    * Terraform core: binary of Terraform
    * Providers: versions of providers
    * Modules: versions of modules
    
### How to fix dependencies
#### For Terraform core
* Use `required_version`
```
terraform {
    required_version = ">= 1.0.0, < 2.0.0"
}
```
* Versioning of Terraform core tool: `tfenv`
    * For Apple Silicon, add environment variable `export TFENV_ARCH=arm64`
    * Commands:
        * `tfenv install <version>`: install specific version of Terraform
        * `tfenv list`: list les versions installées
        * `tfenv use <version>`: utiliser une version spécifique de Terraform
    * Utiliser un fichier .terraform-version pour spécifier la version à utiliser dans le dossier courant

#### For providers
* Use `required_providers` block
```
terraform {
    required_version = ">= 1.0.0, 2.0.0"

    required_providers {
        aws = {
            source = "hashipcorp/aws"
            version = "~> 4.0" # Equivalent to <= 4.0, < 5.0
        }
    }
}
```

#### For modules
* Use source URLs plutôt que des modules locaux
```
module "mymodule" {
    source = "git@github.com:foo/modules/.../?ref=v0.0.5"
}

ou

module "mymodule" {
    source = "terraform-aws-modules/rds/aws"
    version = "4.4.0"
}
```
 