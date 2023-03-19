# Pod

* Smallest object that can be created
* Contain one or multiple containers

## Pod descriptor yaml
```
apiVersion: v1
kind: Pod
metadata:
    name: myapp-pod
    labels:
        app: myapp
        type: front-end

spec:
    containers:
        - name: nginx-container
          image: nginx
```
* Labels: il peut y en avoir autant qu'on veut. Permettent d'identfier les objets