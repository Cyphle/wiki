# Terraform conditionals

## With count
### if statement
* Terraform ne supporte pas le if. Il faut utiliser une manière détournée. Par exemple avec `count`. Si count = 1, la resource est là, si count = 0, il n'y a pas de resource
* Par contre, les ternaire sont supportés
```
<CONDITION> ? <TRUE_VAL> : <FALSE_VAL>
```

### if else statement
* Pour faire des if/else, il possible de créer une resource avec un ternaire et une autre avec le ternaire inverse

## With for_each and for
* Il est possible de créer des conditionals avec des boucles. S'il n'y a aucun élément à créer ou s'il y en
* Pour créer des conditionals avec des boucles il faut combiner les for_each et for loops. For example
```
dynamic "tag" {
    for_each = {
        for key, value in var.custom_tags:
        key => upper(value)
        // Conditional dans une boucle pour filtrer les tags différents de "Name"
        if key != "Name"
    }

    content {
        key = tag.key
        value = tag.value
        propagate_at_launch = true
    }
}
```

## With String directive
* Syntaxe
```
%{ if <CONDITION> }<TRUEVAL>%{ else }<FALSEVAL>%{ endif }
```
