# Quelques fonctions utiles

* format(<format>, ...<args>): formate les <args> selon le <format>
* tempaltefile(<path>, <vars>): lit le <path> et affiche le rendu et utilise <vars> pour remplacer les placeholders ${...}
    * Attention le path est relatif par rapport au dossier courant
    * Pour les chemins relatifs, des helpers existent
        * path.module: chemin du filesystem du module où l'expression est définie
        * path.root: chemin du filesystem du root module
        * path.cwd: chemin du filesystem du dossier courant
* Array lookup syntax pour récupérer un élément d'une liste: `ARRAY[<INDEX>]`
* Array length: `length(<ARRAY>)`
* upper(<VALUE>)
* `concat`prend 2 ou plus listes comme input et les combine en une seule liste
* `one` prend une liste d'input et si la liste a 0 éléments, retourne null, si la liste a 1 element, retourne l'élément, si la liste a plusieurs éléments, retourne une erreur
* `file` pour lire un fichier
* `yamldecode` pour transformer du text en yaml
* `jsondecode` pour transformer du text en json